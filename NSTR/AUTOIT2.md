# AUTOIT2 --


    ; Comment 
    SetEnv,title1,%title% 
    goto,Someplace 
    SomePlace: 
     
    Run,<<>> files\\winzip\\winzip32.exe %1% 
    WinWaitActive,WinZip,StatusBar 
    Send,First Arg %1%<<>> 
     
    IFEqual,var,value,cmd,arg1,... 
    IfNotInString,title,WinZip,Goto,WaitForFinish 
    IfEqual,title,WinZip,Goto,end 
    Sleep,2000 
     
    MsgBox,0,ERROR: Winzip no longer active, 
     
     
     
     
    AdlibOn 
    AdlibOff 
    BlockInput,  
    Break,  
    DetectHiddenText,  
    Exit 
    EndRepeat 
    Gosub,  
    EnvAdd, ,  
    EnvSub, ,  
    FileAppend, ,  
    FileReadLine, , ,  
    Goto,  
    HideAutoItWin,  
    IfInString, , ,  
    IfNotInString, , ,  
    IfWinExist, , [],  
    IfWinNotExist, , [],  
    IfWinActive, , [],  
    IfWinNotActive, , [],  
    IfEqual, , ,  
    IfNotEqual, , ,  
    IfGreater, , ,  
    IfGreaterOrEqual, , ,  
    IfLess, , ,  
    IfLessOrEqual, , ,  
    IfExist, ,  
    IfNotExist, ,  
    IfMsgBox, ,  
    IniRead, , , ,  
    IniWrite, , , ,  
    IniDelete, , ,  
    InputBox, , ,  
    LeftClick, ,  
    LeftClickDrag, , , ,  
    MouseGetPos, ,  
    MouseMove, ,  
    MsgBox, , ,  
    Random, , ,  
    RegRead, , , , ,  
    RegWrite, , , , ,  
    RegDelete, , ,  
    Repeat,  
    Return 
    RightClick, ,  
    RightClickDrag, , , ,  
    Run,  [,] 
    RunWait,  [,] 
    Send,  
    SetEnv, ,  
    SetKeyDelay,  
    SetTitleMatchMode,  
    SetWinDelay,  
    Shutdown,  
    Sleep,  
    SplashTextOn, , , ,  
    SplashTextOff 
    StringCaseSense,  
    StringLeft, , ,  
    StringRight, , ,  
    StringMid, , , ,  
    StringLen, ,  
    StringReplace,  , , ,  
    StringTrimLeft, , ,  
    StringTrimRight, , ,  
    WinGetActiveTitle,  
    WinKill,  [,] 
    WinWait,  [,] 
    WinWaitClose,  [,] 
    WinWaitActive,  [,] 
    WinWaitNotActive,  [,] 
    WinHide,  [,] 
    WinRestore,  [,] 
    WinShow,  [,] 
    WinMinimize,  [,] 
    WinMaximize,  [,] 
    WinActivate,  [,] 
    WinClose,  [,] 
    WinMinimizeAll 
    WinMove, , [], , , ,  
    WinSetTitle, , [],  
     
     
     
     
     
     
     
     
    AdlibOn 
    AdlibOff 
    BlockInput 
    Break 
    DetectHiddenText 
    Exit 
    EndRepeat 
    EnvAdd 
    EnvSub 
    FileAppend 
    FileReadLine 
    Gosub 
    Goto 
    HideAutoItWin 
    IfInString 
    IfNotInString 
    IfWinExist 
    IfWinNotExist 
    IfWinActive 
    IfWinNotActive 
    IfEqual 
    IfNotEqual 
    IfGreater 
    IfGreaterOrEqual 
    IfLess 
    IfLessOrEqual 
    IfExist 
    IfNotExist 
    IfMsgBox 
    IniRead 
    IniWrite 
    IniDelete 
    InputBox 
    LeftClick 
    LeftClickDrag 
    MouseGetPos 
    MouseMove 
    MsgBox 
    Random 
    RegRead 
    RegWrite 
    RegDelete 
    Return 
    RightClick 
    RightClickDrag 
    Run 
    RunWait 
    Repeat 
    Send 
    SetEnv 
    SetKeyDelay 
    SetTitleMatchMode 
    SetWinDelay 
    Shutdown 
    Sleep 
    SplashTextOn 
    SplashTextOff 
    StringCaseSense 
    StringLeft 
    StringRight 
    StringMid 
    StringLen 
    StringReplace 
    StringTrimLeft 
    StringTrimRight 
    WinGetActiveTitle 
    WinKill 
    WinWait 
    WinWaitClose 
    WinWaitActive 
    WinWaitNotActive 
    WinHide 
    WinRestore 
    WinShow 
    WinMinimize 
    WinMaximize 
    WinActivate 
    WinClose 
    WinMinimizeAll 
    WinMove 
    WinSetTitle 
     
     
     
