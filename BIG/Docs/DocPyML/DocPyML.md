
## show level three

# Docs

## PYTHON ML/AI DOCUMENTATION ===
 - (Getting Started With Python For Data Science)
   https://www.kaggle.com/wiki/GettingStartedWithPythonForDataScience
 - Tut for NLP words    http://nbviewer.ipython.org/url/norvig.com/ipython/How%20to%20Do%20Things%20with%20Words.ipynb
 - Quick NP/SP overview
   http://www.engr.ucsb.edu/~shell/che210d/numpy.pdf
 - Cheat sheet   https://s3.amazonaws.com/quandl-static-content/Documents/Quandl+-+Pandas,+SciPy,+NumPy+Cheat+Sheet.pdf
 
## REFERENCE
 - Numpy userguide  http://docs.scipy.org/doc/numpy/user/
 - Numpy ref  http://docs.scipy.org/doc/numpy/reference/
 - Scipy ref  http://docs.scipy.org/doc/scipy/reference/

## ML/AI EXAMPLES
- k-means   http://glowingpython.blogspot.com/2012/04/k-means-clustering-with-scipy.html
- np array tricks http://www.loria.fr/~rougier/teaching/numpy/numpy.html
PLOTTING  http://jakevdp.github.io/mpl_tutorial/tutorial_pages/tut1.html
- EX BIO  http://people.duke.edu/~ccc14/pcfb/numpympl/MatplotlibBarPlots.html
- STRUCTURED DATA http://docs.scipy.org/doc/numpy/user/basics.rec.html


# NumPy

## Scalars
import numpy as np
type(np.asscalar(np.float32(0))) # <type 'float'>
type(np.asscalar(np.float64(0))) # <type 'float'>
type(np.asscalar(np.uint32(0)))  # <type 'long'>
type(np.asscalar(np.int16(0)))   # <type 'int'>
type(np.asscalar(np.cfloat(0)))  # <type 'complex'>
type(np.asscalar(np.datetime64(0)))  # <type 'datetime.datetime'>
type(np.asscalar(np.timedelta64(0))) # <type 'datetime.timedelta'>


# Lib SciPy




