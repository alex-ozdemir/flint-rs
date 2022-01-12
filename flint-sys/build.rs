
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::self,
    io::{Result as IoResult},
    path::{Path, PathBuf},
    process::Command,
    str,
};

const FLINT_DIR: &str = "flint-c";

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Mingw,
    Msvc,
    Other,
}

struct Environment {
    target: Target,
    src_dir: PathBuf,
    out_dir: PathBuf,
    lib_dir: PathBuf,
    include_dir: PathBuf,
    build_dir: PathBuf,
    cache_dir: Option<PathBuf>,
    version_prefix: String,
    version_patch: Option<u64>,
}

fn main() {
    let cc = env::var_os("CC");
    let cc_cache_dir = cc.as_ref().map(|cc| {
        let mut dir = OsString::from("CC-");
        dir.push(cc);
        dir
    });

    let raw_target = cargo_env("TARGET")
        .into_string()
        .expect("env var TARGET having sensible characters");

    let target = if raw_target.contains("-windows-msvc") {
        Target::Msvc
    } else if raw_target.contains("-windows-gnu") {
        Target::Mingw
    } else {
        Target::Other
    };

    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));

    let (version_prefix, version_patch) = get_version();

    println!("cargo:rerun-if-env-changed=FLINT_SYS_CACHE");
    let cache_dir = match env::var_os("FLINT_SYS_CACHE") {
        Some(ref c) if c.is_empty() || c == "_" => None,
        Some(c) => Some(PathBuf::from(c)),
        None => system_cache_dir().map(|c| c.join("antic-sys")),
    };
    let cache_dir = cache_dir
        .map(|cache| cache.join(&version_prefix))
        .map(|cache| match cc_cache_dir {
            Some(dir) => cache.join(dir),
            None => cache,
        });

    let env = Environment {
        target,
        src_dir,
        out_dir: out_dir.clone(),
        lib_dir: out_dir.join("lib"),
        include_dir: out_dir.join("include"),
        build_dir: out_dir.join("build"),
        cache_dir,
        version_prefix,
        version_patch,
    };
       
    // make sure we have target directories
    create_dir_or_panic(&env.lib_dir);
    create_dir_or_panic(&env.include_dir);

    compile(&env);
}

fn compile(env: &Environment) {
    let flint_ah = (
        env.lib_dir.join("libflint.a"), 
        env.include_dir.join("wrapper.h"), 
        );
    if need_compile(env, &flint_ah) {
        check_for_msvc(env);
        remove_dir_or_panic(&env.build_dir);
        create_dir_or_panic(&env.build_dir);
        link_dir(&env.src_dir.join(FLINT_DIR), &env.build_dir.join("flint-src"));
        let (ref a, ref h) = flint_ah;
        build(env, a, h);
        if !there_is_env("CARGO_FEATURE_CNODELETE") {
            remove_dir_or_panic(&env.build_dir);
        }
        assert!(save_cache(env, &flint_ah));
    }
    write_link_info(env);
}


fn get_version() -> (String, Option<u64>) {
    let version = cargo_env("CARGO_PKG_VERSION")
        .into_string()
        .unwrap_or_else(|e| panic!("version not in utf-8: {:?}", e));
    let last_dot = version
        .rfind('.')
        .unwrap_or_else(|| panic!("version has no dots: {}", version));
    if last_dot == 0 {
        panic!("version starts with dot: {}", version);
    }
    match version[last_dot + 1..].parse::<u64>() {
        Ok(patch) => {
            let mut v = version;
            v.truncate(last_dot);
            (v, Some(patch))
        }
        Err(_) => (version, None),
    }
}

fn need_compile(
    env: &Environment,
    flint_ah: &(PathBuf, PathBuf),
) -> bool {
    let flint_fine = flint_ah.0.is_file() 
        && flint_ah.1.is_file();
    if flint_fine {
        if should_save_cache(env) {
            assert!(save_cache(env, flint_ah));
        }
        return false;
    } else if load_cache(env, flint_ah) {
        // if loading cache works, we're done
        return false;
    }
    true
}

fn save_cache(
    env: &Environment,
    flint_ah: &(PathBuf, PathBuf),
) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let version_dir = match env.version_patch {
        None => cache_dir.join(&env.version_prefix),
        Some(patch) => cache_dir.join(format!("{}.{}", env.version_prefix, patch)),
    };
    let mut ok = create_dir(&version_dir).is_ok();
    let dir = version_dir;
    let (ref a, ref h) = *flint_ah;
    ok = ok && copy_file(a, &dir.join("libflint.a")).is_ok();
    ok = ok && copy_file(h, &dir.join("wrapper.h")).is_ok();
    ok
}

