# Install-bak.WMR --

    VTDSDK --  Voice Type Dictation Software Development Kit 
    - exec uscntl.exe 
    - exec vtdt.exe 
    - Add ibmvtype/bin to path 
    - Start:prog:IBMvoice:MicrophoneSetup 
    input/output setting  (Use auto gain) 
     
     
    WATCH ME READ SOURCE SETUP 
    - unzip into \new 
    > Start MSDS 
    - New project -- Win32 App 
    - Add .rc 
    - Add all .c 
    > Proj:settings 
      C/C++: (category=preprocessor) add include  <<>> 
      link:  add  
        vfw32.lib  -- camera 
        winmm.lib  -- sound, midi 
        <<>>   -- Voice type dictation access 
     
