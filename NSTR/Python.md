# Python --

     <<>>          (Python Cheat Sheet)
     <<>>    (Python Cheat Sheet)
     <<>>     
     <<>>     
     <<>>    
     <<>> 

    None notes:  cannot be added to string.
    .index and map get throw exceptions!

    os.path.getsize()
    time.sleep(1)
    spawn exec popen

    import string;
    string.index(str1, str2)  !!!!

    execfile()
     switch ???
    abspath()

    RANDOM PYTHON BITS

    for x in range(10,20)
    sys.exit()


    global g=7 
    def foo(g=g): 
      x=g+g 
     
    ---- 
    if x in list: 
     
    --- 
    import "  time.sleep(5)  seconds 
     
    --- 
    os.exec 
    os.spawnlp(os.P_NOWAIT, path, ARG0, arg1, ...) 
    l=listargs, v=argv, e=env@end, p=searchPath, exec=do not return 
    note ARG0!! 
    --- 
    int(raw_input("$")) 
    --- 
    sys.stdout.write("no carriage return") 
     

     
     
     
     
    ? help inspect pydoc 
     
    <<>>  dir() <<>> <<>>   
     
    None 
     
    sys.argv 
     
    import linecache.getline(filename, lineno)  
    import copy.copy(x) copy.deepcopy(x) 
    import pp=pprint.PrettyPrinter(indent=2)  pp.pprint(x) 
    import re 
    import dircache   a=dircache.listdir('<<>>') 
    import getopt.getopt(args, 'abc:d:') 
     
     
     
    import os os.getcwd()  os.getenv('PATH')  os.putenv(k,v) 
    tmpfile()  execl(path, arg0, arg1, ...)  
     
        from random import Random 
        g = Random(firstseed) 
        result = [g] 
     
    randrange([start,] stop[, step])  
     
    compileFile(source)  
    os.path.join(dir, file) 
     
    import sys, operator 
    nums = map(float, sys.argv[1:]) 
    print reduce(operator.add, nums)<<>>(nums) 
     
     
     
    [VARS] x=y=1  x,y=1,2 
     
    [CONTROL] 
    #Comment 
    while x<10:   break continue 
    if x==1:   action 
    elif x==2: action 
    else:      action 
    lambda <<>> x == " " 
    def proc(a, b=7, ): 
      """doc string""" 
      body 
     
    class <<>> 
        def method(self): 
            pass 
     
    c = C() 
    c.method.im_func.whoami = 'my name is c' 
     
     
     
    [STRING]  "string" 'one' word=r"\raw\str"   
      word[1:3]=="ra"  len(word)   
     
    r"'x'*5 
     
    [LIST]  l=['one', 'two', 'three']   
      [3*x for x in l]  range(5,10) 
      s[i]=x  s[<<>>]=t   del s[<<>>]  s.append(x)  s.count(x)   s.index(x)  s.len() 
      s.insert(i, x)     s.pop([i])  s.remove(x)  s.reverse()  s.sort([cmpfunc]) 
     
     
    [MAP]  tel=<<>>  tel['k2']  tel.has_key('k2') 
      len(a)  a[k]  a[k]=v  del a[k]  a.clear()  a.copy()  a.has_key(k) 
      k in a   k not in a   a.items()   a.keys() a.update(b)  a.values() 
      a.get(k[, x])  a.setdefault(k[, x]) a[k]  a.popitem()  a.iteritems() 
      a.iterkeys()  a.itervalues()  
     
     
     
     
    [IO] 
    raw_input 
    print "%i %f5.2 %s8" % 1 1.2 'str' 
     
    [TOP] 
    #!<<>> python 
    import os 
    src=os.environ.get('PYTHONSTARTUP') 
     
     
     
     
    Addition a + b add(a, b)  
    Concatenation seq1 + seq2 concat(seq1, seq2)  
    Containment Test o in seq contains(seq, o)  
    Division a <<>> b div(a, b) # without __future__.division  
    Division a <<>> b truediv(a, b) # with __future__.division  
    Division a <<>> b floordiv(a, b)  
    Bitwise And a & b and_(a, b)  
    Bitwise Exclusive Or a ^ b xor(a, b)  
    Bitwise Inversion ~ a invert(a)  
    Bitwise Or a | b or_(a, b)  
    Indexed Assignment o[k] = v setitem(o, k, v)  
    Indexed Deletion del o[k] delitem(o, k)  
    Indexing o[k] getitem(o, k)  
    Left Shift a << b lshift(a, b)  
    Modulo a % b mod(a, b)  
    Multiplication a * b mul(a, b)  
    Negation (Arithmetic) - a neg(a)  
    Negation (Logical) not a not_(a)  
    Right Shift a >> b rshift(a, b)  
    Sequence Repitition seq * i repeat(seq, i)  
    Slice Assignment seq[<<>>] = values setslice(seq, i, j, values)  
    Slice Deletion del seq[<<>>] delslice(seq, i, j)  
    Slicing seq[<<>>] getslice(seq, i, j)  
    String Formatting s % o mod(s, o)  
    Subtraction a - b sub(a, b)  
    Truth Test o truth(o)  
    Ordering a < b lt(a, b)  
    Ordering a <= b le(a, b)  
    Equality a == b eq(a, b)  
    Difference a != b ne(a, b)  
    Ordering a >= b ge(a, b)  
    Ordering a > b gt(a, b)  
     
     
     
    pickle 
     
    dump(object, file[, bin])  
    Write a pickled representation of object to the open file object file. This is equivalent to Pickler(file, bin).dump(object). If the optional bin argument is true, the binary pickle format is used; otherwise the (less efficient) text pickle format is used (for backwards compatibility, this is the default).  
    file must have a write() method that accepts a single string argument. It can thus be a file object opened for writing, a StringIO object, or any other custom object that meets this interface.  
     
     
    load(file)  
    Read a string from the open file object file and interpret it as a pickle data stream, reconstructing and returning the original object hierarchy. This is equivalent to Unpickler(file).load().  
    file must have two methods, a read() method that takes an integer argument, and a readline() method that requires no arguments. Both methods should return a string. Thus file can be a file object opened for reading, a StringIO object, or any other custom object that meets this interface.  
     
    This function automatically determines whether the data stream was written in binary mode or not.  
     
     
    dumps(object[, bin])  
    Return the pickled representation of the object as a string, instead of writing it to a file. If the optional bin argument is true, the binary pickle format is used; otherwise the (less efficient) text pickle format is used (this is the default).  
     
    loads(string)  
    Read a pickled object hierarchy from a string. Characters in the string past the pickled object's representation are ignored.  
    The pickle module also exports two callables3.4, Pickler and Unpickler:  
     
     
    class Pickler(file[, bin])  
    This takes a file-like object to which it will write a pickle data stream. Optional bin if true, tells the pickler to use the more efficient binary pickle format, otherwise the ASCII format is used (this is the default).  
    file must have a write() method that accepts a single string argument. It can thus be an open file object, a StringIO object, or any other custom object that meets this interface.  
     
    Pickler objects define one (or two) public methods:  
     
     
    dump(object)  
    Write a pickled representation of object to the open file object given in the constructor. Either the binary or ASCII format will be used, depending on the value of the bin flag passed to the constructor.  
     
    clear_memo()  
    Clears the pickler's ``memo''. The memo is the data structure that remembers which objects the pickler has already seen, so that shared or recursive objects pickled by reference and not by value. This method is useful when re-using picklers.  
    Note: clear_memo() is only available on the picklers created by cPickle. In the pickle module, picklers have an instance variable called memo which is a Python dictionary. So to clear the memo for a pickle module pickler, you could do the following:  
     
     
    mypickler.memo.clear() 
     
    It is possible to make multiple calls to the dump() method of the same Pickler instance. These must then be matched to the same number of calls to the load() method of the corresponding Unpickler instance. If the same object is pickled by multiple dump() calls, the load() will all yield references to the same object3.5.  
     
    Unpickler objects are defined as:  
     
     
    class Unpickler(file)  
    This takes a file-like object from which it will read a pickle data stream. This class automatically determines whether the data stream was written in binary mode or not, so it does not need a flag as in the Pickler factory.  
    file must have two methods, a read() method that takes an integer argument, and a readline() method that requires no arguments. Both methods should return a string. Thus file can be a file object opened for reading, a StringIO object, or any other custom object that meets this interface.  
     
    Unpickler objects have one (or two) public methods:  
     
     
    load()  
    Read a pickled object representation from the open file object given in the constructor, and return the reconstituted object hierarchy specified therein.  
     
    noload()  
    This is just like load() except that it doesn't actually create any objects. This is useful primarily for finding what's called ``persistent ids'' that may be referenced in a pickle data stream. See section 3.14.5 below for more details.  
    Note: the noload() method is currently only available on Unpickler objects created with the cPickle module. pickle module Unpicklers do not have the noload() method.  
     
     