fn cache_directories(env: &Environment, base: &Path) -> Vec<(PathBuf, Option<u64>)> {
    let dir = match fs::read_dir(base) {
        Ok(dir) => dir,
        Err(_) => return Vec::new(),
    };
    let mut vec = Vec::new();
    for entry in dir {
        let path = match entry {
            Ok(e) => e.path(),
            Err(_) => continue,
        };
        if !path.is_dir() {
            continue;
        }
        let patch = {
            let file_name = match path.file_name() {
                Some(name) => name,
                None => continue,
            };
            let path_str = match file_name.to_str() {
                Some(p) => p,
                None => continue,
            };
            if path_str == env.version_prefix {
                None
            } else if !path_str.starts_with(&env.version_prefix)
                || !path_str[env.version_prefix.len()..].starts_with('.')
            {
                continue;
            } else {
                match path_str[env.version_prefix.len() + 1..].parse::<u64>() {
                    Ok(patch) => Some(patch),
                    Err(_) => continue,
                }
            }
        };
        vec.push((path, patch));
    }
    vec.sort_by_key(|k| k.1);
    vec
}

fn load_cache(
    env: &Environment,
    flint_ah: &(PathBuf, PathBuf),
) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let env_version_patch = env.version_patch;
    let cache_dirs = cache_directories(env, cache_dir)
        .into_iter()
        .rev()
        .filter(|x| match env_version_patch {
            None => x.1.is_none(),
            Some(patch) => x.1.map(|p| p >= patch).unwrap_or(false),
        })
        .collect::<Vec<_>>();
    let suffixes: &[Option<&str>] = &[None];
    for suffix in suffixes {
        for (version_dir, _) in &cache_dirs {
            let joined;
            let dir = if let Some(suffix) = suffix {
                joined = version_dir.join(suffix);
                &joined
            } else {
                version_dir
            };
            let mut ok = true;
            let (ref a, ref h) = *flint_ah;
            ok = ok && copy_file(&dir.join("libflint.a"), a).is_ok();
            let header = dir.join("wrapper.h");
            ok = ok && copy_file(&header, h).is_ok();
            if ok {
                return true;
            }
        }
    }
    false
}

fn should_save_cache(env: &Environment) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let cache_dirs = cache_directories(env, cache_dir)
        .into_iter()
        .rev()
        .filter(|x| match env.version_patch {
            None => x.1.is_none(),
            Some(patch) => x.1.map(|p| p >= patch).unwrap_or(false),
        })
        .collect::<Vec<_>>();
    let suffixes: &[Option<&str>] = &[None];
    for suffix in suffixes {
        for (version_dir, _) in &cache_dirs {
            let joined;
            let dir = if let Some(suffix) = suffix {
                joined = version_dir.join(suffix);
                &joined
            } else {
                version_dir
            };
            let mut ok = true;
            ok = ok && dir.join("libflint.a").is_file();
            ok = ok && dir.join("wrapper.h").is_file();
            if ok {
                return false;
            }
        }
    }
    true
}

fn build(env: &Environment, lib: &Path, h: &Path) {
    let build_dir = env.build_dir.join("build");
    create_dir_or_panic(&build_dir);
    println!("$ cd {:?}", build_dir);
    let conf = String::from("../flint-src/configure --disable-shared");
    configure(&build_dir, &OsString::from(conf));
    make_and_check(env, &build_dir);

    let build_lib = build_dir.join("libflint.a");
    copy_file_or_panic(&build_lib, lib);
    let build_h = build_dir.join("wrapper.h");
    copy_file_or_panic(&build_h, h);
}

fn write_link_info(env: &Environment) {
    let out_str = env.out_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.out_dir.display()
        )
    });
    let lib_str = env.lib_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.lib_dir.display()
        )
    });
    let include_str = env.include_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.include_dir.display()
        )
    });

    println!("cargo:out_dir={}", out_str);
    println!("cargo:lib_dir={}", lib_str);
    println!("cargo:include_dir={}", include_str);
    println!("cargo:rustc-link-search=native={}", lib_str);
    println!("cargo:rustc-link-lib=static=mpfr");
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=flint");
}


