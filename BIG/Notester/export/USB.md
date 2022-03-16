# USB --

    [USB]
    #changed X server config; Added input selection
    mount -t usbdevfs usb <<>>  # added to fstab
    modprobe hid
    modprobe usb-uhci  # usb hub support
    modprobe mousedev  # usb mouse support
