# Install-bak.Printers --

    8/10 -- Vas-a-Vie installed new printer support.  No longer use NI print below 
     
    PRINTER SUPPORT 
    - exec install from winapps\niprint 
    - foreach printer get queue name and servername 
        (can lookup on web watson I/S search > Watson key=nickname) 
    > Start:prog:NIPrint:NIPrint32 
      CLICK Configuration:general settings:enable remote print server 
      > Configuration:Remote Print Configuration\ 
        > Foreach printer AddPrinter 
          - Printername:   
          - hostname:      
          - username:     oblinger 
    > MyComputer:Printers:AddPrinter 
     
    LOCAL PRINTER INFO 
    lex = Lexmark Optra PS/2 
     
    Nick    Qname    Host     Driver 
    30-1b   w301scad yktprs02 lex 
    25-1a   w251scpc yktprs01 tex340  (Color) 
    25-1d   w251simp yktprs01 lex 
    24-2a   w242scps yktprs21 lex 
    24-2b   w242scad yktprs01 lex 
     
    30-1a?? 
    25-1?   w251scac yktprs01 tex34025-1?   w151scad yktprs01 4049 
     
     
