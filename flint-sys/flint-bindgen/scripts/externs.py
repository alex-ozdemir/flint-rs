import sys
from os import listdir
from os.path import isfile, join

def combine_externs(extern_path):
    includes = ""
    out = ""
    for f in listdir(extern_path):
        fp = join(extern_path, f)
        if isfile(fp):
            includes += f"#include \"{f[:-2]}.h\"\n"
            with open(fp, 'r') as file:
                for line in file.readlines()[3:]:
                    out += line

    print(includes)
    print(out)
    

if __name__ == '__main__':
    if len(sys.argv) == 1:
        print("Provide path to extern files")
        exit()

    extern_path = sys.argv[1]
    combine_externs(extern_path)
