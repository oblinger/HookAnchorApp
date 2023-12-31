  [koans](koans/koans.md)

# === RUBY SCRIPTING ===

[TOPLEVEL]
#!/usr/bin/ruby

def main
  puts "Hello World.  Args=#{ARGV.inspect}"
  system("ls")
end
main


# This code will only execute if this file is the file
# being run from the command line.
if $0 == __FILE__

[SYSTEM]
files=`ls`.split



=== MAC RUBY ===


http://rubycocoa.sourceforge.net/

https://developer.apple.com/library/mac/#documentation/Cocoa/Conceptual/RubyPythonCocoa/Articles/UsingScriptingBridge.html



# INSTALL  (see doc mac)
  
# Oblib

# DEBUGGING
#   tee
#
# EXECUTE
#   run
#   osascript
~#
# I/O
#   get_clipboard              -- returns clipboard as a string
#   sets_clipboard(str)        -- sets the clipboard to be the string
#   send_keys(keys [,using])   -- sends 'keys' to app w. focus.  
#                                 using = "shift down, option down, control down, command down"
~#   send_key(int)              -- sends single applescript keycode  (see # -space keycode)
#                                 also http://www.codemacs.com/coding/applescript/applescript-key-codes-reference.8288271.htm

# $: << File.dirname(__FILE__)
# require 'oblib.rb'

# RUBY Scripting
  sleep 1

require File.dirname(__FILE__) + "/../../config/application"
%x{osascript -e '#{cmd}'}

$running = true
Signal.trap("TERM") do 
  $running = false
end

while($running) do

  Rails.logger.info "Daemon running at #{Time.now}.\n"

  # sanity check
  %x{touch /foo.txt}

  # this stops working after a while
  result = %x{/usr/bin/osascript -s o /path/to/the/script.scpt}

  # under normal operation, my applescript spits out a string result
  Rails.logger.info "Result: #{result}.\n"

  sleep 30
end

# BLAM RUBY CONTROL 
  1099 form
[TOPLEVEL]
#!/usr/bin/ruby
require "#{File.dirname(__FILE__)}/../lib/converter.rb"
system('ls')  x=`ls`.chomp
ARGV[0]  ENV['X']='a'; y=`echo $X`.chomp
STDIN.gets.chomp; sleep(3)   printf('.'); STDOUT.flush;

str=IO.read(file)

[TOPLEVEL] $ irb \\  if __FILE__ == $0 \\ (main code ARGV/ENV/ARGF) # Comment 
  assert_equal x,y,msg  assert expr, fail_message
  require File.dirname(__FILE__) + '/../helper'
  pred?  destruct!  o = self.new   puts ENV['PATH']
  $ irb -c(check) -d(debug) -e(oneline) -h(help) -n(gets loop) -rL(require L) -v(ervose) -w(arn) -y(comp debug)
[SPECIAL]  defined?(somevar)
[CONTROL]  x==1 ? 'y' : 'n'  if x; return end
  if/unless tweets.size!=0 || t.nil?\\..\\ elsif t.size()==3 \\..\\ else \\ end  # comment
  while true; break; next; end
  case $age; when 0..2: \..\ when 5,8: \..\ else \..\ end
  single_stmt if true
  list.each do |var| \\ body \\ end    --OR--  
  (5..10).each{|v| break\\ next\\ redo\\ retry ...} --OR--  for (i=0; i<list.size; i++) {|v| ... }
  begin \..\ rescue Exception=>e \..\ ... else \..\ ensure \..\ end    \\ throw :some_label
  begin \..\ rescue=>e; e.backtrace.join("\n"); \..\ end
  raise [Exception,] 'msg'
  catch (:some_tag) do \..\ throw :some_tag \..\ end
  def \..\ ensure \..\ end
[FILE]
  File.exists?() .size()
  File.open('/Users/oblinger/.bash_profile', 'rb') {|f| str=f.read()}
  File.open('/Users/oblinger/zzz', 'a') {|f| f.write('line')}


