from pyparsing import Word, Literal, Forward, ZeroOrMore, Group, Regex
from functools import reduce

# Expr items: Var, Not, Bin, Cond, bool
# parse from str: parser.parse(str) -> Expr
# get vars: getAllVar(expr) -> [var]
# making true-flase table: makeTbl(expr,vars) -> {vname:val, ..., '!':expr-val}
# get used and unused: splitByUsing(expr,envs,vars) -> (used,unused)

class Var:
    def __init__(self, name):
        self.name = name
    def __repr__(self):
        return '$' + self.name
    def __str__(self):
        return self.name
    def eval(self,env):
        try:
            return env[self.name]
        except:
            return True
    def replace(self,pairs):
        if self.name in pairs:
            self.name = pairs[self.name]

class Not:
    def __init__(self,expr):
        self.expr = expr
    def __repr__(self):
        return '~%s' % repr(self.expr)
    def __str__(self):
        return '~' + str(self.expr)
    def eval(self,env):
        return not self.expr.eval(env)
    def replace(self,pairs):
        self.expr.replace(pairs)

class Bin:
    def __init__(self,f,call,a,b):
        self.fun = f
        self.a = a
        self.b = b
        self.call = call
    def __repr__(self):
        return '(%s %s %s)' % (repr(self.a), self.fun, repr(self.b))
    def __str__(self):
        return '(%s %s %s)' % (self.a, self.fun, self.b)
    def eval(self,env):
        a = self.a.eval(env)
        b = self.b.eval(env)
        return self.call(a,b)
    def replace(self,pairs):
        self.a.replace(pairs)
        self.b.replace(pairs)

class Cond:
    def __init__(self,c,a,b):
        self.cond = c
        self.yes = a
        self.no = b
    def __repr__(self):
        return 'if {%s} {%s} {%s}' % (repr(self.cond), repr(self.yes), repr(self.no))
    def __str__(self):
        return 'if {%s} {%s} {%s}' % (str(self.cond), str(self.yes), str(self.no))
    def eval(self,env):
        if self.cond.eval(env):
            return self.yes.eval(env)
        else:
            return self.no.eval(env)
    def replace(self,pairs):
        self.cond.replace(pairs)
        self.yes.replace(pairs)
        self.no.replace(pairs)

class Parser:
    funs = {
            '||': bool.__or__,
            '&&': bool.__and__,
            '==': bool.__eq__,
            '!=': bool.__xor__
        }
    def pushVar(self,a):
        self.vstack.append(Var(a[0]))
        self.binpop()
    def pushVal(self,a):
        self.vstack.append(a[0] == 'true')
        self.binpop()
    def dumpNot(self,_):
        self.bstack.append(self.binval)
        self.binval = None
    def pushNot(self,_):
        self.vstack[-1] = Not(self.vstack[-1])
        self.binval = self.bstack.pop()
        self.binpop()
    def pushCond(self,_):
        e = self.vstack.pop()
        t = self.vstack.pop()
        c = self.vstack.pop()
        self.vstack.append(Cond(c,t,e))
    def pushBin(self,a):
        self.binval = a[0]
    def blockIn(self,_):
        self.bstack.append((self.vstack, self.binval))
        self.binval = None
        self.vstack = []
    def blockOut(self,_):
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
            self.vstack.append(Bin(self.binval, Parser.funs[self.binval], l, r))
            self.binval = None
    def __init__(self):
        self.vstack = []
        self.bstack = []
        self.binval = None
        #var_name = Word('ABCDEFGHIJKLMNOPQRSTUVWXYZ')
        #var_num  = Word('1234567890')
        var = Regex('[A-Z][A-Z0-9]*').setParseAction(self.pushVar)
        val = (Word('false') | Word('true')).setParseAction(self.pushVal)
        _lp = Literal('(').suppress().setParseAction(self.blockIn)
        _rp = Literal(')').suppress().setParseAction(self.blockOut)
        _lr = Literal('{').suppress()
        _rr = Literal('}').suppress()
        expr = Forward()
        #func = (Literal('&&') | Literal('||') | Literal('==') | Literal('!=')).setParseAction(self.pushBin)
        func = reduce(lambda a,b: a | b, [Literal(k) for k in Parser.funs.keys()]).setParseAction(self.pushBin)
        _if = Literal('if').suppress()
        cond = Group(_if + _lr + expr + _rr + _lr + expr + _rr + _lr + expr + _rr).setParseAction(self.pushCond)
        nsym = Literal('~').setParseAction(self.dumpNot)
        fnot = (nsym + (Group(var) | Group(val) | Group(_lp + expr + _rp))).setParseAction(self.pushNot)
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

def getAllVars(expr):
    acc = set()
    stack = [expr]
    while len(stack) > 0:
        e = stack.pop()
        if type(e) == Var:
            acc.add(e.name)
        elif type(e) == Bin:
            stack.append(e.a)
            stack.append(e.b)
        elif type(e) == Not:
            stack.append(e.expr)
        elif type(e) == Cond:
            stack.append(e.cond)
            stack.append(e.yes)
            stack.append(e.no)
    acc = list(acc)
    acc.sort()
    return acc

def makeTbl(expr,vnames):
    envs = [{}]
    for v in vnames:
        new = []
        for e in envs:
            e1 = dict(e)
            e1[v] = True
            e[v] = False
            new.append(e1)
        envs.extend(new)
    for env in envs:
        env['!'] = expr.eval(env)
    return envs

def showTbl(envs,names):
    line = ''
    for v in names:
        line = '%s|%s' % (line, v)
    line = line + '|!|'
    print(line)
    for e in envs:
        line = ''
        for v in names:
            line = '%s|%d' % (line, int(e[v]))
        line = '%s|%d|' % (line, int(e['!']))
        print(line)

def splitByUsing(expr, envs, names):
    used   = []
    unused = []
    for v in names:
        inUse = False
        for e in envs:
            e[v] = not e[v]
            inUse = expr.eval(e) != e['!']
            e[v] = not e[v]
            if inUse:
                break
        if inUse:
            used.append(v)
        else:
            unused.append(v)
    used.sort()
    unused.sort()
    return (used,unused)
