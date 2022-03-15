# Ansi --

          ESC  [  n  A  moves the cursor up n lines 
          ESC  [  n  B  moves the cursor down n lines 
          ESC  [  n  C  moves the cursor n columns forward without 
              changing lines 
          ESC  [  n  D  moves the cursor n columns backward without 
              changing lines 
          ESC  [  s  save cursor position 
          ESC  [  u  restore previously saved cursor position 
          ESC  [  2  J  erase the display 
          ESC  [  K  erase the current line 
          ESC  [  n ; ... ; n  m  set the screen attributes 
              0  all attributes off 
              1  bold on 
              2  dim on 
              3  italic on 
              5  blink on 
              6  rapid blink on 
              7  reverse video on 
              8  concealed on 
              30  black foreground 
              31  red foreground 
              32  green foreground 
              33  yellow foreground 
              34  blue foreground 
              35  magenta foreground 
              36  cyan foreground 
              37  white foreground 
              40  black background 
              41  red background 
              42  green background 
              43  yellow background 
              44  blue background 
              45  magenta background 
              46  cyan background 
              47  white background 
              48  subscript background 
              49  superscript background 
     
     
     
     
    ANSI PLUS  (whatever that is) 
     
    Escape Seq. Description and Link  
         
    Display mode and character set  
    Esc [#;#h Set video mode  
    Esc [#;#l Reset video mode  
    Esc [!#;#t Select character set height  
    Esc [!#;... #e Select graphics mode text treatment options  
         
    Cursor positioning  
    Esc [!#v  Select video page  
    Esc [#U Select next page  
    Esc [#V  Select preceding page  
    Esc [#;#H Set cursor position  
    Esc [#;#f Set cursor position  
    Esc [#A Move cursor up  
    Esc [#B Move cursor down  
    Esc [#C Move cursor right  
    Esc [#D Move cursor left  
    Esc [#E  Move cursor to next line  
    Esc [#F  Move cursor to preceding line  
    Esc [#G  Move cursor to absolute column in same line  
    Esc [#a  Move cursor to absolute line in same column  
    Esc [#;#;#g Reset tabs  
    Esc [#;... #W Tab control  
    Esc [#I Horizontal tab  
    Esc [#Y Vertical tab  
    Esc [#Z Backward tab  
    Esc [#;#s Save cursor position or other driver context  
    Esc [#;#u Restore cursor position or other driver context  
         
    Color selection and definition  
    Esc [!#;#;#;#;#c Define colors, select palette, setup blinking  
    Esc [#;... #m Select current color attributes for output  
     
        
    Erase, insert, delete and fill  
    Esc [#;#J Clear the current screen page  
    Esc [#;#;#K Clear the current line  
    Esc [#;#X  Erase characters on the current line  
    Esc [#;#L Insert lines on the screen  
    Esc [!#;#L Insert lines from scroll-back  
    Esc [#;#M  Delete lines from the screen  
    Esc [!#;#M Delete lines and add to scroll-back  
    Esc [#;#@ Insert characters on a line  
    Esc [#;#P Delete characters from a line  
    Esc [#b Repeat last output character  
         
    Scrolling  
    Esc [#;#;#;#;#S Scroll up  
    Esc [!#;#;#;#;#S Scroll up and add to scroll-back  
    Esc [#;#;#;#;#T Scroll down  
    Esc [!#;#;#;#;#T Scroll down from scroll-back  
    Esc [#;#;#;#;# @ Scroll left (Note: a space must precede @)  
    Esc [#;#;#;#;# A Scroll right (Note: a space must precede A)  
    Esc [#;#;#;#r Set scrolling region  
         
    Driver status and control  
    Esc [#;#n Query ANSIPLUS status  
    Esc [!#;... #d Enable/disable ANSIPLUS driver features  
    Esc [!#;... #g Define Ctrl+G beep tone  
    Esc [#;... #p Define key reassignment  
    Esc [!#;... #k Add keys to keyboard buffer  
    Esc [!#w Wait for specified time to elapse  
     
