# Install-bak.Log -- Initial setups performed on each machine

    SOURCES:  <<>>, <<>>, <<>>, <<>>, <<>>,  
              <<>> Installer, <<>> Install Dir 
     
    <<>> 
    OBLIO T21 WIN-XP (Build II; 10/11/02) 
    > BASE SYSTEM 
     -  Fdisk:CreatePartition   
          8G;Fat32 (XP) + 8G; (Linux) + 4G (Other) + Rest;Vfat (<<>>) 
     C WINDOWS XP (Typical options) 
     W <<>>  All recommended stuff 
     -  MyComputer:Properties:Advanced:Performance 
          (<<>> 50meg swap; <<>> 2Gig swap) 
     - (net)settings (just the basics; no dipinst) 
     
    > HARDWARE 
     C verizon cell dialer 
     A AT&T Dialer (See apps:: for setup; C Drive) 
     A Aeronet card 
     A Sine 
     C Palm pilot (Verizon version) 
     - Plain Password Fix for DFS (so I can access Samba and other AIX machines) 
     W  Printers             <<>> 
                             ### Rename <<>> to <<>> 
     - DRIVERS: PowerMgt 
     - DAAT                  ### daatmaster.autsin.ibm.com 
    >   Create BL1 Ghost 
     C I-Feel mouse 
     
    > EARLY STUFF 
     - boot-o 
     - install-cygwin 
     - boot-install 
     A _GreatestCorners 
     A Notes 
     
    PRELOAD APPS 
     S  Norton Anti Virus    ### C DRIVE ###  (Server Edition) 
     x  Adobe Acrobat 
     S  Notes Client         (Integrated *client* with *designer* & *admistrator*) 
    .xx  Lotus Smartsuite 
     ?  Lotus Sametime 
     S  Personal Comm 
     x  PowerToys?? 
     W  quicktime.com 
     W  www.real.com         Real Player          Minimum 
     ?  Tweak UI            download.com  (right click install) 
     x  partition magic      <<>>>NewFloppy Windows\Setup\setup.exe; PP501ENWSCD-2191 
     W  microsoft.com/ie      
    STANDARD INSTALLS 
     ?  Win CAB files        <<>> 
    . C  Adobe Photoshop 4.0 
    . C  Adobe Framemaker 5.5 
    ?? Adobe Acrobat (on Framemaker CD) 
     
    . C  Install Shield 
    . C  Install Pkg For Web 
     X  Lotus Suite 9.5      Std Install.  (Do not install viavoice) 
    . C  Palm Pilot           Id: 'Daniel Oblinger'  (First out of cradle then in) 
    . C  Quicken              Typical:  Not Netscape...  
     x  Riven 
    . C  Visio 
     S  MatLab 
     C  MailAssistant 
    . S  MS Office            ### ADD: OfficeTools:EqEdit;  
                             ### FROM '97 WebAuthoring; Convert:Text: NOTES & HTML 
    INSTALL TREE APPS 
    - Ifeel 
     I  INSTALL-WINZIP 
     I  PGP                  MSG: "can't find keyrings" is expected;  
                             May click Eudora; then browse to it 
     I  Atomic Clock         ;;; System must be socksified. 
     I  Treesize Pro 
     I  GreatisCorners 
     I  Viceversa 
     x  SprintDialer 
     ?? activeperl.com       (Needed to perl runs under windows) 
     ?? Active state python (Just use under cygwin?) 
    Websphere 
     W  google               Taskbar 
     W  www.musicex.com/mediajukebox      ### Do setup while connected 
     W  www.spinner.com      (reboot to get explorer)  
     WI www.vindigo.com      ### IMPT!!! SEE APPS:  <<>> <<>> 
     I  Eudora               ### DATA DIR: <<>> 
     
     <<>>                   (Just ones listed) 
     
    DANGEROUS APPS 
    - VNC 
     x  MS Visual C++ 6.0 
     x  MSDN Library 6.0a    FULL install 
     
    <<>> 
     
    - Moved js32.dll from system32 --> vindigo (it breaks java notes access) 
    ? Ghost 2001 
     X Watson Domain 
     
    <<>> 
    OBLIO T21 WIN2000 Professional  (Build III after virus) 
    BASE SYSTEM 
     -  Fdisk:CreatePartition  <<>> 4gig 
     C  WINDOWS 2000 Server  (Typical options) 
     W  TP T21 DRIVERS  ibm.com -> driverMatrix -> Expand each to <<>> 
        - ForEach:  Video,TP-Util,OnScreen,SpeedStep,PwrMgt   Aud?, Track?, PCI? 
          - DeviceManager:Properties (on "!" ones): UpdateDriver 
        - ForEach:  OnScreen, SpeedStep 
     C  - Modem/PCI combo DeviceMgr:Properties:UpdateDriver (for 2 devices) 
     -  Aeronet (See *note apps::)  
     -  Configure Network   (see Install:net:settings) 
     -  Reboot; Create Oblinger user; Login 
     -  CntlPnl:AdmTools:CompMgmt:DiskManagement 
        - RtClick:CreatePartition  <<>>, PrimaryPartition, NTFS, RestOfDrive, Prime 
     -  MyComputer:Properties:Advanced:Performance 
          (<<>> 50meg swap; <<>> 2Gig swap) 
     -  add IIS 
     I  AT&T Dialer (See apps:: for setup; C Drive) 
     SW <<>>  critical win2000patch1+, Service pac 
    >   Create BL1 Ghost 
     
    PRELOAD APPS 
     *  <<>> Files     Change default program load directoy 
     S  Norton Anti Virus    ### C DRIVE ###  (Server Edition) 
     S  Adobe Acrobat 
     S  Notes Client         (Integrated *client* with *designer* & *admistrator*) 
     S  Lotus Smartsuite 
     S  Lotus Sametime 
     S  Personal Comm 
     S  PowerToys?? 
     W  quicktime.com 
     W  www.real.com         Real Player          Minimum 
     SW Tweak UI            download.com  (right click install) 
     SI partition magic      <<>>>NewFloppy Windows\Setup\setup.exe; PP501ENWSCD-2191 
     W  Printers             <<>> 
                             ### Rename <<>> to <<>> 
     W  microsoft.com/ie      
    STANDARD INSTALLS 
     C  Win CAB files        <<>> 
     C  Adobe Photoshop 4.0 
     C  Adobe Framemaker 5.5 
     C  Install Shield 
     C  Install Pkg For Web 
     C  Lotus Suite 9.5      Std Install.  (Do not install viavoice) 
     C  Palm Pilot           Id: 'Daniel Oblinger'  (First out of cradle then in) 
     C  Quicken              Typical:  Not Netscape...  
     C  Riven 
     C  Visio 
     C  MatLab 
     C  MailAssistant 
     C  MS Office            ### ADD: OfficeTools:EqEdit;  
                             ### FROM '97 WebAuthoring; Convert:Text: NOTES & HTML 
    INSTALL TREE APPS 
     I  INSTALL-WINZIP 
     I  PGP                  MSG: "can't find keyrings" is expected;  
                             May click Eudora; then browse to it 
     I  Atomic Clock         ;;; System must be socksified. 
     I  Treesize Pro 
     I  GreatisCorners 
     I  Viceversa 
     I  SprintDialer 
     IW activeperl.com       (Needed to perl runs under windows) 
     W  google               Taskbar 
     W  www.musicex.com/mediajukebox      ### Do setup while connected 
     W  www.spinner.com      (reboot to get explorer)  
     WI www.vindigo.com      ### IMPT!!! SEE APPS:  <<>> <<>> 
     I  Eudora               ### DATA DIR: <<>> 
     
     <<>>                   (Just ones listed) 
     
    DANGEROUS APPS 
     I  IBM Desktop On Call 
     C  MS Visual C++ 6.0 
     C  MSDN Library 6.0a    FULL install 
     W  microsoftupdate.micrsoft.com  (Service packs; Critical updates) 
     
    <<>> 
     
     
     
    XI  Music Match          ### DO NOT DO TYPICAL INSTALL; it will install on <<>> 
    XI  SocksCap             Settings:  socks1.server.ibm.com 
        from                 <<>> 
    XI  Britannica-eblast    taskbar item 
    X C  Delorme 7.0 
    X C  WMR Suite            (Do not overwrite newer .dlls) 
    xW  AdobeAcrobat 
    x C  Omni-Sky             Setup.exe 
    X I  Avantgo              (Pilot) 
     
     
    <<>> 
    OBLIO T21 WIN2000 Professional 
    BASE SYSTEM 
       IBM e-client 
    PRELOAD APPS 
     W= WSDC Downloads  <<>> 
     N  Printers             <<>> 
     W  partition magic      Extract; Windows\Setup\setup.exe 
     N  www.spinner.com      (reboot to get explorer)  
    STANDARD INSTALLS 
     C  MS Office            ### ADD: WebAuthoring; OfficeTools:EqEdit;  
                             ### Convert:Text: NOTES & HTML 
     C  Adobe Photoshop 4.0 
     C  Install Shield 
     C  Package For Web 
     C  Adobe Framemaker 
     C  Delorme              7.0 
     C  Palm Pilot           Id: 'Daniel Oblinger'  (First out of cradle then in) 
     C  Quicken              Typical:  Not Netscape...  
     C  Riven 
     C  Visio 
     C  Lotus Suite 9.5      Std Install.  (Do not install viavoice) 
     C  WMR Suite            (Do not overwrite newer .dlls) 
     C  MS Visual C++ 6.0 
     C  MSDN Library 6.0a    FULL install 
     C  MatLab 
     C  IIS from WIN2000     DISK 
    INSTALL TREE APPS 
     I  INSTALL-O winzip 
     I  SocksCap             Settings:  socks1.server.ibm.com 
        from                 <<>> 
     I  Eudora 
     I  PGP                  MSG: "can't find keyrings" is expected 
     I  Atomic Clock         ;;; System must be socksified. 
     I  Britannica-eblast    taskbar item 
     I  Treesize Pro 
     I  Music Match          ### DO NOT DO TYPICAL INSTALL; it will install on <<>> 
     I  Media Jukebox 5 
     I  Desktop On Call 
     I  GreatisCorners 
     I  AddressAssistant 
     I  SprintDialer 
     
     <<>>                   (Just ones listed) 
     
    COPY <<>> TREE; NOTES 
        NOTES 
         
     
    <<>> 
    xI  Personal Comm 
    xN  Netscape             www.netscape.com/download 
    xW  Norton AntiVirus     Win2000 -> Win2000 professional version 
    xW  IGN dialer           <<>> 
    xW  AdobeAcrobat 
    xN  Notes                (see App:: NOTES) 
    x C  Adobe Acrobat        installs w. harmless errormsg 
    x C  Omni-Sky             Setup.exe 
    ;;; I  IBM SoftwareInstall  (after explorer; before IGN dialer) 
    ;;; do install in apps section I  Sprint .DLL          FOR WIN 2000, just add .dll 
     
     
     
    >>>PR*SAFE 
    >>>PR*FULL 
     ?  Personal Communi... 
     C  Lotus Suite 
     move notes 
     palm 
    >>>PR3FULL 
     I  Vindigo              (Pilot) 
     I  Avantgo              (Pilot) 
     
     
     
     
    ;N  ISSI Installer       <<>> 
     C  WIN NT 4.0 (Jan 2000)  Laptop Configuation;  
        1.5 GIG <<>> partition 
        Add NetBEUI & DHCP; Cancel Token & Modem Setup 
     -  Dialup Networking  (MyComputer:DialupNetworking:install) 
        May need to swap modem to com1??? 
     C  NT Step 5 (Jan 2000) 
    DRIVERS:  Install\_DRIVERS\TP600-NT all 
     -  Audio:  CntlPnl:MultiMedia:Device:AudioDev:Add:HaveDisk 
     -  Video:  CntlPnl:Display:Setting:DspType:Change:HaveDisk 
     -  TrkPt:  CntlPnl:Mouse:General:Change:HaveDisk (drivers\win\tpoint\...) 
     -  Modem:  NOT CNTL PANEL!!  \drivers\win\modem\setup 
     -  drivers\wnt\utility\setup 
     -  Token:  Cntl:Network:Adaptor:Add:HaveDisk (from drivers\16-4token\trpcc2) 
     -  Configure Network   (see Install:net:settings) 
        - Add Administrator:oblinger (in FISHDOM!) 
        - Login as oblinger (in fishdom) don't create account 
    ; I  DRIVERS\config Util  (!Done above) Exec. then <<>> 
    ; I  TotalRecorder        Help->Register  "Dan Oblinger"  
                              6LHQ.AKWU.N8TK.1VDQ.CC3R.3M8F.2JSC.QOLS  
    ; I  partition magic      Serial: PM400ENP-586031 
    ; I  drive image 
    ; N  internet explorer 
    ; C  Sprint PCS           (Before palm) Uncheck launch @ start. 
                             Click Add, Dont detect, Select SprintPCS, 
    ;C  PC Anywhere          ### MUST ADD ANYWHERE PATCH BEFORE RE-BOOT ### 
    ;   pcAnywhere patch     ### DONT LOAD PCAny.. on new systems!!! till needed. 
    ; C  TP600 Install CD     ;Config Safe, ;IBM Installer, Netscape, PC-Doctor 
     
     
     
    <<>> 
    OLD INIT BUILD 
     
       Destroy existing partition.  (Use Win98 boot disk & fdisk;  SHOULD DO THIS?) 
     C WINDOWS 2000 Server  (Typical options) 
       - PARTITON DISK:  1.5Gig Fat16 
     C Aeronet utils\setup.exe  (See *note apps::)  
       Watson Win2000 patch 
       Configure Network   (see Install:net:settings) 
       Windows Update: win2000patch1+  (<<>>) 
       Reboot; Create Oblinger user; Login 
       Partition & Format <<>> 1.5Gig  <<>> Remainder 
       - CntlPnl:AdmTools:CompMgmt:DiskManagement 
         -  change drive letter (second disk should be <<>>) 
       - MyComputer:Properties:Advanced:Performance 
         (put all swap on <<>> 2Gigs) 
       - add IIS 
     
     
    <<>> 
    OBLIO TP600X  WIN2000 Professional 
    BASE SYSTEM 
       PARTITON DISK:  1.5Gig Fat16 
     C WINDOWS 2000 Server  (Typical options) 
       Watson Win2000 patch 
       Configure Network   (see Install:net:settings) 
       Windows Update: win2000patch1+  (<<>>) 
       Reboot; Create Oblinger user as part of FISHDOM; Login 
       Format remainder of disk as <<>>  
       - CntlPnl:AdmTools:CompMgmt:DiskManagement 
         -  change drive letter (second disk should be <<>>) 
    add IIS 
    PRELOAD APPS 
     W= WSDC Downloads  <<>> 
     W  partition magic      Serial for 5.01: PP501ENWSCD-2191 
     W  Norton AntiVirus     Win2000 -> Win2000 professional version 
     W  IGN dialer           <<>> 
     N  www.spinner.com      (reboot to get explorer)  
     N  Netscape             www.netscape.com/download 
     N  Notes                (see App:: NOTES) 
    STANDARD INSTALLS 
     C  MS Office            ### ADD: WebAuthoring; OfficeTools:EqEdit;  
                             ### Convert:Text: NOTES & HTML 
     C  Adobe Photoshop 4.0 
     C  Install Shield 
     C  Package For Web 
     C  Adobe Acrobat        installs w. harmless errormsg 
     C  Visio 
     C  Palm Pilot           Id: 'Daniel Oblinger'  (First out of cradle then in) 
     C  Omni-Sky             Setup.exe 
     C  Delorme              7.0 
     C  Quicken              Not Netscape...  
                             Cancel setup wizard; open <<>> 
     C  Riven 
     C  Lotus Suite 9.5      Std Install.  (Do not install viavoice) 
     C  WMR Suite            (Do not overwrite newer .dlls) 
     C  MS Visual C++ 6.0 
     C  MSDN Library 6.0a    FULL install 
     C  IIS from WIN2000     DISK 
    INSTALL TREE APPS 
     I  INSTALL-O winzip 
     I  SocksCap             Settings:  socks1.server.ibm.com 
        from                 <<>> 
     I  Eudora 
     I  PGP                  MSG: "can't find keyrings" is expected 
     I  Atomic Clock         ;;; System must be socksified. 
     I  Britannica-eblast    taskbar item 
     I  Treesize Pro 
     I  Music Match          ### DO NOT DO TYPICAL INSTALL; it will install on <<>> 
     I  Media Jukebox 5 
    ;;; I  IBM SoftwareInstall  (after explorer; before IGN dialer) 
    ;;; do install in apps section I  Sprint .DLL          FOR WIN 2000, just add .dll 
     I  Personal Comm 
     <<>>     
     <<>>                   (Just ones listed) 
     
    BUILD DRIVE IMAGE 
    COPY <<>> TREE 
     <<>>   (CONFIGURE) 
        NOTES, MS OUTLOOK, MS OFFICE, QUICKEN 
        Palm: HotSync ony when desktop active 
        Copy Riven Data? 
     
    >>>PR*SAFE 
    >>>PR*FULL 
     ?  Personal Communi... 
     C  Lotus Suite 
     move notes 
     palm 
    >>>PR3FULL 
     I  Vindigo              (Pilot) 
     I  Avantgo              (Pilot) 
     
     
     
     
    ;N  ISSI Installer       <<>> 
     C  WIN NT 4.0 (Jan 2000)  Laptop Configuation;  
        1.5 GIG <<>> partition 
        Add NetBEUI & DHCP; Cancel Token & Modem Setup 
     -  Dialup Networking  (MyComputer:DialupNetworking:install) 
        May need to swap modem to com1??? 
     C  NT Step 5 (Jan 2000) 
    DRIVERS:  Install\_DRIVERS\TP600-NT all 
     -  Audio:  CntlPnl:MultiMedia:Device:AudioDev:Add:HaveDisk 
     -  Video:  CntlPnl:Display:Setting:DspType:Change:HaveDisk 
     -  TrkPt:  CntlPnl:Mouse:General:Change:HaveDisk (drivers\win\tpoint\...) 
     -  Modem:  NOT CNTL PANEL!!  \drivers\win\modem\setup 
     -  drivers\wnt\utility\setup 
     -  Token:  Cntl:Network:Adaptor:Add:HaveDisk (from drivers\16-4token\trpcc2) 
     -  Configure Network   (see Install:net:settings) 
        - Add Administrator:oblinger (in FISHDOM!) 
        - Login as oblinger (in fishdom) don't create account 
    ; I  DRIVERS\config Util  (!Done above) Exec. then <<>> 
    ; I  TotalRecorder        Help->Register  "Dan Oblinger"  
                              6LHQ.AKWU.N8TK.1VDQ.CC3R.3M8F.2JSC.QOLS  
    ; I  partition magic      Serial: PM400ENP-586031 
    ; I  drive image 
    ; N  internet explorer 
    ; C  Sprint PCS           (Before palm) Uncheck launch @ start. 
                             Click Add, Dont detect, Select SprintPCS, 
    ;C  PC Anywhere          ### MUST ADD ANYWHERE PATCH BEFORE RE-BOOT ### 
    ;   pcAnywhere patch     ### DONT LOAD PCAny.. on new systems!!! till needed. 
    ; C  TP600 Install CD     ;Config Safe, ;IBM Installer, Netscape, PC-Doctor 
     
     
     
     
     
    <<>> 
    OBLIO TP600X  NT4.0 Workstation  (proposed) 
    BASE SYSTEM 
     C  WIN NT 4.0 (Jan 2000)  Laptop Configuation;  
        1.5 GIG <<>> partition 
        Add NetBEUI & DHCP; Cancel Token & Modem Setup 
     -  Dialup Networking  (MyComputer:DialupNetworking:install) 
        May need to swap modem to com1??? 
     C  NT Step 5 (Jan 2000) 
    DRIVERS:  Install\_DRIVERS\TP600-NT all 
     -  Audio:  CntlPnl:MultiMedia:Device:AudioDev:Add:HaveDisk 
     -  Video:  CntlPnl:Display:Setting:DspType:Change:HaveDisk 
     -  TrkPt:  CntlPnl:Mouse:General:Change:HaveDisk (drivers\win\tpoint\...) 
     -  Modem:  NOT CNTL PANEL!!  \drivers\win\modem\setup 
     -  drivers\wnt\utility\setup 
     -  Token:  Cntl:Network:Adaptor:Add:HaveDisk (from drivers\16-4token\trpcc2) 
     -  Configure Network   (see Install:net:settings) 
        - Add Administrator:oblinger (in FISHDOM!) 
        - Login as oblinger (in fishdom) don't create account 
     -  Format remainder of disk as <<>>  (start:program:adminTools:DiskAdmin) 
     <<>>     
    PRELOAD APPS 
     I  PGP                  MSG: "can't find keyrings" is expected 
     I  Atomic Clock         ;;; System must be socksified. 
     I  Britannica-eblast    taskbar item 
     I  Treesize Pro 
     I  Music Match          Save new music to <<>> 
                             Options:MusLib:Open <<>> 
                             Record Options 
     I  DRIVERS\config Util  (!Done above) Execute. then <<>> 
     I  TotalRecorder        Help->Register  "Dan Oblinger"  
                             6LHQ.AKWU.N8TK.1VDQ.CC3R.3M8F.2JSC.QOLS  
     I  SocksCap             Settings:  socks1.server.ibm.com 
    ; I  partition magic      Serial: PM400ENP-586031 
    ; I  drive image 
     N  internet explorer 
     I  IBM SoftwareInstall  (after explorer; before IGN dialer) 
     N  IGN dialer           <<>> 
     N  www.spinner.com      (reboot to get explorer)  
     N  Notes                (see App:: NOTES) 
     <<>>                   (Just ones listed) 
     
    BUILD DRIVE IMAGE 
    COPY <<>> TREE 
     
    STANDARD INSTALLS 
     C  Sprint PCS           (Before palm) Uncheck launch @ start. 
                             Click Add, Dont detect, Select SprintPCS, 
     C  MS Office            ### ADD: WEB AUTHORING & L. NOTES ### 
     C  TP600 Install CD     ;Config Safe, ;IBM Installer, Netscape, PC-Doctor 
    XC  PC Anywhere          ### MUST ADD ANYWHERE PATCH BEFORE RE-BOOT ### 
        pcAnywhere patch     ### DONT LOAD PCAny.. on new systems!!! till needed. 
     C  Adobe Photoshop 4.0 
     C  Install Shield 
     C  Package For Web 
     C  Adobe Acrobat        installs w. harmless errormsg 
     C  Visio 
     C  Palm Pilot           Id: 'Daniel Oblinger'  (First out of cradle then in) 
     C  Delorme              7.0 
     C  Quicken              Not Netscape...  
                             Cancel setup wizard; open <<>> 
     C  Riven 
     C  Lotus Suite 9.5      from kay 
     <<>>   (CONFIGURE) 
        NOTES, MS OUTLOOK, MS OFFICE, QUICKEN 
        Palm: HotSync ony when desktop active 
        Copy Riven Data? 
    >>>PR*SAFE 
     C  WMR Suite            (Do not overwrite newer .dlls) 
     C  MS Visual C++ 6.0 
     C  MSDN Library 6.0a    FULL install 
    >>>PR*FULL 
     ?  Personal Communi... 
     C  Lotus Suite 
     move notes 
     palm 
    >>>PR3FULL 
     I  Vindigo              (Pilot) 
     I  Avantgo              (Pilot) 
     
     
     
    <<>> 
    OBLIO TP600X  WIN2000 Server 
     C WINDOWS 2000 Server  (Typical options) 
       Configure Network   (see Install:net:settings) 
       Reboot; Create Oblinger user as part of FISHDOM; Login 
    STANDARD INSTALLS 
     C  Adobe Photoshop 4.0 
     C  Install Shield 
     C  Package For Web 
     C  MS Visual C++ 6.0 
     C  MSDN Library 6.0 
     C  MS Office            ### ADD: WEB AUTHORING & L. NOTES ### 
     C  Adobe Acrobat        seemed to install; but had error 
     C  Visio 
     C  Palm Pilot           Id: 'Daniel Oblinger'  (First out of cradle then in) 
     C  Delorme              7.0 
     C  Quicken              Cancel setup wizard; open <<>> 
     C  WMR Suite            (Do not overwrite newer .dlls) 
     C  Riven 
     C  Lotus Suite 9.5      from kay 
     C  TP600 Install CD     Config Safe, IBM Installer, Netscape, PC-Doctor 
     <<>>     
    <<>> TREE (Copy it; init-o; install-o) 
     C  PC Anywhere          ### MUST ADD ANYWHERE PATCH BEFORE RE-BOOT ### 
        pcAnywhere patch 
     I  PGP                  MSG: "can't find keyrings" is expected 
     I  Atomic Clock         ;;; System must be socksified. 
     I  Britannica-eblast    taskbar item 
     I  Treesize Pro 
     I  Music Match          Options:MusLib:Open <<>> 
     I  DRIVERS\config Util  Execute. then <<>> 
     N  www.spinner.com      
     I  TotalRecorder        Help->Register  "Dan Oblinger"  
                             6LHQ.AKWU.N8TK.1VDQ.CC3R.3M8F.2JSC.QOLS  
     <<>>   (CONFIGURE) 
        NOTES, MS OUTLOOK, MS OFFICE, QUICKEN 
        Palm: HotSync ony when desktop active 
        Copy Riven Data? 
     
     
     
     
    ;;; I  Partition Magic 
    ;;; C  Adobe Framemaker 
    ;;; I  SHUTDOWN NOW         365387501 
     
     
    <<>> 
    OBLIO NT for TP600  0325 
     - NT4.0 
     - Std Apps: Advantis, Notes, Printers, Netscape 
    CONFIG 
     - NetNeighborhood 
     - Desktop & menus 
     - UI Tweaks 
    STANDARD INSTALLS 
     C  Adobe Photoshop 4.0 
     C  Install Shield 
     C  Package For Web 
     C  MS Visual C++ 6.0 
     C  MSDN Library 6.0 
     C  MS Office 
     C  Adobe Acrobat        seemed to install; but had error 
     C  Visio 
     C  Palm Pilot  
     C  Delorme              7.0 
     C  PC Anywhere 
     C  Quicken              Cancel setup wizard; open <<>> 
     C  Adobe Framemaker 
     I  Winzip Self Ext      Not needed) 
     I  Winzip6.3            a/d/disks/winzip6.3/setup 
     I  PGP                  can't find keyrings 
     I  Atomic Clock 
     I  Britannica-eblast    taskbar item 
     I  SHUTDOWN NOW         365387501 
     I  Treesize Pro 
     I  InternetExplorer5    Click everything; Advanced install later 
     C  WMR Suite 
     C  Riven 
    CONFIG 
        MS OFFICE, QUICKEN 
     
    SPECIAL COPYING 
        Quicken?, Riven 
     
    END CONFIGS 
        My 'A' tree; init-o; install-sys 
        Sync Palm pilot 
     
    <<>> 
    OBLIO SETUP 9628 
     C  Palm Pilot  
     C  Adobe Photoshop 4.0 
     C  Install Shield 
     C  Package For Web 
     C  MS Visual C++ 6.0 
     C  MSDN Library 6.0 
     C  MS Office 
     C  Adobe Acrobat (seemed to install; but had error) 
     C  Adobe Framemaker 
     C  Visio 
     C  WMR Suite 
     C  Britannica (FAILED: no key numbers) 
     C  Visual Age For Java 2.0 (From eric 
     I  KAWA 
     I  PGP (can't find keyrings) 
     I  Atomic Clock 
     I  Britannica-eblast taskbar item 
     
    X C  Colorado Backup 
    X C  IOMega Zip Disk 
    X C  DeLorme 
     
    <<>> 
    BINKY SETUP 9525 
     C  Install Shield 
     C  Package For Web 
     C  MS Visual C++ 6.0 
     C  MSDN Library 6.0 
     C  MS Office 
     C  Colorado Backup 
     C  Palm Pilot  
     C  IOMega Zip Disk 
     C  Adobe Acrobat (seemed to install; but had error) 
     C  FAILED: Visual Age for Java 
     I  WinZip from floppy (created by cmd in log file) 
     I  WinZip self extractor (non-registered copy) 
     I  Britannica-eblast taskbar item 
     I  PGP (can't find keyrings) 
     I  Atomic Clock 
     I  WS_FTP 
     I  JAVA; Kawa 
     C  Britannica (no key numbers) 
     
    <<>> 
    BURN IN LAB SETUP 
     C  FDISK 6G; FORMAT; WIN95  (688M) 
        Notes 
        Personal Communications 
        Lotus WP 
        ==SETUP== 
        TokenRing card; Dynamic IP 
        PSM; printer 
        Global Dialer 
    MY SETUP 
     C  Colorado Backup 
     C  Install Shield 
     C  MS VC++ 
     C  Photoshop 
     C  Palm Pilot 
     C  WMR; Admin; Diag 
    +C  MS Office 
     I  JAVA; Kawa 
     C  IOmega Zip 
     N  Netscape 
     
    TINY APPS 
     I  Real Audio  ?? 
     I  VP buddy 
      
     
    CONFIGURATION 
     
     
    CONFIGURATION DETAILS 
     -  Enable hibernation.  User ``oblinger''.  Clean systray, menus, etc. 
     
    AFTER TAPE BACKUP 
     C  Palm Pilot 
     
     
    =================== 
     
    INITIAL INSTALL 2/26/98 
    - Fdisk 
    - Format (s) 
    - Dos 6.3 
    - CD Drivers (NONE NEEDED??? Note: win95 reboots during install) 
    - Win95  CDROM:win95\retail\full\setup 
    - Matronics Video Drivers  <<>> 
    = Soundblaster setup 
    - Bill's Util  Unzip to \dosutils & add to path 
    = Iomega Zip 
    - VTDSDK 
    - WMR release version 
    - MSDS (MS C++) 
    - Microsoft Developer Network 
    = Watch me read source setup 
    - Adobe 4.0 
    - Slick Edit 
     
     
    CD DRIVERS  (Not Done) 
    copy  
    autoexec add  
    config   add     
     
