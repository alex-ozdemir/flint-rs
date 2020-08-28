#!/usr/bin/env python

from bs4 import BeautifulSoup
import requests
import sys
from pprint import pprint

from typing import NamedTuple, List, Dict, Set, Optional

flint_url = "http://flintlib.org/doc/{}.html"
flint_files = ["fmpz", "fmpz_mod_poly"]


def main():
    ty_map = {
        "ulong": "ulong",
        "slong": "slong",
        "slong *": "*const isize",
        "fmpz_t": "*const fmpz",
        "const fmpz_t": "*const fmpz",
        "flint_rand_t": "*mut flint_rand",
        "flint_bitcnt_t": "usize",
        "mp_limb_t": "mp_limb_t",
        "mp_limb_signed_t": "mp_limb_signed_t",
        "int": "c_int",
        "double": "c_double",
        "size_t": "usize",
        "const mpz_t": "*const gmp::mpz_t",
        "const mpf_t": "*const gmp::mpf_t",
        "const mpfr_t": "*const mpfr::mpfr_t",
        "const mpq_t": "*const gmp::mpq_t",
        "mpz_t": "*mut gmp::mpz_t",
        "mpf_t": "*mut gmp::mpf_t",
        "mpfr_t": "*mut mpfr::mpfr_t",
        "mpq_t": "*mut gmp::mpq_t",
        "mpfr_rnd_t": "mpfr::rnd_t",
        "mp_ptr *": "*mut *const mp_limb_t",
        "mp_limb_t *": "*mut mp_limb_t",
        "const mp_limb_t *": "*const mp_limb_t",
        "mp_srcptr": "*const mp_limb_t",
        "mp_ptr": "*mut mp_limb_t",
        "const ulong *": "*const ulong",
        "ulong *": "*mut ulong",
        "char *": "*mut c_char",
        "const char *": "*const c_char",
        "FILE *": "*mut FILE",
        "mp_size_t": "c_long",
        "const fmpz *": "*const fmpz",
        "fmpz *": "*mut fmpz",
        "fmpz_mod_poly_t": "*const fmpz_mod_poly",
        "const fmpz_mod_poly_t": "*const fmpz_mod_poly",
        "fmpz **": "*mut *mut fmpz",
        "fmpz_mod_poly **": "*mut *mut fmpz_mod_poly",
        "const fmpz_mod_poly_struct *": "*const fmpz_mod_poly",
        "fmpz_mod_poly_struct * const *": "*const *mut fmpz_mod_poly",
        "fmpz_mod_poly_struct **": "*mut *mut fmpz_mod_poly",
        "fmpz_mod_poly_struct *": "*mut fmpz_mod_poly",
        "fmpz_poly_t": "*mut fmpz_poly",
        "fmpz_poly_struct **": "*mut *mut fmpz_poly",
        "fmpz_poly_struct * const *": "*const *const fmpz_poly",
        "const fmpz_poly_t": "*const fmpz_poly",
    }
    # Functions to omit
    skipfn = {
        "PTR_TO_COEFF",
        "COEFF_TO_PTR",
        "COEFF_IS_MPZ",
        "_fmpz_new_mpz",
        "_fmpz_clear_mpz",
        "_fmpz_cleanup_mpz_content",
        "_fmpz_cleanup",
        "_fmpz_promote",
        "_fmpz_promote_val",
        "_fmpz_demote",
        "_fmpz_demote_val",
        "fmpz_preinvn_init",
        "fmpz_fdiv_qr_preinvn",
        "fmpz_preinvn_clear",
        "_nmod_poly_crt_local_size",
    }
    # Skip any function over these types
    skipty = {
        "fmpz_comb_t",
        "fmpz_comb_temp_t",
        "const fmpz_multi_crt_t",
        "fmpz_multi_crt_t",
        "const fmpz_factor_t",
        "thread_pool_handle *",
        "fmpz_mod_poly_frobenius_powers_2exp_t",
        "fmpz_mod_poly_frobenius_powers_t",
        "fmpz_mat_t",
        "void *",
        "fmpz_mod_poly_radix_t",
        "fmpz_mod_berlekamp_massey_t",
    }
    for t in list(skipty):
        skipty.add("const " + t)
    # Modification to the symbol names that a function should link against.
    linknames = {
        "free_fmpz": "__free_fmpz",
        "fmpz_set_si": "__fmpz_set_si",
        "fmpz_set_ui": "__fmpz_set_ui",
        "fmpz_init": "__fmpz_init",
        "fmpz_init_set_ui": "__fmpz_init_set_ui",
        "fmpz_clear": "__fmpz_clear",
        "fmpz_lt": "__fmpz_lt",
        "fmpz_gt": "__fmpz_gt",
        "fmpz_lte": "__fmpz_lte",
        "fmpz_gte": "__fmpz_gte",
        "fmpz_eq": "__fmpz_eq",
        "fmpz_neq": "__fmpz_neq",
        "fmpz_init_set": "__fmpz_init_set",
        "fmpz_neg": "__fmpz_neg",
    }
    cfns = []
    for file in flint_files:
        url = flint_url.format(file)
        r = requests.get(url)
        soup = BeautifulSoup(r.text, features="lxml")
        encoding = soup.meta["charset"]
        r.encoding = encoding
        soup = BeautifulSoup(r.text, features="lxml")
        fns = []
        for d in soup("dl"):
            if "function" in d["class"]:
                fns.extend(d("dt"))
        tys = set()
        for f in fns:
            tokens = []
            for c in f.children:
                s = c.string
                if s is not None:
                    s = s.strip()
                    if len(s) > 0 and s.isascii():
                        tokens.append(s)
            cfns.append(CFn.from_tokens(tokens, file))
            c = cfns[-1]
            tys.add(c.ret_ty)
            for t in c.args:
                tys.add(t.ty)
    for c in cfns:
        if c.name not in skipfn:
            s = c.as_rust_fn(ty_map, skipty, linknames)
            if s is not None:
                print(s)
                print()


