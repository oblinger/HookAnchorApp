# Perl.String --

      length(), substr(), split(), chop()  (operators eq,ne,~=,~!) 
      $a = $b . " " x 30;   # Concat 30 blanks 
     
    $info = "Caine:Michael:Actor:14, Leafy Drive";  # split at : 
    @personal = split(<<>>, $info 
    $_ = "Capes:Geoff::Shot putter:::Big Avenue";   # split at : group 
    @personal = split(<<>>);  # split at the ':+' combo 
     
    substr("Once upon a time", 3, 4);   # returns "e up" 
    substr("Once upon a time", 7);      # returns "on a time" 
    substr("Once upon a time", -6, 5);  # returns "a tim" 
     
     
     
     
     
