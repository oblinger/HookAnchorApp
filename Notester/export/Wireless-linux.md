# Wireless-linux --

    [LEVELS]
    - Not Associated in ACU
      . set SSID  & other ACU Parameters& default profile in ACU
    - No IP address in ipconfig
      . router down???
    - Extranet Web
    - Intranet Web
    - Net Neighborhood





    sudo ifconfig ipsec0 mtu 300
    sudo ifconfig eth1 mtu 300



    [WIRELESS]
    <<>>     # is eth1 up?



    <<>>
       acu           Cisco GUI

    cardctl status







    Apr 24 13:23:00 oblio kernel: airo:  Probing for PCI adapters
    Apr 24 13:23:00 oblio kernel: airo:  Finished probing for PCI adapters
    Apr 24 13:23:00 oblio cardmgr[794]: executing: 'modprobe airo_cs'
    Apr 24 13:23:00 oblio cardmgr[794]: executing: './network start eth1'
    Apr 24 13:23:00 oblio kernel: airo: MAC enabled eth1 0:40:96:39:f8:b6
    Apr 24 13:23:00 oblio kernel: eth1: index 0x05: Vcc 5.0, Vpp 5.0, irq 5, io 0x01
    00-0x013f
    Apr 24 13:23:00 oblio cardmgr[794]: + (RC=127) <<>> "oblio@
    us.ibm.com" "******" ; Output: <<>> <<>>
    apscript: No such file or directory
    Apr 24 13:23:00 oblio <<>> invoke ifup eth1
    Apr 24 13:25:00 oblio dhcpcd[11304]: timed out waiting for a valid DHCP server r
    esponse 
    Apr 24 13:28:30 oblio cardmgr[794]: shutting down socket 1
    Apr 24 13:28:30 oblio cardmgr[794]: executing: './network stop eth1'
    Apr 24 13:28:31 oblio kernel: airo:  WEP_TEMP set 21ff
    Apr 24 13:28:31 oblio cardmgr[794]: + Operation failed.
    Apr 24 13:28:31 oblio cardmgr[794]: executing: 'modprobe -r airo_cs'
    Apr 24 13:28:31 oblio cardmgr[794]: executing: 'modprobe -r airo'
    Apr 24 13:28:31 oblio <<>> NET unregister event not supported
    Apr 24 13:28:39 oblio cardmgr[794]: initializing socket 1
    Apr 24 13:28:39 oblio cardmgr[794]: socket 1: Aironet PC4800
    Apr 24 13:28:39 oblio cardmgr[794]: executing: 'modprobe airo ssids=IBM auto_wep
    =1'
    Apr 24 13:28:39 oblio kernel: airo:  Probing for PCI adapters
    Apr 24 13:28:39 oblio kernel: airo:  Finished probing for PCI adapters
    Apr 24 13:28:39 oblio cardmgr[794]: executing: 'modprobe airo_cs'
    Apr 24 13:28:39 oblio kernel: airo: MAC enabled eth1 0:40:96:39:f8:b6
    Apr 24 13:28:39 oblio kernel: eth1: index 0x05: Vcc 5.0, Vpp 5.0, irq 5, io 0x01
    00-0x013f
    Apr 24 13:28:39 oblio cardmgr[794]: executing: './network start eth1'
    Apr 24 13:28:39 oblio cardmgr[794]: + (RC=127) <<>> "oblio@
    us.ibm.com" "******" ; Output: <<>> <<>>
    apscript: No such file or directory
    Apr 24 13:28:39 oblio <<>> invoke ifup eth1
    (END) 