fn cargo_env(name: &str) -> OsString {
    env::var_os(name)
        .unwrap_or_else(|| panic!("environment variable not found: {}, please use cargo", name))
}

fn there_is_env(name: &str) -> bool {
    env::var_os(name).is_some()
}

fn check_for_msvc(env: &Environment) {
    if env.target == Target::Msvc {
        panic!("Windows MSVC target is not supported (linking would fail)");
    }
}

fn remove_dir(dir: &Path) -> IoResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    assert!(dir.is_dir(), "Not a directory: {:?}", dir);
    println!("$ rm -r {:?}", dir);
    fs::remove_dir_all(dir)
}

fn remove_dir_or_panic(dir: &Path) {
    remove_dir(dir).unwrap_or_else(|_| panic!("Unable to remove directory: {:?}", dir));
}

fn create_dir(dir: &Path) -> IoResult<()> {
    println!("$ mkdir -p {:?}", dir);
    fs::create_dir_all(dir)
}

fn create_dir_or_panic(dir: &Path) {
    create_dir(dir).unwrap_or_else(|_| panic!("Unable to create directory: {:?}", dir));
}

fn copy_file(src: &Path, dst: &Path) -> IoResult<u64> {
    println!("$ cp {:?} {:?}", src, dst);
    fs::copy(src, dst)
}

fn copy_file_or_panic(src: &Path, dst: &Path) {
    copy_file(src, dst).unwrap_or_else(|_| {
        panic!("Unable to copy {:?} -> {:?}", src, dst);
    });
}

fn configure(build_dir: &Path, conf_line: &OsStr) {
    let mut conf = Command::new("sh");
    conf.current_dir(&build_dir).arg("-c").arg(conf_line);
    execute(conf);
}

fn make_and_check(_env: &Environment, build_dir: &Path) {
    let mut make = Command::new("make");
    make.current_dir(build_dir);
    execute(make);
    
    let mut make_check = Command::new("make");
    make_check
        .current_dir(build_dir)
        .arg("check");
    execute(make_check);
}

#[cfg(unix)]
fn link_dir(src: &Path, dst: &Path) {
    println!("$ ln -s {:?} {:?}", src, dst);
    unix_fs::symlink(src, dst).unwrap_or_else(|_| {
        panic!("Unable to symlink {:?} -> {:?}", src, dst);
    });
}

#[cfg(windows)]
fn link_dir(src: &Path, dst: &Path) {
    println!("$ ln -s {:?} {:?}", src, dst);
    if windows_fs::symlink_dir(src, dst).is_ok() {
        return;
    }
    println!("symlink_dir: failed to create symbolic link, copying instead");
    let mut c = Command::new("cp");
    c.arg("-R").arg(src).arg(dst);
    execute(c);
}

fn execute(mut command: Command) {
    println!("$ {:?}", command);
    let status = command
        .status()
        .unwrap_or_else(|_| panic!("Unable to execute: {:?}", command));
    if !status.success() {
        if let Some(code) = status.code() {
            panic!("Program failed with code {}: {:?}", code, command);
        } else {
            panic!("Program failed: {:?}", command);
        }
    }
}

fn system_cache_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        use core::{mem::MaybeUninit, ptr, slice};
        use std::os::windows::ffi::OsStringExt;
        use winapi::{
            shared::winerror::S_OK,
            um::{combaseapi, knownfolders::FOLDERID_LocalAppData, shlobj, winbase},
        };
        let id = &FOLDERID_LocalAppData;
        let flags = 0;
        let access = ptr::null_mut();
        let mut path = MaybeUninit::uninit();
        unsafe {
            if shlobj::SHGetKnownFolderPath(id, flags, access, path.as_mut_ptr()) == S_OK {
                let path = path.assume_init();
                let slice = slice::from_raw_parts(path, winbase::lstrlenW(path) as usize);
                let string = OsString::from_wide(slice);
                combaseapi::CoTaskMemFree(path as _);
                Some(string.into())
            } else {
                None
            }
        }
    }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        env::var_os("HOME")
            .filter(|x| !x.is_empty())
            .map(|x| AsRef::<Path>::as_ref(&x).join("Library").join("Caches"))
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios")))]
    {
        env::var_os("XDG_CACHE_HOME")
            .filter(|x| !x.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME")
                    .filter(|x| !x.is_empty())
                    .map(|x| AsRef::<Path>::as_ref(&x).join(".cache"))
            })
    }
}

