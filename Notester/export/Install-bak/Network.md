# Install-bak.Network --

     <<>>     
     
    INDEX 
    - SETUP DRIVERS (not documented) 
    - SETUP NETWORK PARAMS 
     
    >>>SETUP NETWORK PARAMS 
    Start:CntrlPanel:networking:   [ADD:] 
     "Dial-up Adaptor"   (Under adapter; microsoft) 
     "IBM Turbo 16/4 Token-Ring PC Card"   (Added automatically when connected) 
     "Client for Microsoft Networks",  
     "File and printer sharing for Microsoft Networks"  (browse master????) 
     "NetBEUI" & "TCP/IP"  (both under protocalls) 
     "IBM Networks Client for Win 95" ??? 
     
      - identification  Set machine name & FISHDOM 
      - ClientForMSNetworks   Click Logon; NT domain=FISHDOM; QuickLogon 
      - TCP/IP tokenRing:properties:DNS Conf 
        host=; domain="diz.watson.ibm.com" 
        DNS="9.2.250.86"  "9.2.254.86"  "9.2.99.81" 
        DomainSuffix="watson.ibm.com", "ibm.com" 
     
     
    LEVELS OF OPERATION 
    * Boot 
    * Ping out 
    * Netscape 
    * Notes 
     
    ! NOBOOTLOG        -- Do not get network name at login time 
    ! NONEIGHBORHOOD   -- Cannot access NN "entire network" 
    ! NOINCLUSION      -- Machine not included in NN list 
      - Identification not set correctly 
     
     
     
     
    TCP/IP parameters: 
    - Enable DNS 
    - cub  watson.ibm.com 
     
    DNS search order 
    9.14.1.30        <-- Eastern US?? 
    165.87.194.244 
     
    Domain Suffix Order 
      watson.ibm.com 
      pok.ibm.com 
      ibm.com 
     
     
     
    Token Ring Adaptor for Laptop 
    - Control:AddNewHW:NetworkAdaptor:Havedisk 
      - <<>>  -> IBM PCI Token-Ring Adaptor, NDIS2/3 
     
    =========== 
    === LOG === 
    =========== 
     
    --- 
    MOUNTING NETWORK DRIVES 
    net use <<>> \\ykdls01\winapps 
    net use <<>> \\ykdls01\win95 
    plus winbeta 
    --- 
    DIRECT SOCKS SUPPORT 
    - Get the WSDC:WinApps hummingbird socks app 
      Unzip and Run install.bat FROM DOS 
      May need to added <<>>  from IBM Winapps drive 
     
     
     
