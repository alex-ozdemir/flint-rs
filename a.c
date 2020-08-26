#include "flint/fmpz.h"
#include "flint/fmpz_mod_poly.h"

int main() {
    fmpz_t p;
    fmpz_init(p);
    fmpz_set_ui(p, 17);
    fmpz_mod_poly_t f, g, h;
    fmpz_mod_poly_init(f, p);
    fmpz_mod_poly_set_coeff_si(f, 0, -3);
    fmpz_mod_poly_set_coeff_si(f, 1, 1);
    fmpz_mod_poly_init(g, p);
    fmpz_mod_poly_set_coeff_si(g, 0, -3);
    fmpz_mod_poly_set_coeff_si(g, 1, 1);
    fmpz_mod_poly_init(h, p);
    fmpz_mod_poly_mul(h, f, g);
    fmpz_mod_poly_print_pretty(h, "x");
}
