# Install-bak.Printer2 --

    DEFAULT:        usyktl4c 
    TRANSPARENCIES: usyktlc3 
     
    - USE PSM 
    - In notes use the common print solution to create printer objects 
    -  
    Printer 
     
    PSM 
     
    NOTE: when the printer object is renamed its assocaition to the 
          printer is broken.   
          - Start:Settings:Printers: your printer :properties:general 
            - Print test page (from properties) 
              From "Job Properties" window click find 
                - select printer 
     
    Print server yktspl01 
     
    NOTE: If it keeps forgetting the printer's location then 
    add this line in the correct spot in:  <<>> 
    ignorelastdrive=1 
     
     
    NOTE: See (install)apps for transparency setup 
