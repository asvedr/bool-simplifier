#
# simpleBool.py
#
# Example of defining a boolean logic parser using
# the operatorGrammar helper method in pyparsing.
#
# In this example, parse actions associated with each
# operator expression will "compile" the expression
# into BoolXXX class instances, which can then
# later be evaluated for their boolean value.
#
# Copyright 2006, by Paul McGuire
# Updated 2013-Sep-14 - improved Python 2/3 cross-compatibility
#
from pyparsing import infixNotation, opAssoc, Keyword, Word, alphas

# define classes to be built at parse time, as each matching
# expression type is parsed
class BoolOperand(object):
    def __init__(self,t):
        self.label = t
        self.value = eval(t[0])
    def __str__(self):
        return self.label
    def __repr__(self):
        return self.__str__()

class BoolBinOp(object):
    def __init__(self,t):
        self.args = t[0][0::2]
    def __str__(self):
        #sep = " %s " % self.reprsymbol
        #return "(" + sep.join(map(str,self.args)) + ")"
        return '(%s %s %s)' % (self.symbol, *self.args)
    def __repr__(self):
        return self.__str__()

class Cond(object):
    def __init__(self,t):
        self.args = t[0][0::2]
    def __str__(self):
        return 'if(%s) {%s} {%s}' % self.args
    def __repr__(self):
        return self.__str__()

class BoolAnd(BoolBinOp):
    symbol = '&&'

class BoolOr(BoolBinOp):
    symbol = '||'

class BoolEq(BoolBinOp):
    symbol = '=='

class BoolXor(BoolBinOp):
    symbol = '!='

class BoolNot(object):
    def __init__(self,t):
        self.arg = t[0][1]
    def __bool__(self):
        v = bool(self.arg)
        return not v
    def __str__(self):
        return "!" + str(self.arg)
    def __repr__(self):
        return self.__str__()

TRUE = Keyword("True")
FALSE = Keyword("False")
boolOperand = TRUE | FALSE | Word(alphas,max=1)
boolOperand.setParseAction(BoolOperand)

# define expression, based on expression operand and
# list of operations in precedence order
boolExpr = infixNotation( boolOperand,
    [
        ("!",  1, opAssoc.RIGHT, BoolNot),
        ("&&", 2, opAssoc.LEFT,  BoolAnd),
        ("||", 2, opAssoc.LEFT,  BoolOr),
        ("==", 2, opAssoc.LEFT,  BoolEq),
        ("!=", 2, opAssoc.LEFT,  BoolXor)
    ])


if __name__ == "__main__":
    p = True
    q = False
    r = True
    tests = ["p",
             "q",
             "p && q",
             "p && !q", 
             "!!p", 
             "!(p && q)", 
             "q || !p &&r", 
             "q || !p || !r", 
             "q || !(p && r)", 
             "p || q || r", 
             "p || q || r && False", 
             "(p || q || r) && False", 
            ]

    print("p =", p)
    print("q =", q)
    print("r =", r)
    for t in tests:
        res = boolExpr.parseString(t)
        print(str(res))
        #success = "PASS" if bool(res) == expected else "FAIL"
        #print (t,'\n', res, '=', bool(res),'\n', success, '\n')

