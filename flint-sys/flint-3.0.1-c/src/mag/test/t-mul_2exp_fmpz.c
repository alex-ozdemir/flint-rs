/*
    Copyright (C) 2014 Fredrik Johansson

    This file is part of Arb.

    Arb is free software: you can redistribute it and/or modify it under
    the terms of the GNU Lesser General Public License (LGPL) as published
    by the Free Software Foundation; either version 2.1 of the License, or
    (at your option) any later version.  See <http://www.gnu.org/licenses/>.
*/

#include "arf.h"
#include "mag.h"

int main(void)
{
    slong iter;
    flint_rand_t state;

    flint_printf("mul_2exp_fmpz....");
    fflush(stdout);

    flint_randinit(state);

    for (iter = 0; iter < 100000 * 0.1 * flint_test_multiplier(); iter++)
    {
        arf_t x, y, z;
        mag_t xb, yb;
        fmpz_t e;

        arf_init(x);
        arf_init(y);
        arf_init(z);

        fmpz_init(e);

        mag_init(xb);
        mag_init(yb);

        mag_randtest_special(xb, state, 100);
        fmpz_randtest(e, state, 100);
        arf_set_mag(x, xb);

        mag_mul_2exp_fmpz(yb, xb, e);

        arf_mul_2exp_fmpz(y, x, e);

        arf_set_mag(z, yb);

        MAG_CHECK_BITS(yb)

        if (!arf_equal(z, y))
        {
            flint_printf("FAIL\n\n");
            flint_printf("x = "); arf_printd(x, 15); flint_printf("\n\n");
            flint_printf("y = "); arf_printd(y, 15); flint_printf("\n\n");
            flint_printf("z = "); arf_printd(z, 15); flint_printf("\n\n");
            flint_abort();
        }

        arf_clear(x);
        arf_clear(y);
        arf_clear(z);

        mag_clear(xb);
        mag_clear(yb);

        fmpz_clear(e);
    }

    flint_randclear(state);
    flint_cleanup();
    flint_printf("PASS\n");
    return 0;
}