(See file:///ob/topics/Docs/src/Blam_Python/Blam_Python.md )
# Blam Python
--------------------------------------------------------------------
                            ===  BLAM PYTHON !  ===
[DOCS]    Search http://docs.python.org/search.html  http://www.python.org/doc/ 
[SCRIPT]  #!/usr/bin/env python     
   import os; os.getcwd()  os.getenv('PATH') os.putenv(k,v) os.environ
   os.chdir(os.path.dirname(sys.argv[0])) os.getcwd()
   import getopt; opts,rest=getopt.getopt(sys.argv[1:],'abc:d:')
   os.execlp("emacs", "emacs", "file.txt")
   user_time,sys,child_user,child_sys,elapsed = os.times()
   os.urandom(n)
[CONTROL] 
   while not x!=None:  break; continue; yo=1;   \     <--- single backslash will continue on next line
   if True and not x==None: y=1; z='two'  \\ elif x==2: pass \\ else: pass
   for ele in list: print ele   for i in range(1, 10): print i
   { 'case1': 'val1', 'case2':'val2' }.get(switch_index, 'default_val')
   apply( lambda x,y:x+y , (2,3) )
   def square(x, root=false): return x*x    #Comment '\\' means indented on next line
   def proc(a, b=7, ): \\  \\ global gname \\ my (loc_name)=(1) \\ body
   class C: \\  def method_name(self): \\  self.inst_var=5
     def __init__(self,x): pass
   c = C(); c.method.im_func.whoami='my name is c'
   try: \\ body \\except IOError, msg: pass \\ except: general \\ finally: noerr
[VARS/OPS] x=y=1;  x,y="parallel",'assign'   my(x)=(3);   Operator precedence:
   lambda or and not  [in, not in]  [is, is not]  [<, <=, >, >=, <>, !=, ==]
   | ^ &  [<<, >>]  [+, -]  [*, /, %]  [+x, -x]  ~x ** x.attr x[index]
   x[y:z] f(arg...) (expr...) [exprs...] {key:val...} `expr...`
   global a,b  # allows assignment to global variables
   a if b else c
   abs(x) int(x) long(x) float(x) complex(r,i) c.conjugate() pow(x) x**y  div,mod=divmod(x,y)
   math.trunc(x) round(x,n) math.floor(x) math.ceil(x)
[ITERATOR]  .__iter__() .next()
[TUPLE] t=(1,2)  t[0:-1]  t[0:-1,2]   l3=l1+l2  x in tup    ==,<=,...
   int<--  len(s) max(s) min(s)    .count(x)
   tup<--  range(5,10)   tuple(itr) 
[SEQ] list(itr)  s.copy()
[LIST]. l=['one','two']; list(iterable);  l2=None   acopy=l3[:]   
   <--      str=", ".join(s);    if (x in l) or (l == None): pass
   ELE<--  .index(index, default)  .pop()  .reduce(sum,s)   
   MODIFY: .append(ele)  s.extend(itr)   .insert(i,x)  .remove(x)  s[i]=x  s[i:j]=tuple   del s[i:j]
   seq<--  l[0:-1]  l[0:-1,2]   [x*x for x in l if x%2==0]  
           .reverse()  .filter(lambda x:x>1,s)  .sort([cmpfunc]) reps*lst
[DICT]. d={'key1':'val1','k2':22}  dict(itr)  dict(map)  dict(k=v, ...)   fromkeys(keys,vals)
           [ k for (k,v) in tel.items() ];   for k,v in a.items(): pass
  ele<--   d[k]  .get(k,default)
  int<--   len(a)
  <--      x in d   k not in d   .has_key(k)
  MODIFY   d[k]=v   del a[k]   .pop(k,def)   .clear() .update(b)    d.setdefault(k[,x])  random_pair=d.popitem()
  dict<--  .copy()
  VIEW<--  .viewitems()  .veiwkeys()  .viewvalues()    VIEW OPS:  v3=v1&v2  v1|v2  v1-v2  v1^v2
  seq<--   .keys() .items()  
  itr<--   iter(d)  .iteritems()  .iterkeys()  .itervalues()
[STRING]. len("a"+"b")  str(99.99)
   x,y,z,r = "to", 'to', `2`, r"\raw\str"       """multi-line doc string"""
   "format Str %s %c %i %d %e %f %%  \r\n \t \xFF \u00FF" % ('aa', 'a', 7, Decimal(7) 7.1, 7.1e+00)
   "%r %s  %-i %0i" % ('repr', 'str', left_adjusted, zero_padded,  )
   print '{0:2d} {1:3d} {2:4d}'.format(x, x*x, x*x*x)
   if ch in str: pass; if str.find('sub')!=-1:  ch==ord(chr(ch))
  bool<--  .endswith(suffix[,start[,end]])  .startswith(prefix[,start[,end]])   
           .isalnum()  .isalpha() .isdigit() .islower()  .isspace()  .istitle()  .isupper() 
  int<--   len(s)  .index(sub[,start[,end]]) .count(sub[,start[,end]])   .find(sub[,start[,end]])    .rindex   .rfind
  str<--   s[:-2]  .format(arg1,...)
  MODIFY   s[1:3]=="ra"  .replace(old,new[,maxsplit])  .swapcase() 
  seq<--   .strip("stripchars") .lstrip  .rstrip .splitlines([keepends])
  PAD      .ljust(width) .rjust(width)   .lstrip()  .center(width)  .expandtabs([tabsize])  .zfill(w)
  CASE     .lower()  .upper()  .capitalize() .title() 
  OTHER    .decode([enc[,err]])  .encode([enc[,err]])  .translate(table[,delchs])
  (before,sep,after)=s.partition(sep)  .rpartition
[IO]
   print "%i %f5.2 %s8 %s" % (1, 1.2, 'str', int( raw_input('prompt> ') ) ), end=''

   sys.stdout.write('prompt> ')   sys.stdout.flush()
   o=open('out.txt','w+');   # r=read w=write a=append b=binary +=andUpdate
   o.write("xx"); o.writelines(lst); o.close();  
   inp=open('in','r'); while line=inp.readline():
   for each in o.readlines():  y=each     # iterate through the file
   open(fname[,mode[,bufsize]])    sys.stdout.write("no carriage return")
   tmpfile()
   user=raw_input();  lines = sys.stdin.readlines();  
import os    # import the os module first
os.path.join(path_segment1, path_segment2, ...)
os.path.exists(path)
os.listdir(directory_path)
os.remove(file_path)
os.rmdir(directory_path)
file = open(path, "rw")
file.read()
string.write("string")
{FILE} .close() .flush() .next() .read([size]) .readline([size]) .readlines([size]) .write(s) .writestr(seq) 
[DIR] 
  import os;  os.remove(f) .rename(f1,f2)  os.chdir(d)  a=os.listdir('/')
  os.path.... .join(dir,file) .dirname(f) .basename(f)
  os.path.exists(f) .getsize(f) .getatime(f) .getmtime(f) 
         .isfile(f) .isdir(f) .split(f) .splitdrive(f) .splitext(f)
  os.popen(command,mode,bufSize) os.tmpfile() os.stat(f)
[MODULES] import module;  from module import class, function, variable
  from pickle import *;   dump(obj,file);  load(file);  dumps(obj);  loads(str) 
  import linecache;       linecache.getline(filename, lineno) 
  import copy;            copy.copy(x) copy.deepcopy(x)
  import pprint;          pp=pprint.PrettyPrinter(indent=2)  pp.pprint(x)
  from random import *;   randrange(0,8,2) --> 0,2,4, or 6
  from datetime import datetime;    str(datetime.fromtimestamp(time));  str(datetime.datetime.utcnow())
  import dateutil.parser; yourdate = dateutil.parser.parse(datestring)

[CLASS/FN]
  def fn(*args, **kwmap):
  class ClassName(SuperClass): 
    classVar=1
    def __inst__(self):
      self.instVar=2
    def method(self, arg1): pass
  isinstance(Decimal(4), Decimal)
  .__dict__  .__class  .__bases__ .__name__ .__mro__ .mro() .__subclasses__()
[SYSTEM]
  time.sleep(5)  os.spawnlp(os.P_NOWAIT, path, ARG0, arg1, ...)
  os.exec(...) l=listargs, v=argv, e=env@end, p=searchPath, exec=do not return
[FUNCTIONAL]
  args=filter(lambda x:x!='',args)
  nums = map(float, sys.argv[1:])
  print reduce(operator.add, nums)/len(nums)
[REGEXP]   re.search('([^/]*)\.mp3', fileName).group(1)
  . ^ $ * + ? *? +? ?? {m} {m,n} {m,n}? \ [] | (...) (?...)
  (?iLmsux) (?:...) (?P<name>...) (?P=name) (?#...) (?=...) (?!...)  (?<=...)
  (?&lt;!...) 
   m=re.search('(.*)\.mp3',f) m.group(1)
  import re;  m=re.search('(?<=abc)def','abcdef');  m.group(0); --> 'def'
  #This example looks for a word following a hyphen: 
  m=re.search('(?<=-)\w+', 'spam-egg');  m.group(0)  -->  'egg'
  re.compile(pat[,flgs])   .match("ba", l)  .search("ba", 1) 
  search(str[,pos[,end]])  match(str[,pos[,end]]) split(str[,max=0]) 
  findall(string)  finditer(string) 
  sub(repl,str[,cnt=0])  subn(repl,str[,count=0]) 
  flags  groupindex  pattern 
  \A begin-str \b \B begin-word \d digit \D !digit \s space \S !space
  \w word \W !word \Z end-str  
  STANDARD:  \a  \b  \f  \n  \r  \t  \v  \x  \\
[SPECIAL]
  __dict__  dir() __class__ __bases__  
[TOOLS] compileFile(source) ? help inspect pydoc
 .dir(x)


[SET]       set(itr)  frozenset(itr)   .isdisjoint(other)  s<o==s.issubset(o) s>o==s.issuperset(o) s|o==s.union(o)
            s&o==s.insersection(o) s-o==s.difference(o) s^o==s.symmetric_difference(o)
            .update(o)  .intersection_update(o) .difference_update(o) .symmetric_difference_update(o)
            .add(e) .remove(e) .discard(e) .pop() .clear()

[MORE FILE]   .seek(offset[,os.SEEK_CUR]) .tell()  .fileno() .isatty()      .truncate([sz])   .closed .encoding .errors .mode .name .newlines .softspace
[MEMORY VIEW, CONTEXT MANAGER TYPE, ]

[CSV]
import csv
>>> with open('eggs.csv', 'rb') as csvfile:
...     spamreader = csv.reader(csvfile, delimiter=' ', quotechar='|')
...     for row in spamreader:
...         print ', '.join(row)
import csv
with open('eggs.csv', 'wb') as csvfile:
    spamwriter = csv.writer(csvfile, delimiter=' ',
                            quotechar='|', quoting=csv.QUOTE_MINIMAL)
    spamwriter.writerow(['Spam'] * 5 + ['Baked Beans'])
    spamwriter.writerow(['Spam', 'Lovely Spam', 'Wonderful Spam'])
#SNIFFER
with open('example.csv', 'rb') as csvfile:
    dialect = csv.Sniffer().sniff(csvfile.read(1024))
    csvfile.seek(0)
    reader = csv.reader(csvfile, dialect)
    # ... process CSV file contents here ...

csv.list_dialects()




# Other

## Blam NumPy
(see python BLAM)

import numpy as np
np.bool, np.float32

IN PLACE
 y[:] = 2*y

## Help

lookfor(what[, module, import_modules, ...])	Do a keyword search on docstrings.
Reading help
info([object, maxwidth, output, toplevel])	Get help information for a function, class, or module.
source(object[, output])	Print or write to a file the source code for a Numpy object.



# Py Docs
- Py Tutorial          http://docs.python.org/3/tutorial/modules.html
- Py for data science  https://www.kaggle.com/wiki/GettingStartedWithPythonForDataScience
- Tut for NLP words    http://nbviewer.ipython.org/url/norvig.com/ipython/How%20to%20Do%20Things%20with%20Words.ipynb

- Quick Ref http://rgruet.free.fr/PQR25/PQR2.5.html  (import into blam)

https://zapier.com/engineering/debugging-python-boss/


EXAMPLES
- Flask  https://github.com/mitsuhiko/flask/blob/master/flask/blueprints.py
- ?Praw https://github.com/praw-dev/praw/blob/master/praw/helpers.py
- ?Pyrimid  https://github.com/Pylons/pyramid/blob/master/pyramid/authentication.py


TO LOOK AT WHEN STUDYING
- http://climateecology.wordpress.com/2014/02/10/a-side-by-side-example-of-r-and-python/
http://blog.scrapinghub.com/2014/03/26/optimizing-memory-usage-of-scikit-learn-models-using-succinct-tries/
http://www.datasciencecentral.com/profiles/blogs/sample-data-science-project-optimizing-all-business-levers-simult



## Python usage
- As Sci Stack  http://www.r-bloggers.com/the-homogenization-of-scientific-computing-or-why-python-is-steadily-eating-other-languages-lunch/
- Building ML stack on Vagrant    http://jeroenjanssens.com/2013/12/07/lean-mean-data-science-machine.html


## Doc Topics

### Python speed
-  http://code-redefined.blogspot.com/2011/03/cython-made-my-python-code-go.html



# Python Other


        

LAMBDA
global='g'
def foo(list):
   lexical='loc'
   map(lambda arg,lexical=lexical: arg+lexical+global, list)

#!/usr/local/bin/python
#########################################################################
#                                                                       #
#  Reserved Words:                                                      #
#                                                                       #
#        and             elif            global          or             #
#        assert          else            if              pass           #
#        break           except          import          print          #
#        class           exec            in              raise          #
#        continue        finally         is              return         #
#        def             for             lambda          try            #
#        del             from            not             while          #
#                                                                       #
#########################################################################

#########################################################################
#                                                                       #
#  Importing Modules:                                                   #
#                                                                       #
#########################################################################
import sys                             # access module
from sys import stdout                 # access module without qualiying name
from sys import *                      # ditto for all functions/classes in module
from win32com.client import constants  # packages are supported - requires __init__.py in dir
exec "import sys"                      # dynamic imports are possible
__import__("sys")                      # another form of dynamic import
reload(sys)                            # reload module code

x = sys.argv                           # access module feature: module.feature
x = argv                               # access module feature without qualifer - from
x = sys.path                           # module search path - can be modified - PYTHONPATH
x = sys.__dict__["argv"]               # each module has dictionary used for name lookups
x = getattr(sys, "argv")               # call built in fetch function
argv = ["/"]                           # from only assigns name - sys.argv is not changed

#########################################################################
#                                                                       #
#  hello:                                                               #
#                                                                       #
#########################################################################
print "hello world"                    # stdout
print "hello", "world",                # comma on end suppresses newline
print ""
x = sys.argv                           # list of command line arguments
#sys.exit()                            # exit application

#########################################################################
#                                                                       #
#  line spanning:                                                       #
#                                                                       #
#########################################################################
x = "hello" + \
   "world"                             # span several lines- no comments after \
if (0 and                              # open parenthesis may span lines
   1): pass
x = [                                  # open list may span lines
   "hello",
   "world"
]
x = 10; y = 10                         # multiple statements on a line

#########################################################################
#                                                                       #
#  Primitive Variable Types:                                            #
#                                                                       #
#########################################################################
x = 123                                # integer
x = 123L                               # long integer
x = 3.14                               # double float
x = 3+4j                               # complex
x = "hello"                            # string
x = [0,1,2]                            # list
x = (0,1,2)                            # tuple
x = open('hello.py', 'r')              # file

x = 077                                # octal number
x = 0xFF                               # hexadecimal number
x = 3.14E-2                            # scientific format
x = None                               # null value

X = 1                                  # python is case sensitive X &lt;&gt; x
x = (1 &lt; 2)                            # boolean: False = 0 (0, Empty, or None); True = 1 | !0

#########################################################################
#                                                                       #
#  Strings:                                                             #
#                                                                       #
#########################################################################
x = ""                                 # empty string
x = 'hello'                            # strings can use either single or double quote
x = r"hello\n"                         # raw string - retain backslashes
x = """hello"""                        # triple quoted strings may span several lines

y = `x`                                # convert to string
y = str(123)                           # convert to string
x = "hello" * 3                        # string repeat
x = "hello" + " world"                 # string concatenate
y = len(x)                             # string length
for char in x: y = char                # iterate through characters

                                       # string slicing
y = x[:2]                              # left(s, b)               - "he"
y = x[:-2]                             # left(s, len(s)+b)        - "hello wor"
y = x[-2:]                             # right(s, -a)             - "ld"
y = x[6]                               # mid(s, a, 1)             - "w"
y = x[2:4]                             # mid(s, a, b-a)           - "ll"
y = x[2:-3]                            # mid(s, a, len(s)+b-a)    - "llo wo"
y = x[-4:-2]                           # mid(s, len(s)-a, -a-b)   - "or"

                                       # string backslash characters
x = "\\"                               # backslash
x = "\'"                               # single quote
x = "\""                               # double quote
x = "\a"                               # bell
x = "\b"                               # backspace
x = "\e"                               # escape
x = "\000"                             # null that does not end string
x = "\n"                               # newline
x = "\v"                               # vertical tab
x = "\t"                               # tab
x = "\r"                               # carriage return
x = "\f"                               # formfeed
x = "\077"                             # octal character
x = "\x7F"                             # hex character
x = "\z"                               # any other chars - the slash &amp; char is retained (\z)

                                       # string formatting
x = "hello %s" % "world"               # %s = string
x = "hello %c" % 0x77                  # %c = character
x = "hello %d" % 123                   # %d = decimal int
x = "hello %i" % 123                   # %i = integer
x = "hello %u" % 123                   # %u = unsigned integer
x = "hello %o" % 077                   # %o = octal integer (w/o leadin 0)
x = "hello %x" % 0xFF                  # %x = hex integer lowercase (w/o leadin 0x)
x = "hello %X" % 0xFF                  # %X = hex integer uppercase (w/o leadin 0x)
x = "hello %e" % 1.23                  # %e = floating point (1.230000e+000)
x = "hello %E" % 1.23                  # %E = floating point (1.230000E+000)
x = "hello %f" % 1.23                  # %f = floating point (1.230000)
x = "hello %g" % 1.23                  # %g = floating point (1.23)
x = "hello %G" % 1.23                  # %G = floating point (1.23)
x = "hello %s%%" % "world"             # %% = literal percent character

#########################################################################
#                                                                       #
#  Lists:                                                               #
#                                                                       #
#########################################################################
x = []                                 # empty list
x = [0, 1, 2, "abc"]                   # four item list: indexed x[0]..x[3]
x = [0, 1, 2, 3, [1, 2]]               # nested sublists
y = len(x)                             # list length
y = x[0]                               # indexed item
y = x[4][0]                            # indexed sublist
x = [0, 1] * 2                         # repeat
x = [0, 1, 2] + [3, 4]                 # concatenation
x = range(5)                           # make list over range
x = range(0, 5)                        # make list over range with start index
x = range(0, 5, 1)                     # make list over range with start index and increment
for item in x: y = item                # iterate through list
b = 3 in x                             # test list membership

                                       # list slicing
y = x[:2]                              # left(L, n)               - [0, 1]
y = x[:-2]                             # left(L, len(L)+b)        - [0, 1, 2]
y = x[-2:]                             # right(L, -a)             - [3, 4]
y = x[2:4]                             # mid(L, a, b-a)           - [2, 3]
y = x[1:-3]                            # mid(L, a, len(L)+b-a)    - [1]
y = x[-4:-2]                           # mid(L, len(L)-a, -a-b)   - [1, 2]

                                       # methods
x.append(5)                            # grow list: x[len(x):] = [a]
x.extend([7,8])                        # grow list: x[len(x):] = L
x.insert(0, 9)                         # insert into list
x.remove(9)                            # remove first item in list with value
y = x.pop()                            # remove last item from list and return value
y = x.pop(1)                           # remove indexed item from list and return value
x.reverse()                            # reverse list
x.sort()                               # sort list
y = x.index(3)                         # search list and return index if value found
y = x.count(3)                         # search list and return number of instances found

del x[3]                               # delete item from list (can also del a slice)
x[1:2] = []                            # delete slice

x[1] = 'a'                             # replace item
x[1:2] = ['a', 'b', 'c']               # replace slice

x = [0, 1, 2, 3]
y = map(lambda a: a*a, x)              # apply function to each item in list
y = filter(lambda a: a&gt;1, x)           # return list of items that meet condition
y = reduce(lambda a,b: a+b, x)         # return value by applying iteration (sum list)

#########################################################################
#                                                                       #
#  Tuples:                                                              #
#                                                                       #
#########################################################################
x = ()                                 # empty tuple
x = (0,)                               # one item tuple
x = (0, 1, 2, "abc")                   # four item tuple: indexed x[0]..x[3]
x = 0, 1, 2, "abc"                     # parenthesis are optional
x = (0, 1, 2, 3, (1, 2))               # nested subtuples
y = x[0]                               # indexed item
y = x[4][0]                            # indexed subtuple
x = (0, 1) * 2                         # repeat
x = (0, 1, 2) + (3, 4)                 # concatenation
for item in x: y = item                # iterate through tuple
b = 3 in x                             # test tuple membership

                                       # tuple slicing
y = x[:2]                              # left(L, n)               - [0, 1]
y = x[:-2]                             # left(L, len(L)+b)        - [0, 1, 2]
y = x[-2:]                             # right(L, -a)             - [3, 4]
y = x[2:4]                             # mid(L, a, b-a)           - [2, 3]
y = x[1:-3]                            # mid(L, a, len(L)+b-a)    - [1]
y = x[-4:-2]                           # mid(L, len(L)-a, -a-b)   - [1, 2]

#########################################################################
#                                                                       #
#  Dictionaries:                                                        #
#                                                                       #
#########################################################################
x = {}                                 # empty dictionary
x = {"a": 10, "b": 20}                 # two item dictionary
x = {"a": 1, "b": {"c": 2, "d": 3}}    # nested dictionary
y = x["a"]                             # indexing by key
y = x["b"]["c"]                        # indexed subdictionary
x["e"] = 4                             # add an entry
x["e"] = 5                             # change an entry
del x["e"]                             # delete an entry
y = len(x)                             # number of items
y = x.has_key("a")                     # test if has key
y = x.get("a")                         # indexing by key
y = x.get("a", "z")                    # if key not found then return second arg
for item in x.keys(): y = item         # iterate over keys
for item in x.values(): y = item       # iterate over values
for key, val in x.items(): y = key     # iterate over copy of (key, value) pairs
y = x.copy()                           # shallow copy
x.update(y)                            # update dictionary with another dictionary's values
x.clear()                              # remove all items

#########################################################################
#                                                                       #
#  Files:                                                               #
#                                                                       #
#########################################################################
x = open("test.txt", "w")              # open file for write
x.write("hello\n")                     # write string
x.writelines(["fred\n", "hank\n"])     # write all strings in list
x.flush()                              # flush buffer
y = x.fileno()                         # file handle number
b = x.isatty()                         # true if tty device
b = x.closed                           # current state of file object
y = x.name                             # file name
b = x.softspace                        # boolean to indicate whether print requires space
x.close()                              # close file

x = open("test.txt", "r")              # open file for input

while 1:                               # iterate through the file one line at a time
   y = x.readline()
   if (not y): break
for each in x.readlines(): y = each    # iterate through the file

y = x.read()                           # read entire file into a string
y = x.read(5)                          # read n characters
y = x.readline()                       # read next line
y = x.readlines()                      # read file into list of strings
y = x.seek(0)                          # seek to the specified byte
y = x.tell()                           # tell current file position
x.close()

import pickle
x = open("test.txt", "w")
pickle.dump([0, 1, 2], x)              # save an object off to file - serialize
x.close

x = open("test.txt", "r")
y = pickle.load(x)                     # restore an object from file
x.close

#########################################################################
#                                                                       #
#  Operators:                                                           #
#                                                                       #
#        ``   convert to string             &amp;   bitwise and             #
#        ()   tuple                         ^   bitwise exclusive or    #
#        []   list                          |   bitwise or              #
#        {}   dictionary                    &lt;   less than               #
#       [x]   indexing                     &lt;=   less than or equal      #
#     [x:y]   slicing                       &gt;   greater than            #
#       m.f   qualification                &gt;=   greater than or equal   #
#       f()   function call                ==   equal                   #
#         -   unary negation               &lt;&gt;   not equal               #
#         +   unary positive               !=   not equal               #
#         ~   bitwise compliment           is   identity                #
#        **   exponentiation           is not   identity not            #
#         *   multiply | repeat            in   sequence membership     #
#         /   divide                   not in   sequence non-membership #
#         %   remainder (modulo)          not   logical not             #
#         +   add                         and   logical and             #
#         -   subtract                     or   logical or              #
#        &lt;&lt;   shift left               lambda   anonymous function      #
#        &gt;&gt;   shift right                                               #
#                                                                       #
#########################################################################
x = 0                                  # assignment
x = y = 0                              # compound assignment
[x, y, z] = [0, 1, 2]                  # positional list assignment: x=0, y=1, z=2
x, y, z = (0, 1, 2)                    # positional tuple assignment: x=0, y=1, z=2
b = (0 &lt; x &lt; 2)                        # range test

x = -5                                 # unary minus
x = +5                                 # unary plus
x = ~0x7f                              # bitwise compliment
x = 2 ** 8                             # power operator
x = 5 / 2                              # divide - if both int's then div function
x = 5 % 2                              # remainder modulo
x = 5 + 2                              # add
x = 5 - 2                              # subtract
x = 0x0f &lt;&lt; 2                          # shift left
x = 0x3c &gt;&gt; 2                          # shift right
x = 0x0f &amp; 0x8f                        # bitwise and
x = 0x0f | 0x80                        # bitwise or
x = 0x0f ^ 0x80                        # bitwise xor

x = [0, 1, 2]
y = [0, 1, 2]

b = (x == y)                           # equal
b = (x != y)                           # not equal - preferred
b = (x &lt;&gt; y)                           # not equal
b = (x &lt; y)                            # less than
b = (x &lt;= y)                           # less than or equal
b = (x &gt; y)                            # greater than
b = (x &gt;= y)                           # greater than or equal

b = (x is y)                           # identity reference
b = (x is not y)                       # different reference
b = (1 in x)                           # membership
b = (1 not in x)                       # not member
b = not 0                              # logical not
b = 0 and 1                            # logical and
b = 0 or 1                             # logical or

x = (1+2j).conjugate()                 # conjugate of complex number

#########################################################################
#                                                                       #
#  Flow Control:                                                        #
#                                                                       #
#########################################################################
x = 10
y = 10
b = 1

if (b):                                # if then else
   y = 1
elif (x == 10):
   y = 2
else:
   y = 3

while (x != 0):                        # while loop
   x = x - 1
   pass                                # empty placeholder
   if (b): continue                    # continue with next loop
   break;                              # break out of loop
else:                                  # run if didn't exit loop with break
   x = x + 1

for x in range(4):                     # for loop
   y = y + 1
   pass
   if (b): continue
   break;
else:                                  # run if didn't exit loop with break
   y = y + 1

#########################################################################
#                                                                       #
#  Exceptions and Assertions:                                           #
#                                                                       #
#########################################################################
class myGeneralError: pass             # define class and subclass to use with exceptions
class mySpecificError(myGeneralError): pass

try:                                   # try-catch block
   raise AttributeError                # throw an exception
   raise IOError, "bad IO"             # with an exception message
   raise mySpecificError()             # with user defined exception class
except AttributeError:                 # catch a specific exception
   pass
except myGeneralError:                 # catch user defined exception (subclasses also caught)
   pass
except IOError, msg:                   # catch a specific exception with extra data
   pass
except (RuntimeError, StandardError):  # catch any of the list of exceptions
   pass
except:                                # catch all exceptions not handled above
   pass
else:                                  # run if no exceptions thrown
   pass

try:                                   # try-finally block - note exception propogates up
   x = 1
finally:                               # always perform this block
   pass

assert(1&lt;2)                            # raise AssertionError if condition not met
assert(1&lt;2), "Assertion message"       # with message to display

#########################################################################
#                                                                       #
#  Functions:                                                           #
#                                                                       #
#########################################################################
def factorial(n):                      # define a function
   if (n == 1):
      return (1)
   else:
      return (n * factorial(n-1))

x = factorial(5)                       # call a function
x = factorial                          # indirect call to function
y = x(5)

def accessglobal():
   global glib                         # assign to var from global scope
   glib = 100

glib = 0
accessglobal()

def outer(a, b):
   i = 100
   def inner(c=a, d=b):                # only access to outer vars is thru default params
      i = 101                          # var is local to inner scope
   inner()                             # outer scope var not changed

x = outer(1, 2)


def test(a, b, c=0, d=0):              # default values for optional parameters
   pass                                # vals eval'd when def parsed - not when function called

test(1, 2)
test(1, 2, 3, 4)
test(a=1, b=2)                         # keyword matched parameters
test(1, d=1, b=2)

def posA(a, *b):                       # match remaining positional chars in tuple
   pass

posA(1, 2, 3, 4)

def posB(a, **b):                      # match remaining positional chars in dictionary
   pass

posB(1, b=2, c=3, d=4)

x = lambda a: a * a                    # lambda functions
y = x(2)

x = apply(factorial, (5,))             # built-in apply function

#########################################################################
#                                                                       #
#  Classes:                                                             #
#                                                                       #
#########################################################################
class Shape:
   def __init__(self, x, y):
      self.x = x
      self.y = y

class Drawable:
   def draw(self):
      pass

def getNumInstances():                 # class functions not supported - use module function
   return Circle.numInstances

class Circle(Shape, Drawable):
   numInstances = 0                    # class attribute (static)

   def __init__(self, x, y, radius):   # constructor
      Shape.__init__(self, x, y)
      self.radius = radius
      Circle.numInstances = Circle.numInstances + 1
   def __del__(self):                  # destructor
      pass

   def draw(self): print "Drawing a", self.__repr__()          # normal instance method
   def __mangle(self): pass                                    # mangle: _Circle__mangle

   def __repr__(self):                                         # print X or `X`
      return "Circle at:(%d,%d), radius %d" % (self.x, self.y, self.radius)
   def __str__(self): return self.__repr__()                   # convert to string
   def __hash__(self): return 0                                # 32 bit hash int
   def __nonzero__(self): return 1                             # truth test
   def __cmp__(self, other): return 0                          # compare self to other

   def __getattr__(self, name): return self.__dict__[name]     # control attribute access
   def __setattr__(self, name, val): self.__dict__[name] = val # set attribute value
   def __delattr__(self, name): pass                           # delete attribute
   def __call__(self, *args): return self.__repr__()           # function call x()

   def __len__(self): return 1                                 # get length
   def __getitem__(self, key): pass                            # get indexed item
   def __setitem__(self, key, val): pass                       # set indexed item
   def __delitem__(self, key): pass                            # delete indexed item

   def __getslice__(self, i, j): pass                          # get indexed slice
   def __setslice__(self, i, j, val): pass                     # set indexed slice
   def __delslice__(self, i, j): pass                          # delete indexed slice

   def __add__(self, other): pass                              # + operator
   def __sub__(self, other): pass                              # - operator
   def __mul__(self, other): pass                              # * operator
   def __div__(self, other): pass                              # / operator
   def __mod__(self, other): pass                              # % operator
   def __divmod__(self, other): return (1, 2)                  # divmod()
   def __pow__(self, other, *modulo): pass                     # pow()
   def __lshift__(self, other): pass                           # &lt;&lt; operator
   def __rshift__(self, other): pass                           # &gt;&gt; operator
   def __and__(self, other): pass                              # &amp; operator
   def __or__(self, other): pass                               # | operator
   def __xor__(self, other): pass                              # ^ operator

   def __radd__(self, other): pass                             # + operator - reversed
   def __rsub__(self, other): pass                             # - operator - reversed
   def __rmul__(self, other): pass                             # * operator - reversed
   def __rdiv__(self, other): pass                             # / operator - reversed
   def __rmod__(self, other): pass                             # % operator - reversed
   def __rdivmod__(self, other): return (1, 2)                 # divmod() - reversed
   def __rpow__(self, other): pass                             # pow() - reversed
   def __rlshift__(self, other): pass                          # &lt;&lt; operator - reversed
   def __rrshift__(self, other): pass                          # &gt;&gt; operator - reversed
   def __rand__(self, other): pass                             # &amp; operator - reversed
   def __ror__(self, other): pass                              # | operator - reversed
   def __rxor__(self, other): pass                             # ^ operator - reversed

   def __pos__(self): pass                                     # + operator - unary plus
   def __neg__(self): pass                                     # - operator - unary minus
   def __abs__(self): pass                                     # abs()
   def __invert__(self): pass                                  # invert function

   def __complex__(self): pass                                 # complex()
   def __int__(self): pass                                     # int()
   def __long__(self): pass                                    # long()
   def __float__(self): pass                                   # float()
   def __oct__(self): pass                                     # oct()
   def __hex__(self): pass                                     # hex()
   def __coerce__(self, other): pass                           # coerce for mixed mode

scribble = Circle(15, 25, 8)           # allocate object
scribble.draw()                        # method call
Circle.draw(scribble)                  # alternative form of method call
x = getNumInstances()                  # static function emulation
x = scribble.__dict__                  # dictionary used to store object attributes
x = scribble.__class__                 # class which object belongs to
del scribble                           # manually delete object

#########################################################################
#                                                                       #
#  Documentation strings:                                               #
#                                                                       #
#########################################################################
def docMe():
   "this is documentation for a function"
   pass

class docYou:
   "this is documentation for a class"

   def methYou(self):
      "this is a documentation for a method"
      pass

x = docMe.__doc__                      # retrieve documentation string
x = docYou.__doc__
x = docYou.methYou.__doc__

#########################################################################
#                                                                       #
#  __builtins__                                                         #
#                                                                       #
#        __debug__      divmod         isinstance     raw_input         #
#        __doc__        eval           issubclass     reduce            #
#        __import__     execfile       len            reload            #
#        __name__       exit           list           repr              #
#        abs            filter         locals         round             #
#        apply          float          long           setattr           #
#        buffer         getattr        map            slice             #
#        callable       globals        max            str               #
#        chr            hasattr        min            tuple             #
#        cmp            hash           oct            type              #
#        coerce         hex            open           vars              #
#        compile        id             ord            xrange            #
#        complex        input          pow                              #
#        delattr        int            quit                             #
#        dir            intern         range                            #
#                                                                       #
#        ArithmeticError         IndexError           RuntimeError      #
#        AssertionError          KeyError             StandardError     #
#        AttributeError          KeyboardInterrupt    SyntaxError       #
#        EOFError                LookupError          SystemError       #
#        Ellipsis                MemoryError          SystemExit        #
#        EnvironmentError        NameError            TypeError         #
#        Exception               None                 ValueError        #
#        FloatingPointError      NotImplementedError  ZeroDivisionError #
#        IOError                 OSError                                #
#        ImportError             OverflowError                          #
#                                                                       #
#########################################################################
if (__debug__): pass                   # flag for debug mode
x = __doc__                            # get documentation string
__import__("sys")                      # built in import function
if (__name__ == "__main__"): pass      # check if this is the top level module
x = abs(-5)                            # abs value
x = apply(factorial, (5,))             # apply function
x = buffer("hello")                    # create reference to buffer object
b = callable("hello")                  # flag whether object is callable x()
x = chr(0x2A)                          # convert ascii int to character
x = cmp(1, 2)                          # compare function (-1 0 or 1)
x = coerce(1, 2.0)                     # return tuple with two arguments coerced to common type
#compile("x = 1", "test.pyc", "exec")  # compile the string into a code object
x = complex(1, 2)                      # build complex number: same as (1+2j)
#delattr(Circle, "numInstances")       # delete attribute from object
x = dir()                              # list of items in the dictionary
x = dir(__builtins__)                  # list of built in objects
(x, y) = divmod(4, 3)                  # return both div and mod values in a tuple
x = eval("1 + 2")                      # evaluate string as python code
x = execfile("test.py")                # pull in external file
y = filter(lambda a: a&gt;1, [1, 2, 3])   # return list of items that meet condition
x = float(3)                           # convert to floating point
x = getattr(sys, "argv")               # fetch attribute
x = globals()                          # dictionary with current global symbol table
b = hasattr(Circle, "numInstances")    # test if object has attribute
x = hash("abc")                        # return 32 bit hash value for object
x = hex(0x7f)                          # convert int to hex string
x = id("abc")                          # return the identity of the object
#try: x = input("&gt;Prompt")             # get input from user prompt
#except: pass
x = int(3.56)                          # convert to int - truncate
x = intern("abc")                      # intern a string - for speeding up lookups
b = isinstance(x, Circle)              # test if object is instance of class or a subclass
b = issubclass(Circle, Shape)          # test if class is a subclass
x = len([1, 2, 3])                     # length of sequence
x = list([1, 2, 3])                    # convert to or copy list
x = locals()                           # dictionary with current local symbol table
x = long(3.56)                         # convert to long - truncate
y = map(lambda a: a*a, [1, 2, 3])      # apply function to each item in list
x = max([1, 2, 3])                     # max item value in sequence
x = min([1, 2, 3])                     # min item value in sequence
x = oct(077)                           # convert int to octal string
x = open("test.txt", "r")              # open file
x = ord("a")                           # convert ascii char to int
x = pow(2, 4)                          # power function
x = range(0, 5, 1)                     # make list over range with start index and increment
#try: x = raw_input("&gt;Prompt")         # get input from user prompt
#except: pass
y = reduce(lambda a,b: a+b, [1, 2, 3]) # return value by applying iteration (sum list)
reload(sys)                            # reload module code
x = repr("hello")                      # get string representation of object
x = round(1.5)                         # round value (2.0)
x = round (123.456, 2)                 # round to decimal place (123.46)
x = round (123.456, -2)                # round to digit (100.0)
setattr(Circle, "numInstances", 2)     # set attribute value
x = slice(0, 5, 1)                     # create a slice attribute in range form
x = str(1.0)                           # convert to string
x = tuple([1, 2, 3])                   # convert to tuple
x = type([1, 2, 3])                    # object type
x = vars()                             # dictionary with current local symbol table
x = vars(Circle)                       # return local symbol table of object
x = xrange(0, 5, 1)                    # same as range, but does not create a list

x = None                               # null object

try:
   raise Exception                     # root class for all exceptions
   raise StandardError                 # base class for all exceptions except SystemExit
   raise ArithmeticError               # base class for arithmetic exceptions
   raise LookupError                   # base class for key or index exceptions
   raise EnvironmentError              # base class for external exceptions

   raise AssertionError                # assert statement fails
   raise AttributeError                # attribute reference fails
   raise EOFError                      # eof condition without reading any data
   raise FloatingPointError            # floating point operation fail
   raise ImportError                   # import statement fail
   raise IndexError                    # sequence subscript out of range
   raise IOError                       # io exception
   raise KeyError                      # dictionary key not found
   raise KeyboardInterrupt             # user hit interrupt key
   raise MemoryError                   # out of memory
   raise NameError                     # local or global name not found
   raise NotImplementedError           # used for abstract classes
   raise OverflowError                 # overflow exception
   raise OSError                       # os exception
   raise RuntimeError                  # exceptions that don't fit other classes
   raise SyntaxError                   # syntax error encountered
   raise SystemError                   # internal error
   raise SystemExit                    # sys.exit command
   raise TypeError                     # built in applied to inappropriate object
   raise ValueError                    # built in has correct type but bad value
   raise ZeroDivisionError             # divide by zero error

except:
   pass

<hr>

<b>[UNSORTED]</b>

? help inspect pydoc

__dict__  dir() __class__ __bases__  

None





tmpfile()  execl(path, arg0, arg1, ...) 

    from random import Random
    g = Random(firstseed)
    result = [g]

randrange([start,] stop[, step]) 

compileFile(source) 
os.path.join(dir, file)

import sys, operator
nums = map(float, sys.argv[1:])
print reduce(operator.add, nums)/len(nums)





</PRE></BODY></HTML>


