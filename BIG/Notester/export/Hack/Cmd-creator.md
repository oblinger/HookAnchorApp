# Hack.Cmd-creator --

    could be all done with this stuff..... 
     
     
     
    >PROPOSED 2/18/01 
    - cmd in comments of any file 
    - CmdSpec install/uninstall used to control. 
    .pl  ##-- 
    .bat ::--  
    .aut ;;-- 
    - create .lnk from a template.lnk file by replacing bytes after $$$ marker 
     
     
     
    > PROPOSED: 2/13/00 
    - cmd is defined in its own .spec file 
    - ec will edit spec entries, and will expand them on save 
      cbin entries are built directly (this works fine) 
      ? how to do a rebuild of the tree entries  (use identifiable entry & strip?) 
        could use cmd-spec --install foo.spec   &  cmd-spec --uninstall foo.spec 
     
     
    <<>> 
    Example 
    CmdSpec 
    -n emacs 
    -t Runs the GNU emacs editor 
    -g Editors 
    -m S/G 
    -m A/E/G 
    -r <<>>  ; Generates a raw entry 
    -x                            ; could have generated non raw entry 
    -auto                         ; Rest of file is an autoit script 
    -end 
    ; Always generates a cbin entry to either the -x, -auto, -pl, etc. 
     
    Each command has the following keys 
    -n name 
    -t tree-path        ; Tree path and tree name of command 
    -x executable-name  ; Exec line allows shell var expansion & args 
    -c cbin entry       ; exe | perl cmd line 
     
     
     
    <<>> 
     
    Goal, standard obj for launching 
     
     
    Features         Method=    pbang.exe   .lnk   start * 
    Speed 
     from CmdLine               =?          =? 
    Size                        -  36K      +800 
    Recursive                   +           -? 
     
     
     
     
