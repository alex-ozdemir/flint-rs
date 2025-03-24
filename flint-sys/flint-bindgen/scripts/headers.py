import sys
from os import listdir
from os.path import isfile, join

skip = [
    "NTL-interface.h",
    "crt_helpers.h",
    "fft_small.h",
    "longlong_asm_clang.h",
    "longlong_asm_gcc.h",
    "longlong_div_gnu.h",
    "longlong_msc_arm64.h",
    "longlong_msc_x86.h",
    "machine_vectors.h",
    "profiler.h"
]

def gen_mod_statements(header_fns):    
    out = ""
    for fn in header_fns:  
        out += f"pub mod {fn[:-2]};\n"
    print(out)

def gen_header_list(header_fns):    
    out = "const FLINT_HEADERS: &[&str] = &[\n"
    for fn in header_fns:        
        out += f"    \"{fn}\",\n"
    out += "];"
    print(out)

if __name__ == '__main__':
    if len(sys.argv) == 1:
        print("Provide path to header files")
        exit()

    header_path = sys.argv[1]
    header_fns = []

    for f in listdir(header_path):
        if isfile(join(header_path, f)):
            if not f in skip and f[-2:] == '.h':
                header_fns.append(f)

    header_fns.sort()
    gen_header_list(header_fns)
    gen_mod_statements(header_fns)    
