import sys
from os import listdir
from os.path import isfile, join

# Headers to exclude from BOTH copying and binding generation
skip = [
    "NTL-interface.h",
    "crt_helpers.h",
    "fft_small.h",
    "machine_vectors.h",
    "profiler.h",
    "test_helpers.h",
]

# Headers to exclude ONLY from the bindgen list
bindgen_skip = [
    "longlong_msc_arm64.h",
    "longlong_msc_x86.h",
    "longlong_asm_clang.h",
    "longlong_asm_gcc.h",
    "longlong_asm_gnu.h",
    "longlong_div_gnu.h",
    "fq_zech_mpoly.h",
    "fq_zech_mpoly_factor.h",
    "flint-mparam.h",
    "flint-config.h",
    "gettimeofday.h",
    "config.h",
]

def gen_mod_statements(header_fns):
    out = ""
    for fn in header_fns:
        out += f"pub mod {fn[:-2]};\n"
    return out

def gen_header_list(header_fns, var_name):
    out = f"const {var_name}: &[&str] = &[\n"
    for fn in header_fns:
        out += f"    \"{fn}\",\n"
    out += "];"
    return out

def write_to_file(filename, content):
    with open(filename, 'w') as f:
        f.write(content)

if __name__ == '__main__':
    if len(sys.argv) == 1:
        sys.stderr.write("Usage: python script.py <path_to_headers>\n")
        sys.exit(1)

    header_path = sys.argv[1]
    
    all_files = [f for f in listdir(header_path) if isfile(join(header_path, f)) and f.endswith('.h')]

    # 1. Build headers: exclude global skip list
    build_headers = [f for f in all_files if f not in skip]
    build_headers.sort()

    # 2. Bindgen headers: exclude global skip AND bindgen_skip
    bindgen_headers = [f for f in build_headers if f not in bindgen_skip]
    bindgen_headers.sort()

    # Generate contents
    build_headers_content = gen_header_list(build_headers, "FLINT_HEADERS")
    bindgen_headers_content = gen_header_list(bindgen_headers, "FLINT_HEADERS")
    mod_statements_content = gen_mod_statements(bindgen_headers)

    # Write to files
    write_to_file("build_headers.rs", build_headers_content)
    write_to_file("bindgen_headers.rs", bindgen_headers_content)
    write_to_file("__mod_statements.rs", mod_statements_content)

    sys.stderr.write("Files generated: build_headers.rs, bindgen_headers.rs, mod_statements.rs\n")