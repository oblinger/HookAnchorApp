# Install-bak.Apps1 --

    <<>> 
    GENERAL PERSONALIZATION 
     
    > \a\d\My\Install\_Cygwin\Setup.exe 
      Unix! ForEveryone! <<>> 
     
    > Boot-Install  (or execute all in the install dir) 
     
    > ControlPanel 
      - FolderOptions  (NT=file browser:view:options) 
        General: WebView=Classic 
        View (Many changes) 
      - Display:Background (Coffee Bean) 
      - PowerOptions   (ThinkPadConfig:Power) 
        - PowerSchemes:PluggedIn 
          Turn off monitor (NEVER) 
        - Advanced 
          Always show icon on taskbar 
          (UNCHECK) Prompt for password 
        - Hibernate  (enable) 
     
    > MyComputer:View options ... 
     
    > Enable swapping to <<>> drive 
      MYComputer:Prop:Advanced:Performace:Cache: 500-2000 on <<>> 
     
    > Create StartMenu:Programs:Toplevel 
      - Remove things from start bar; startmenu; desktop 
     
    > TaskMgr  
      Options:HideWhenMinimized 
     
    > Add links to the a\d\tree\App\System\Bin 
     
    > MyComputer:C:Properties:Sharing 
      NewShare:  C as O  (Permisions for oblinger) 
      \a            (Read Share) 
      \a\d\public   (Full Share) 
      \a\d\install  (Read Share) 
     
    > Place windows #1 #2 #3 #4 
     
    > Move shortcuts to toplevel folder 
     
     
    <<>> 
    GREATEST CORNERS 
      (If you get an error exit & restart) 
     
    I _GreatistCorners 
     
    > SystemTray:RtClick! Buy/Register: 
      User: "oblinger@us.ibm.com"   
      Reg Code: "RU-038-707" 
     
    > Completely Terminate Greatest Corners 
     
    > Start:GreatisCorners:Action:Keys:Add! 
       Type "`";  Ok! 
       Action: "Folder"  (NOT Folders Menu or Folder collection) 
       ApplicationOrFolderPath: "<<>>" 
      Refresh: 
     
    <<>> 
    AERONET WIRELESS WEB CARD 
     
    SAMNI 10/02 
     
    IBM I/S: "wireless"  "how do i install ... 
    - Download and run CLW_Install  (WITH AERONETNET CARD OUT & OFF NETWORK) 
     
    - Turn off net firewall; put in card 
    - Run second install (derived from first) 
     
    DO NOT ALLOW WINDOWS TO CONTROL WIRELESS CARD 
      - In profile manager, and in wireless connection settings 
     
     
    <<>> 
    I _Aeronet   (cisco.com ... softw cntr:wireless software:Cisco LAN client) 
     
     
     
    - C  CD:\utils\setup.exe 
    - ClientUtils:Commands:Edit properties:  
      Client:  "" 
      SSID1: "IBM" 
      RF:EnableWEP! 
         Authentication:SharedKey! 
     
    - XP just recognized the device 
    <<>> cisco site for Aeronet 340 adaptors 
     
     
    <<>> 
    MTS IGN DIALER  ATT AT&T Global Dialer 
     
     
     
    - Version 5.03.1 from samni  (in install) 
    - ISSI:MTS AT&T Network Dialer   
     
      <<>>, <<>>  DNS: 9.2.250.86  9.14.1.30 
      See (ids) for password 
      ... Both the intranet and internet 
      ... Managed VPN (IPSEC) 
     
    - Desktop:AT&T Net Client 
      - click triangle in upper left corner:login properties 
      - check "override defaults" 
     
    - CntlPanel:Network and internet:NetworkConnections: AT&T net client:properties 
      Uninstall QOS service 
     
    - Change modem speed to 56K 
     
     
    X Add following locations in any order 
      Cell, Home, Remote, and Work 
     
     
    <<>> 
    SINE 
     
    - ISSI:IBM Aventail ??? 
    - ISSI:IBM Aventail clinet config ??? 
    L Sine version from support team 
     
    - Run install  
     
    - Copy three config files to aventail/connnect 
     
     
     
     
     
    <<>> 
    DVD (Purchased from National Multimedia Products) 
     
    I 28-DVD-Player 
    - <<>> 
     
    <<>> 
    NOTES -- Win2000 
     
    Download&Install from parrot: 
      R5Global-504a-client 
    Copy Lous/Notes/data files to new drive ***BEFORE*** install 
      desktop.dsk, Desktop5.dsk, names.nsf, oblinger.id, oblinger.nsf 
     
    Start Notes 
      Say "no" ... (browse to oblinger.id) 
     
    D01ML020/01/M/IBM 
    ...ML244...??? 
     
    <<>> 
    INTERNET EXPLORER 
    > InternetExplorer 
    xxx  Proxy: (socks)  socks1.server.ibm.com 1080 
      Homepage:       w3.watson.ibm.com 
     
    <<>> 
    EUDORA 
    When installed data dir should be pointed at <<>> 
     
    <<>> 
    OFFICE 97 -- Win2000 
     
    (Custom w. no changes) 
     
    Run Word:Tools:Options:File Locations 
      Documents        <<>>                                       ; load dir 
      User Templates   <<>> Office\Templates       ;  
      Startup          <<>> Office\Office\STARTUP  ; Ld initally 
     
    Tool:Options:Spelling&Grammar: 
      Dictionaries:Add  <<>> Office\Office 
      CustomDictionary  DAO.dic 
      Deselect Custom.dic 
     
    (Dont save changes to the old Normal.dot) 
    Save to \a\d\conf\Microsoft Office\Templates 
    (Maybe I should not overwrite that Normal, but use its setting instead of 
     configuring.  i.e. just change file locations?) 
     
    <<>> 
    QUICKEN -- Win2000 
     
    Quicken; cancel new user config 
      File:Open:a\d\conf\quicken\qdata1 
    <<>> 
    PILOT -- ??? 
     
    - Palm Pilot Reset 
    - Set Date 
    - Reload pilot 
    - Add software in \a\s\pkg\pilotapps\active 
    - Change the application categories from unfiled 
     
    - Palm:Hotsync:setup: only when desktop is loadeds 
     
    VINDIGO 
    - <<>>  <<>> 
    - Move js32.dll from system32 --> vindigo (it breaks java notes access) 
    xxxx <<>> 
     
    <<>> 
    VICE VERSA 
     
    Enter registration code 
    - ViceVersa:Help:EnterRegistrationInfo 
      Name: oblinger@ponymail.com   Key: VV200-2001-SSxxYY 
     
    <<>> 
    :::MUSIC:::  MEDIA JUKEBOX 5 
     
    - execute install:__Music:09-MediaJukebox5:enc_lame.mjp 
    - Settings:Options:Encoding:Recording:Compression = Lame MP3, 128bps 
    - Options:Encoding:Normalize = 95% 
    - Options:FileNaming: 
      RecordedMusicBase: <<>> 
      Temp:              <<>> 
      TrackFileNaming:    -  
    AssociateWithAudioCDs 
    - Options:ProxySettings:... 
     
    ?? Normalization 
    ?? 
    <<>> 
    ARCHOS JUKEBOX 6000 
     
    - Connect by cable 
    - Find driver install:_archos:JB_Driver:ISD200.INF 
     
    <<>> 
    IBM HOST ON DEMAND 
     
    - Install IIS; and security patches 
     
    Start:IBMHostOnDemand:Administration:DeploymentWizard 
    - All std options.  PageTitle=machine-name; FileName=machine-name. 
     
    <<>> 
    PERL 
     
    - Cygwin has perl  
      Trying to use this perl for all things (need cygwin file str & shell) 
      - must add cygwin/bin to dos path so external launch works 
      - PATHEXT should have .PL on end 
      - ASSOC & FTYPE should point to correct perl 
      - PBANG.EXE should point to correct perl 
      - ActiveState should not be loaded 
     
    - OpenWith menu can be used to set .pl to point to particular perl 
      (seems to override reg menu entries.  you can delete these from: 
      CURRENT_USER/Software/Microsoft/Windows/CurrentVersion/Explorer/FileExts/ 
    - perl -v show version in use. 
     
    HACK PATHEXT should list .PL  but adding .PL;%PATHEXT% fails in boot-vars.bat 
     
     mv <<>> <<>> 
     ln -s <<>> <<>> 
     
    Ways to invoke 
    Start: X           (Xlate to dos: X) ??? 
    dos:  test-perl    (Xlate thru PATHEXT to test-perl.pl) 
    dos:  test-perl.pl (Xlate thru ASSOC and FILETYPE) 
    bash: test-perl     
    bash: test-perl.pl 
     
     
    ############################################################################### 
    ##########################   O L D   S T U F F   ############################## 
    ############################################################################### 
     
    <<>> 
    SPRINT CELL PHONE 
     <<>>     
     
    - connect cable & phone 
      Install hardware browse to install/_SprintModem/Win2000.../*.inf 
     
    - \a\d\install\..2.01  ... .exe 
      Read docs around page 115 
      (Sometime COM4 does not show up.  Fool around till it does. 
       It came up after reboot) 
     
    - control panel:system:hardward wizard:Add 
      (it will detect phone & add StdModem) 
      (will only show up in System Manager when phone is connected) 
    - control panel:system:hardware:device manager:modem:standard modem:properties 
      driver:UpdateDriver:SpecifyLocation  (\a\d\install\_SprintModem) 
     
     
    <<>> 
    xxx ADSM 
     
    reswat5.research.ibm.com/projects/service_request/service.nsf/request+adsm 
     
    <<>> 
    xxx MUSIC MATCH 
      Options:Settings:recorder:songs directory: 
         Save new music to <<>> 
      Opts:MusLib:Open <<>> 
     
     
    --- NOT ANYMORE --- 
    Notes:Workspace:CommonPrintSolution (view by site) ykt 
    -  Add all printers on 24-2 and 30-2 
    -  Set default printer: ykt4clb 
    <<>> 
    xxx NETSCAPE -- Win2000 
     
    Installed Netscape 3.01 from Winapps drive 
    > Edit:Preferences:Advanced:Proxys: - click Manual Proxy config 
        socks2.watson.ibm.com,  port - 1080 
     
    <<>> 
    xxx MICROSOFT OUTLOOK -- Win2000 
     
    - Control Panel:Mail Fax 
      - Remove all services 
      - Add "Personal Folders" 
        Browse to <<>>   (Do not point to the <<>> drive) 
        Change Name to "Dan's Mail" 
     
      - Add "email-internet" address.  OK 
        General 
          Account:    PonyMail 
          Name:       Dan Oblinger 
          Email Addr: oblinger@pobox.com 
        DELIVERY  (actually only seen from "outlook:tools:services" 
          Deliver New Mail: Dan's Mail 
        SERVERS 
          SMTP:       ponymail.com 
          POP3:       ponymail.com 
          User:       oblinger 
          Pass:       M7 
        CONNECTION 
          modem     (lan???) 
     
    - LOGOFF 
     
    - Start:...Outlook: 
       Click the "contacts" folder in the inbox folders tree 
       Properties:Outlook Address:Click "show folder as e-mail address book" 
     
    - In left most column, right click the column name : Hide Outlook Bar 
      
    - Outlook:Tools:Options:AutoArchive 
      <<>> Archive\archive.pst 
     
    - Tools:services:Outlook Address Book:  click "lastname,firstname" 
      :addressing:  "Select Contacts first" 
     
    <<>> 
    ON PALM (OMNI SKY) 
     
    system:preferences:Network 
    Menu:new: 
    Service=IGS 
    ig 
     
    No queryy DNS 
     
    9.2.99.81 
    9.2.250.86 
     
    <<>> 
    REAL JUKE BOX -- Win2000 
    Change registry 
    HKEY_CLASSES_ROOT\Software\RealNetwork\RealJukeBox\Preferences 
      databasePath= "<<>>" 
    Change recording options 
    Change file locations 
     
     
    <<>> 
    PORTABLE DRIVE BAY 
     
    - Devbay.exe 
    - CntlPnl:SCSI Adaptors:Drivers:Add:HaveDisk (<<>>) 
     
     
     
