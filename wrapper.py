import cffi

class CPPWrap:
    def __init__(self, path):
        ffi = cffi.FFI()
        ffi.cdef('void         init_funs();')
        ffi.cdef('void*        new_state(int*, int);')
        ffi.cdef('void         delete_state(void*);')
        ffi.cdef('void*        find_analog(void*,int,int);')
        ffi.cdef('int          expr_depth(void*,void*);')
        ffi.cdef('const char*  show_expr(void*, void*);') # (expr, tbl)
        ffi.cdef('void         delete_expr(void*);')
        self._lib = ffi.dlopen(path)
        self._lib.init_funs()
        self._ffi = ffi
    def findAnalog(self, boolTbl, varCnt, tryCount, maxDepth):
        lib = self._lib
        cBoolTbl = self._ffi.new("int[]", len(boolTbl))
        for i in range(len(boolTbl)):
            cBoolTbl[i] = int(boolTbl[i])
        tbl = lib.new_state(cBoolTbl, varCnt)
        ans = lib.find_analog(tbl, tryCount, maxDepth)
        if ans:
            res = self._ffi.string(lib.show_expr(ans,tbl)).decode('utf8')
            lib.delete_expr(ans)
        else:
            res = None
        lib.delete_state(tbl)
        return res

class RustWrap:
    def __init__(self, path):
        ffi = cffi.FFI()
        ffi.cdef('void* get_expr(int,int*);')
        ffi.cdef('char* show_expr(void*);')
        ffi.cdef('void rem_expr(void*);')
        self._lib = ffi.dlopen(path)
        self._ffi = ffi
    def findAnalog(self, boolTbl, varCnt, tryCount, maxDepth):
        lib = self._lib
        cBoolTbl = self._ffi.new("int[]", len(boolTbl))
        rng = range(len(boolTbl))
        for (i,j) in zip(rng, reversed(rng)):
            cBoolTbl[j] = int(boolTbl[i])
        ans = lib.get_expr(varCnt, cBoolTbl)
        if ans:
            res = self._ffi.string(lib.show_expr(ans)).decode('utf8')
            lib.rem_expr(ans)
        else:
            res = None
        return res