[FILE]  http://www.techotopia.com/index.php/Ruby_Directory_Handling
  string = File.open(TMP_FILE, "rb") { |f| f.read }  # read binary
 File.open(local_filename, 'w+') {|f| f.write(doc) }
 File.exists?(path) && File.directory?(path)
 File.basename(f,File.extname(f))
 list = Dir.entries("/ob")  Dir.foreach(path) {|f| puts f}


Dir.pwd   Dir.chdir("/home/ruby/test")   Dir.mkdir

-----
  fn 1, 2, three:3, four:4  # Last arg is a hash
  l.each {|var| block1 } do\..\while()  do\..\until()   do stmt1, stmt2 end
  def wrap &b \\ print "santa says " \\ 3.times(&b) \\ end \\ wrap {print "Ho "}
    self.is_global? || self.user_id == user.id
  def x(&block) \\ yield(1,2) \\ end

def one (&block) two(&block); end
def two (&block) yield end

fn = lambda {|x| x+x }
fn = lambda {|x| foo(x) }
fn = ->(x) {x+x}
def foo (x) x+3; end
fn.call(5)

def addem (*args) args.sum; end

a=[1,2]
addem(4, *a, 5)

VIEWS -- helper modules, model classes,  

modules use  include
Class   use  <  

private /../ /../ 

close # xx  closes # xx fix # xx fixes # xx




my_method({:a => 1, :b => 2}, "a string")
[1/2/12 MJan 2 1 :32:04 PM] Andrés Mejía: my_method("string", :a => 1, :b => 2)
[1/2/12 MJan 2 1 :32:36 PM] Andrés Mejía: my_method("string", :a => 1, :b => 2) do |parameter|
    puts parameter
end
[1/2/12 MJan 2 1 :33:00 PM] Andrés Mejía: my_method("string", :a => 1, :b => 2) { |parameter| puts parameter }

# File actionpack/lib/action_view/helpers/url_helper.rb, line 231
      def link_to(*args, &block)
        if block_given?
          options      = args.first || {}
          html_options = args.second
          link_to(capture(&block), options, html_options)
        else
          name         = args[0]
          options      = args[1] || {}
          html_options = args[2]

[BLOCKS]
def fn(&block); block.call(10); end

block_given?
lambda {|x|x+1}.call(10)

[CONDITIONALS]
x.is_a? Array

------------


DOCS:
http://www.zenspider.com/Languages/Ruby/QuickRef.html                                      Language
http://ruby.doc.org/   module-kernel  Core, Std-lim, 

set env for sub process?
change dir in currnet process
get input from console (user)
explicitly control the ARGV of a sub process
pipe into a subprocess

# BLAM RUBY DS STRING

.include?(s) .index(s/r,off) 
.each(...) .each_line(...) do

.intern  .to_sym  
   split()   s[start,len]  length(s)  s[3..-1]

