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
from pyparsing import Word, Literal, Forward, ZeroOrMore, Group

class Var:
    def __init__(self, name):
        self.name = name
    def __repr__(self):
        return '$' + self.name

class Not:
    def __init__(self,expr):
        self.expr = expr
    def __repr__(self):
        return '~%s' % repr(self.expr)

class Bin:
    def __init__(self,f,a,b):
        self.fun = f
        self.a = a
        self.b = b
    def __repr__(self):
        return '(%s %s %s)' % (repr(self.a), self.fun, repr(self.b))

class Cond:
    def __init__(self,c,a,b):
        self.cond = c
        self.yes = a
        self.no = b
    def __repr__(self):
        return 'if {%s} {%s} {%s}' % (repr(self.cond), repr(self.yes), repr(self.no))

class Parser:
    def pushVar(self,a):
        self.vstack.append(Var(a[0]))
        self.binpop()
    def pushVal(self,a):
        self.vstack.append(a[0] == 'true')
        self.binpop()
    def pushNot(self,a):
        #print('n')
        self.vstack[-1] = Not(self.vstack[-1])
        self.binpop()
    def pushCond(self,_):
        e = self.vstack.pop()
        t = self.vstack.pop()
        c = self.vstack.pop()
        self.vstack.append(Cond(c,t,e))
    def pushBin(self,a):
        self.binval = a[0]
    def blockIn(self,_):
        #print('b-in')
        self.bstack.append((self.vstack,self.binval))
        self.vstack = []
    def blockOut(self,_):
        #print('b-out')
        (vstack,binv) = self.bstack.pop()
        vstack.append(self.vstack[0])
        self.vstack = vstack
        self.binval = binv
        self.binpop()
    def binpop(self):
        if self.binval is None:
            pass
        else:
            r = self.vstack.pop()
            l = self.vstack.pop()
            self.vstack.append(Bin(self.binval, l, r))
            self.binval = None
    def __init__(self):
        self.vstack = []
        self.bstack = []
        self.binval = None
        var = Word('ABCDEFGHIJKLMNOPQRSTUVWXYZ').setParseAction(self.pushVar)
        val = (Word('false') | Word('true')).setParseAction(self.pushVal)
        _lp = Literal('(').suppress().setParseAction(self.blockIn)
        _rp = Literal(')').suppress().setParseAction(self.blockOut)
        _lr = Literal('{').suppress()
        _rr = Literal('}').suppress()
        expr = Forward()
        func = (Literal('&&') | Literal('||') | Literal('==') | Literal('!=')).setParseAction(self.pushBin)
        _if = Literal('if').suppress()
        cond = Group(_if + _lr + expr + _rr + _lr + expr + _rr + _lr + expr + _rr).setParseAction(self.pushCond)
        nsym = Literal('~')
        fnot = (Group(nsym + var) | Group(nsym + val) | Group(nsym + _lp + expr + _rp)).setParseAction(self.pushNot)
        atom = (var | val | fnot | Group(_lp + expr + _rp) | cond)
        expr <<= (atom + ZeroOrMore(func + expr))
        self._expr = expr
    def parse(self,text):
        self.vstack = []
        self.bstack = []
        self.binval = None
        self._expr.parseString(text)
        return self.vstack[0]

parser = Parser()
