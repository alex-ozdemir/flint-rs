/*
    Copyright (C) 2010 Sebastian Pancratz
    Copyright (C) 2014 Abhinav Baid

    This file is part of FLINT.

    FLINT is free software: you can redistribute it and/or modify it under
    the terms of the GNU Lesser General Public License (LGPL) as published
    by the Free Software Foundation; either version 3 of the License, or
    (at your option) any later version.  See <https://www.gnu.org/licenses/>.
*/

#include "d_vec.h"

int
_d_vec_is_zero(const double *vec, slong len)
{
    slong i;
    for (i = 0; i < len; i++)
        if (vec[i] != 0)
            return 0;
    return 1;
}
