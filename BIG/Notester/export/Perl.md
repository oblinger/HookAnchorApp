# Perl --

     <<>>     
     <<>>   *note builtin:: *note vars:: *note parts:: 
     <<>>   
     <<>>        Perl 5 Docs 
    #!<<>> 
    #!<<>> 
     
    CONTROL  if ($x == $y**2) <<>> elsif <<>> else <<>> 
      for ($i=0; $i<10; ++$i)<<>>;  foreach $l (@lines)<<>>; for $idx (0 .. $#lines)<<>>; 
      sub max <<>> else <<>> } 
      $biggest = &max(37, 24); 
    OPERATORS   ==,!=,<,..., &&, ||, !,   (eq,ne,~=,~! for string) 
    SYNTAX     ($_, @_ implied argments) 
      $scalarVar+= ++$<<>>**2; @listVar=;  # comment 
      $var=`echo evalled cmd`; print("$var '$constant' "); 
    LIST @list; $first=$list[0]; $lastIdx=$#list; $len=@list; 
      @food  = ("apples", "pears");  @food=(); 
      push(@food, "eggs", "lard");  push(@food, @morefood); pop(@food); 
      $f = "@food";  # elements w. spaces 
      ($a, @somefood) = @food; 
    ARRAY $first=$array[0]; $lastIdx=$#array;  $len=@array; if ( @list==() )<<>>  
    @array[3..5] ~ @array[3,4,5] 
    HASH  %sex=("fred", "male", "kim", "female"); $sex<<>>; 
      delete $HASH<<>>;   exists ... defined ... 
      @keys = keys %ENV; @values = values %ENV; 
      while (($key,$value) = each %ENV) <<>> 
     <<>>    length(), substr(), split(), chop()  (operators eq,ne,~=,~!) 
      $a = $b . " " x 30;   # Concat 30 blanks 
     <<>>    if ($string !~ <<>> text/) <<>> 
      $sentence =~ s/subst old/new/gi      # [G]lobal all subst; [I]gnore case 
      s/([A-Z])<<>>;     # the <<>> (or $1) xlates into each matched term 
    IO     open(OUT, ">outfile.dat"); print "input: ";  chomp($var=); 
      open(IN, ";    chop @lines;  close(IN); 
      print("expr"); print OUT "This line goes to the file.\n" 
    SYSTEM  chdir(".."); system('<<>>',"Args @ARGV EnvVar $ENV<<>>\n"); 
    $ARGV[0] fist arg 
     <<>>     
     if ("$ARGV[0]" eq "")    <<>> 
     <<>>   use Getopt::Std;  getopts('ab:'); 
     <<>>     
     
     
    <<>> read file contents 
    $file = join('', );                  # better 
     
    sleep(10);   # SLEEP n seconds 
    ($base,$ext)=($file=~<<>>^(\S+)\.(.*)<<>>);  # FILE NAME SPLIT   tested??? 
     
     
     
     
    days               # the simple scalar value "days" 
        $days[28]           # the 29th element of array @days 
        $days<<>>        # the 'Feb' value from hash %days 
        $#days              # the last index of array @days 
     
    but entire arrays or array slices are denoted by '@', which works much like the word ``these'' or ``those'':  
     
     
        @days               # ($days[0], $days[1],... $days[n]) 
        @days[3,4,5]        # same as @days[3..5] 
        @days<<>>      # same as ($days<<>>,$days<<>>) 
     
    and entire hashes are denoted by '%':  
     
     
        %days               # (key1, val1, key2, val2 ...) 
     
     
