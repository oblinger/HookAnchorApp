# Perl.FILE --

    # PARSE PATH  (dir='./'or ends in slash. $ext='' or '.????'.   MUST use "'") 
    use File::Basename; ($base,$dir,$ext)=fileparse($full, '\..*');   
     
     
    # FILE EXISTS 
        if (!stat("$file")) <<>> 
     
     