def rust_id(s: str) -> str:
    RUST_KEYWORDS = {
        # Real
        "as",
        "break",
        "const",
        "continue",
        "crate",
        "else",
        "enum",
        "extern",
        "false",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "match",
        "mod",
        "move",
        "mut",
        "pub",
        "ref",
        "return",
        "self",
        "Self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
        # 2018
        "async",
        "await",
        # Reserved
        "dyn",
        "abstract",
        "become",
        "box",
        "do",
        "final",
        "macro",
        "override",
        "priv",
        "typeof",
        "unsized",
        "virtual",
        "yield",
        # Reserved 2018
        "try",
    }
    if s in RUST_KEYWORDS:
        return f"{s}_"
    else:
        return s


class CDecl(NamedTuple):
    ty: str
    name: str

    @classmethod
    def from_str(cls, s) -> "CDecl":
        s = s.strip()
        i = s.rfind(" ")
        return cls(s[:i], s[i + 1 :])

    def as_rust_decl(self, i: int, ty_map: Dict[str, str]) -> str:
        t = ty_map[self.ty]
        if i == 0:
            if self.ty == "fmpz_t":
                t = "*mut fmpz"
            elif self.ty == "fmpz_t":
                t = "*mut fmpz_mod_poly"
        return f"{rust_id(self.name)}: {t}"


class CFn(NamedTuple):
    ret_ty: str
    name: str
    args: List[CDecl]
    file: str

    @classmethod
    def from_tokens(cls, ts: List[str], file: str) -> "CFn":
        lparen_i = ts.index("(")
        rparen_i = ts.index(")")
        name = ts[lparen_i - 1]
        ret_ty = " ".join(ts[: lparen_i - 1])
        return cls(
            ret_ty,
            name,
            [
                CDecl.from_str(s)
                for s in " ".join(ts[lparen_i + 1 : rparen_i]).split(",")
                if len(s.strip()) > 0
            ],
            file,
        )

    def as_rust_fn(
        self, ty_map: Dict[str, str], skip: Set[str], linknames: Dict[str, str]
    ) -> Optional[str]:
        try:

            sig = (
                f"pub fn {self.name}("
                + ", ".join(d.as_rust_decl(i, ty_map) for i, d in enumerate(self.args))
                + ")"
            )
            doc = f"/// See the [FLINT Documentation](http://flintlib.org/doc/{self.file}.html#c.{self.name}) for this function."
            if self.ret_ty == "void":
                sig += ";"
            else:
                sig += f" -> {ty_map[self.ret_ty]};"
            l = linknames[self.name] if self.name in linknames else self.name
            link = f'#[link_name = "{l}"]'
            return "\n".join([doc, link, sig])
        except KeyError as e:
            if e.args[0] in skip:
                return None
            print("\n\n\nError: ", repr(e), file=sys.stderr)
            print("in", self, file=sys.stderr)
            sys.exit(1)


main()
