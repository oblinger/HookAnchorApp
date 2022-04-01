# Linux.Samba --



    # Create SAMBA
    1) <<>>  # edit on server
    2) <<>>           # edit on client
    3) create <<>> dir on client
    # service should be started



    service smb start
    smbclient -L \\\\172. \
    map net drive


    [SAMBA]
    <<>>
    sudo smbpasswd -a oblinger   # AsRoot: Creates password in <<>>


    service smb start
    smbclient -L \\\\172. \
    map net drive

    service restart smb
