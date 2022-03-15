# Java -- Java related info

     <<>>   WinDocs 
     <<>>      Java API 
     <<>>          JDK Docs 
     <<>>      Java Language Specification 
     <<>>    Virtual Machine Specification 

    CLASS LIB SOURCE CODE  <<>>    
    EXECUTION SPEED        <<>>?thread=122295
                           <<>>

     <<>>     
     <<>>     
     <<>>     
     <<>>     
     <<>>   Kawa -- java development environment 
     
     <<>>
     
    Making MAIN run 
     
    Oprions:Directories:JavaBin:               <<>> 
    Project:CompilerOptions:OutputDirectory:   Check & baseDir 
    Project:InterpreterOptions:ExecuteProgram: Check & baseDir 
    Options:Interpreter run in 
     
    IN FILE:  baseDir\lowerCasePath\UpperCaseClass.java 
     
    package lowerCasePath;  .... 
    public class UpperCaseClass {   ... 
    public static void main (String[] main) { ... 
     
    Note: will fail if dependent classes cannot be loaded. 
     
    === 
    INSTALL jdk1.1.6; 
    - DOWNLOAD java.sun.com  (products & APIs) (Get jdk1.1.6, docs, & jre) 
    - added <<>> to path 
    - AUTOEXEC.BAT: SET CLASSPATH=<<>> 
                    SET PATH=%path%;<<>> 
     
    INSTALL  jdk116-doc.zip (in <<>> dir) 
     
    INSTALL kawa 
    - Register  LAM  EAC 
    - Customize:Options:directories 
      set internet browser file, java bin, lib, src, docs 
    - set dirs to <<>> 
    - SET EDITOR: insert space instead of tabs ... 
     
    INSTALL jre 
     
    NOTES FOR SDK 1.1.2 
    - rename src.jar to .zip and extract 
    -  
     
