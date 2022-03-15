# ProcSketch -- \_

    Key:
      [[Step name]]             # Group of related steps

      [name]                    # Definition of step 'name' is on subsequent lines
      ALT                       # Means that all text until next 'ALT' is one
                                  alternative that can be chosen
      ALL                       # Means this set of actions must be executed 
                                  but it can be done before or after other 'ALL' clauses
          !  Networking         # Means click on 'Networking'
          !! Networking         # Means double click on 'Networking'
          R! Networking         # Means right click on 'Networking'
          B! ButtonName         # Means left click on  button 'ButtonName'
          M! MenuItem           # Means select 'MenuItem' from current menu or submenu
          T! TabName            # Means left click on tab 'TabName'
          Type "letter"         # Means press 'l' 'e' 't' 't' 'e' 'r' 's' keys
                         # Goto step named 'label'


    [[STEP 1 -- Go to Connection Properties]]

    [Start]
    ALT  !Start !Settings !!ControlPanel  
    ALT  !Start !Network and dialup connections  
    *ALT  !!MyComputer !ControlPanel  


    [ControlPanel]
         ! Network and Dialup     


    [NetworkConnections]
         R! Local Area Connection  ! Properties  



    [[STEP 2 -- Go to DNS stuff]]

    [Step2]  aka
    [ConnectionProperties-1]
         !  Net Firewall                      # If not connected
         

    [ConnectionProperties-2]
         R! Internet Protocal TCP/IP 
         !  Properties  
         !  Advanced  
         !  DNS  
         


    [[STEP 3 -- Set DNS Stuff]]

    [TCP-IP DNS]
    ALL  ! DNS ADD  Type "9.2.250.86"  ! ADD    # only done if missing
    ALL  ! Append these DNS Suffixes            # if not checked
         ! Add (suffix) Type "watson.ibm.com" ! Add  # only done if missing
    ALL  

    [CloseDNS]
         ! Ok  ! Ok  ! Ok
         CLOSE Network and Dialup Connections


    DNS   
