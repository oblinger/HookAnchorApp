# Perl.ARGS --

    use Getopt::Std; 
    getopts('ab:'); 
      CMD LINE:                ==>  $opt_a == 0, $opt_b eq "" 
      CMD LINE: -a -b foo      ==>  $opt_a == 1, $opt_b eq "foo" 
      CMD LINE: -b -a ignored  ==>  $opt_a == 1, $opt_b eq "" 
     
     
    - @ARGV  array of args 
    - $ARG[0] 
     
     
    use Getopt::Std; # Sets $opt_o and $opt_i ==1 or =="" 
    getopts('oif:'); # and sets $opt_f to the string arg or =="" 
     
    SIMPLE ARG PROCESSING 
    use Getopt::Std; 
    getopt('oDI');    # -o, -D & -I take arg.  Sets opt_* as a side effect. 
    getopt('oDI', \%opts);    # -o, -D & -I take arg.  Values in %opts 
    getopts('oif:');  # -o & -i are boolean flags, -f takes an argument 
                      # Sets opt_* as a side effect. 
    getopts('oif:', \%opts);  # options as above. Values in %opts 
     
     
    The getopt() functions processes single-character switches with switch 
    clustering. Pass one argument which is a string containing all 
    switches that take an argument. For each switch found, sets $opt_x 
    (where x is the switch name) to the value of the argument, or 1 if no 
    argument. Switches which take an argument don't care whether there is 
    a space between the switch and the argument. 
     
    Note that, if your code is running under the recommended use strict 
    'vars' pragma, it may be helpful to declare these package variables 
    via use vars perhaps something like this: 
     
        use vars qw<<>> $opt_foo $opt_bar <<>>; 
     
    For those of you who don't like additional variables being created, 
    getopt() and getopts() will also accept a hash reference as an 
    optional second argument. Hash keys will be x (where x is the switch 
    name) with key values the value of the argument or 1 if no argument is 
    specified. 
     
     
     
     
     
