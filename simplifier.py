#!/usr/bin/env python3

import expr
from wrapper import *
import sys

src = sys.argv[1]
e = expr.parser.parse(src)
print('EXPR: %s' % str(e))
vrs = expr.getAllVars(e)
envs = expr.makeTbl(e,vrs)
vrs,unused = expr.splitByUsing(e,envs,vrs)
print('UNUSED:', unused)
envs = expr.makeTbl(e,vrs)

#wrap = CPPWrap('./combinator.so')
wrap = RustWrap('./libsolver.so')
analog = wrap.findAnalog([t['!'] for t in envs], len(vrs), 1000, int(len(vrs) * 2.5))
if analog is None:
    print('not found')
else:
    analog = expr.parser.parse(analog)
    analog.replace(dict(zip(expr.getAllVars(analog), vrs)))
    print(analog)
    ta = list([(1 if t['!'] else 0) for t in envs])
    print(ta)
    tb = list([(1 if t['!'] else 0) for t in expr.makeTbl(analog, vrs)])
    print(tb)
