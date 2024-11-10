/*
    Copyright (C) 2012 Andres Goens
    Copyright (C) 2012 Sebastian Pancratz
    Copyright (C) 2013 Mike Hansen

    This file is part of FLINT.

    FLINT is free software: you can redistribute it and/or modify it under
    the terms of the GNU Lesser General Public License (LGPL) as published
    by the Free Software Foundation; either version 3 of the License, or
    (at your option) any later version.  See <https://www.gnu.org/licenses/>.
*/

#ifdef T

#include "templates.h"

void
_TEMPLATE(T, poly_zero)(TEMPLATE(T, struct) *rop, slong len,
                        const TEMPLATE(T, ctx_t) ctx)
{
    slong i;

    for (i = 0; i < len; i++)
        TEMPLATE(T, zero)(rop + i, ctx);
}

void
TEMPLATE(T, poly_zero)(TEMPLATE(T, poly_t) poly, const TEMPLATE(T, ctx_t) ctx)
{
    _TEMPLATE(T, poly_set_length)(poly, 0, ctx);
}

#endif
