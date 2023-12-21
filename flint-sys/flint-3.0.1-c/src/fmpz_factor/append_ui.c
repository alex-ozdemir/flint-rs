/*
    Copyright (C) 2008, 2009 William Hart
    Copyright (C) 2010 Fredrik Johansson

    This file is part of FLINT.

    FLINT is free software: you can redistribute it and/or modify it under
    the terms of the GNU Lesser General Public License (LGPL) as published
    by the Free Software Foundation; either version 2.1 of the License, or
    (at your option) any later version.  See <https://www.gnu.org/licenses/>.
*/

#include "fmpz.h"
#include "fmpz_factor.h"

void
_fmpz_factor_append_ui(fmpz_factor_t factor, mp_limb_t p, ulong exp)
{
    _fmpz_factor_fit_length(factor, factor->num + 1);
    fmpz_set_ui(factor->p + factor->num, p);
    factor->exp[factor->num] = exp;
    factor->num++;
}