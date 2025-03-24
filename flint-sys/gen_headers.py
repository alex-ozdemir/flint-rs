# output FLINT_HEADERS to update build.rs to a new version of flint

import sys
from os import listdir
from os.path import isfile, join

if __name__ == '__main__':
    if len(sys.argv) == 1:
        print("Provide path to header files")
        exit()

    header_path = sys.argv[1]
    header_fns = []

    skip = [
        "NTL-interface.h",
        "crt_helpers.h",
        "fft_small.h",
        "longlong_asm_clang.h",
        "longlong_asm_gcc.h",
        "longlong_div_gnu.h",
        "longlong_msc_arm64.h",
        "longlong_msc_x86.h",
        "machine_vectors.h"
    ]


    for f in listdir(header_path):
        if isfile(join(header_path, f)):
            if f not in skip:
                header_fns.append(f)

    header_fns.sort()
    out = "const FLINT_HEADERS: &[&str] = &[\n"
    for fn in header_fns:        
        out += f"    \"{fn}\",\n"
    out += "];"
    print(out)
    
