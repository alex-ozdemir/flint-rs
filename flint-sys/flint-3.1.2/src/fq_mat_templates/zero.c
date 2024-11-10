/*
    Copyright (C) 2011 Fredrik Johansson
    Copyright (C) 2013 Mike Hansen

    This file is part of FLINT.

    FLINT is free software: you can redistribute it and/or modify it under
    the terms of the GNU Lesser General Public License (LGPL) as published
    by the Free Software Foundation; either version 3 of the License, or
    (at your option) any later version.  See <https://www.gnu.org/licenses/>.
*/

#ifdef T

#include "templates.h"

void TEMPLATE(T, mat_zero) (TEMPLATE(T, mat_t) A, const TEMPLATE(T, ctx_t) ctx)
{
    slong i, j;

    for (i = 0; i < A->r; i++)
        for (j = 0; j < A->c; j++)
            TEMPLATE(T, zero) (TEMPLATE(T, mat_entry) (A, i, j), ctx);
}


#endif