[STRING]  "this\n"+"that" == %{this\\that} \\ puts "%s, %s!" % [ "Hello", "Matz" ] 
  s[#,#/rng/regex] upcase! center(#,s) chomp!(s) count delete!([s]) downcase! include?(s) index(s/rgx,#)
  insert(#,s) length ljust(#,s) rindex(s/rgx,#) rjust(#,s) split(/:/) strip! swapcase! to_sym  tr!(s,s) upcase!
  join(' ') strip
  each (["]) do |s| ..end   each_line (["]) do |s| ..end   gsub! (rgx) do |match| ..end
  scan(rgx) do |match| ..end  sub! (rgx) do |match| ..end 
  a==b [content equality]  a.eql?(b)   a.equal?(b) [object equality]

[INSPECT]     .count 
[PREDICATES]  .include? .end_with? .start_with?
[EDIT]        .gsub("Hello", "Bye")  .delete!
[TYPOGRAPHIC] .capatalize! .[up|down]case!,  .[r|l|]strip!  .center .[r|l]just
              .downcase[!]  .upcase[!]  .chomp!  .chop!
[COMBINE]     .concat
[CONVERT]     .to_sym .crypt
[STRING]
x    ::new
x    ::try_convert, %, *, +, <<, <=>, ==, ===, =~, [], []=, ascii_o, bytes
-   bytesize, capitalize, capitalize!, casecmp, center, chars, ch, chomp!
-   chop, chop!, chr, clear, codepoints, concat, count, crypt, ??? de, delete!
    downcase, downcase!, dump, each_byte, each_char, each_codepoint, each_line, empty?, 
    e, encode!, encoding, end_with?, eql?, force_encoding, getbyte, gsub, gsub!, hash, 
    hex, include?, index, initialize_copy, insert, inspect, intern, length, lines,
    ljust, lstrip, lstrip!, matc, next, next!, oct, ord, partition, replace, reverse, 
    reverse!, rindex, rjust, rpartition, rs, rstrip!, scan, setbyte, size, slice, 
    slice!, split, squeeze, squeeze!, start_with?, strip, str, sub, sub!, succ, 
    succ!, sum, swapcase, swapcase!, to_c, to_f, to_i, to_r, to_s, to_str, to, tr
    tr!, tr_s, tr_s!, unpack, upcase, upcase!, upto, valid_encoding?

[STRING]  .to_s() 

[REGEX]
   "this is the test" =~ /\s(\w*).*/   puts $1

[REGEX]
  first=str.match(/name: ([^,]*),/)[1]
  puts "ok" if ("Hello"=~/^[A-Z][a-z]+$/)
  .gsub(/OLD/,'NEW')

list = text.scan(/\((\w+)\):(\d+)/).collect{|x| x.join}
list=[]
text.scan(/\((\w+)\):(\d+)/) {
  list << $1+$2
}

"The {Quick} Brown {Fox}".gsub(/\{(\w+)\}/) {|s| s[1..-2].reverse }  # Dynamic action on each match


if v =~ /regex/     #  \d digit \s space \w word (alphanum)


.gsub(
FMT s.capitalize! .center(#,s) .chomp! .delete! .downcase! .insert .length 
OTHER  .count
.sub!(rgx) do |.swapcase! .to_sym .upcase!
a.select{ |n| n>2 }

# BLAM RUBY DATASTRUCTURES
http://ruby-doc.org/docs/ProgrammingRuby/html/builtins.html
http://langref.org/all-languages/pattern-matching


HASH    m={ :one=>"one", :two=>"2" } == {one: "one", two: "2"}         http://www.ruby-doc.org/core-1.9.2/Hash.html
  size()  has_key?(k)  has_value?(v)  keys()  values() 

           puts b[:stat] == puts b.stat
[-->hash]  dup  under.merge!(over)  select{|k,v| blk} → a_hash
[-->array] 


ARRAY        index from 0, neg values from end
[-->val]     a.at(x)==a[x]  [x]=  fetch   key  first  last 
[*-->Arr]    [1,2,3]  new(size,ele)  to_a  to_ary 
             flatten  keys   values  values_at(k1,...)
[Arr-->Arr]  clear            merge!=update  select  slice!  sort!  concat
[Array!]     delete  delete_if!==reject!  keep_if  uniq 
             replace  reverse!  fill  pop   push
             pop  push  shift  unshift

[-->loop]  collect!==map!, compact!, each, each_key, each_value, each_pair, each_index reverse_each
           find_all==select, delete, delete_at, delete_if
[-->bool]  empty?  eql?  value?=has_value? key?=has_key?=include?=member?
[-->int]   index, rindex  length==size  nitems
[-->str]   join
[INFO]     compare_by_identity?  default=  default_proc=  length=size
[pair]     rassoc  shift   to_a
[OPS]      &=intersect  *=repeat  +=concat  --=setDiff  <<=append
           <=>compare  |=union  
[ref]      a[0]  range[4..7]  a[start,len]=["replacement"]

assoc=search_array_of_array  rassoc
indexes==indicies
pack
inject


[MAP]
[modify]  rehash  store=hsh[key]=
[-->hash] .clone merge!(over) compare_by_identity  .symbolize_keys! .compact!  replace  invert
[] each_pair, each_key, each_value

[MAP]   http://www.ruby-doc.org/core-1.9.2/Hash.html
 ::[], ::new, ::try_convert, ==, [], []=, assoc, clear, compare_by_identity,
 compare_by_identity?, default, default=, default_proc, default_proc=, 
delete, delete_if, each, each_key, each_pair, each_value, empty?, eql?, 
fetch, flatten, has_key?, has_value?, hash, include?, initialize_copy, 
inspect, invert, keep_if, key, key?, keys, length, member?, merge, merge!, 
rassoc, rehash, reject, reject!, replace, select, select!, shift, size, 
store, to_a, to_hash, to_s, update, value?, values, values_at



[ARRAY]  a=["one",2,3] \\ x,y=a \\ a=a[0,-1] \\ a << 'push val' \\   %w{ one two } == ["one", "two"}
   .each {|ele| block} ->    .each do |p| \..\ end
   .length .size .at .first .last
   .pop, .push(), .shift .unshift() .slice() .delete_if, .keep_if, 
   sorted = Dir["*"].sort_by {|f| test(?M, f)}  copy = Array.new(some_array)
[ARRAY-GRP] |, clear, compact!, concat

[ARRAY-REDUCE] at, count, 


[ARRAY-]
 ::[], ::new, ::try_convert, &, *, +, -, <<, <=>, ==, [], []=, assoc, at, clear, collect, collect!, combination, 
compact, compact!, concat, count, cycle, delete, delete_at, delete_if, drop, drop_while, each, each_index, empty?, 
eql?, fetch, fill, find_index, first, flatten, flatten!, frozen?, hash, include?, index, initialize_copy, insert, 
inspect, join, keep_if, last, length, map, map!, pack, permutation, pop, product, push, rassoc, reject, reject!, 
repeated_combination, repeated_permutation, replace, reverse, reverse!, reverse_each, rindex, rotate, rotate!, 
sample, select, select!, shift, shuffle, shuffle!, size, slice, slice!, sort, sort!, sort_by!, take, take_while, 
to_a, to_ary, to_s, transpose, uniq, uniq!, unshift, values_at, zip, |
 Dir["*"].collect { |f|
   [test(?M, f), f]
}.sort.collect { |f| f[1] }


[RANGE]  r=(1,2,3) \\ r.to_a \\ r==(1...4)

[LIST] l.join(", ")

# BLAM RUBY IO
File.expand_path("..", "/ob/bin/nstr") # parent folder

# RUBY Topics
## Modules & Classes
require "u"     # before module stmt.  
module ModuleName
  class ClassName
end
## Predefined vars
         __FILE__ (current file constant)
         __LINE__ (current line constant)
$!	$ERROR_INFO[1]	The exception information message set by the last 'raise' (last exception thrown).
$@	$ERROR_POSITION[1]	Array of the backtrace of the last exception thrown.
$&	$MATCH[1]	The string matched by the last successful pattern match in this scope.
$`	$PREMATCH[1]	The string to the left of the last successful match.
$'	$POSTMATCH[1]	The string to the right of the last successful match.
$+	$LAST_PAREN_MATCH[1]	The last bracket matched by the last successful match.
$1 to $9		The Nth group of the last successful regexp match.
$~	$LAST_MATCH_INFO[1]	The information about the last match in the current scope.
$=	$IGNORECASE[1]	The flag for case insensitive, nil by default (deprecated).
$/	$INPUT_RECORD_SEPARATOR[1], $RS[1] or $-0	The input record separator, newline by default.
$\	$OUTPUT_RECORD_SEPARATOR[1] or $ORS[1]	The output record separator for the print and IO#write. Default is nil.
$,	$OUTPUT_FIELD_SEPARATOR[1] or $OFS[1]	The output field separator for the print and Array#join.
$;	$FIELD_SEPARATOR[1], $FS[1] or $-F	The default separator for String#split.
$.	$INPUT_LINE_NUMBER[1] or $NR[1]	The current input line number of the last file that was read.
$<	$DEFAULT_INPUT[1]	An object that provides access to the concatenation of the contents of all the files given as command-line arguments, or $stdin (in the case where there are no arguments). Read only.
$FILENAME		Current input file from $<. Same as $<.filename.
$>	$DEFAULT_OUTPUT[1]	The destination of output for Kernel.print and Kernel.printf. The default value is $stdout.
$_	$LAST_READ_LINE[1]	The last input line of string by gets or readline.
$0		Contains the name of the script being executed. May be assignable.
$*	$ARGV[1]	Command line arguments given for the script. Also known as ARGV
$$	$PROCESS_ID[1], $PID[1] or Process.pid	The process number of the Ruby running this script.
$?	$CHILD_STATUS[1]	The status of the last executed child process.
$:	$LOAD_PATH	Load path for scripts and binary modules by load or require.
$"	$LOADED_FEATURES or $-I	The array contains the module names loaded by require.
$stderr		The current standard error output.
$stdin		The current standard input.
$stdout		The current standard output.
$-d	$DEBUG	The status of the -d switch. Assignable.
$-K	$KCODE	Character encoding of the source code.
$-v	$VERBOSE	The verbose flag, which is set by the -v switch.
$-a		True if option -a ("autosplit" mode) is set. Read-only variable.
$-i		If in-place-edit mode is set, this variable holds the extension, otherwise nil.
$-l		True if option -l is set ("line-ending processing" is on). Read-only variable.
$-p		True if option -p is set ("loop" mode is on). Read-only variable.
$-w		True if option -w is set.''

# BLAM PREDIATES
instance_of? --> is_a? 
defined?(expr)

# BLAM RUBY OBJECTS
[OBJECT] .class .freeze .frozen? .inspect .is_a?(ClassName) .methods .responds_to(sym) .to_s
[CLASS/VAR]  class ClassName < ModuleName::SuperClassName  \\ o=ClassName.new
  attr_accessor :my_attr_name \\ attr_reader, attr_writer, attr_protected
  CONST_VAR = {'k' => 'v', 'k2'=>'v2' }
  protected private
  def method_name(x, y="4", keys={}) \\ local_var=1 \\ CONST_VAR = 0
     @instance_var=-self.instance_var \\ @@class_var \\ $global_var end
  -stmt-> attr_accessor :field
  module ModuleName //..// end  # in file /lib/module_name.rb
  include ModuleName  # in another class
  

class ClName < ActiveRecord::Migration \\  def change \\ create_table :p do |t|
  t.string :title \\ t.decimal :price, precision:8, scale:2
  def self.static_method 

def ClassName.my_static_method \\..\\  # Class Method

[METHOD]
def fn_name(a, b=5, &c) \\..\\ end \\ protected :fn_name
private/  :fn_name  \\ private \\
def devise(*array) array end 
  include Devise::Models::Authenticatable

[CLASS] class MyClass < ActiveRecord::base \\ end
def h(n) \\ puts "hello #{n}" \\ end  \\ h \\ h()
- PARTS  .instance_variables  .instance_variable_get  
- CONSTRUCTORS  .new()  .create()  .initialize() # usr defined


[ACCESSORS] .instance_variable_get("@name")
attr_readable/attr_accessor  :makes_me_read_writable

[VARS]

