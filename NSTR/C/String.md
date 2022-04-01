# C.String -- String manipulation functions

    All are in  or  
     
    SPRINTF 
       int sprintf( char *buffer, char *format [, argument] ... ); 
       int vsprintf( char *buffer, char *format, va_list argptr );  
         Returns num bytes put in buffer not counting terminator. 
     
    CONCATENATE 
       char *strcat( char *strDestination, char *strSource ); 
       char *strncat( char *strDest, char *strSource, size_t count ); 
         Returns 'dest' or null. 
     
    DUPLICATE 
       char *_strdup( char *string ); 
         Returns ptr to fresh copy of 'string' 
     
    FIND CHAR 
       char *strchr( char *string, int c ); 
       char *strrchr( char *string, int c );              <<>> Reverse 
         Returns ptr to char or null. 
     
    COMPARE 
       int strcmp(char *str1, char *str2 ); 
       int _stricmp( char *str1, char *str2 );            <<>> Ignore case 
       int _strnicmp(char *str1, char *str2, int cnt );   <<>> len=n; Ignore case 
         Returns - 0 + if string1 < = > string2 
     
    COPY 
       char *strcpy( char *dest, char *string ); 
       char *strncpy( char *dest, char *string, int count ); 
                    <<>> may not append null terminator. 
       void *memmove( void *dest, void *src, int count );         
                    <<>> Only one to that allows 'src' & 'dest' to overlap 
       void *_memccpy( void *dest, void *src, int ch, int cnt ); 
                    <<>> Copy until 'ch' copied or 'cnt'.  Return null if no 'ch' 
         Copies string.  Returns dest or null 
     
    CHANGE CASE 
       char *_strlwr( char *string );      <<>>    Convert to lowercase. 
       char *_strupr( char *string );      <<>>    Converts to uppercase in place 
         Returns 'string' or null 
     
    LENGTH 
       int strlen( char *string ); 
         Returns the lengh of 'string' 
     
    FIND TERMINATOR 
       int strspn(char *string, char *charSet );       <<>> FIRST IN SET 
       int strcspn( char *string, char *strCharSet );  <<>> First NOT IN SET 
       char *strpbrk( const char *string, const char *strCharSet );  
         Returns index of first char in 'string' that is in/not-in 'charSet' 
     
    REVERSE 
       char *_strrev( char *string ); 
         Reverses a string. 
     
    SET 
       char *_strset( char *string, int c ); 
       char *_strnset( char *string, int c, size_t count ); 
         Sets all chars in 'string' to 'c'. 
         Returns 'string'. 
     
    TOKENIZE 
       char *strtok( char *string, char *delimSet ); 
         Returns ptr to next token in 'string' or null. 
         Copies null char over next delimiter char in 'string' 
     
    SEARCH 
       char *strstr(char *string, char* pattern ); 
         Returns ptr to 'pattern' in 'string' or null 
     
