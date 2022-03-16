# VMware\_Install --



    === INSALL ===
    - DOWNLOAD image
    - VMWARE TOOLS
      - !install VMware tools  (bottom of window)
      - In Terminal
        % sudo su
        % cd <<>> Tools
        % mkdir ~/vmware
        % cp VM* ~/vmware
        % cd ~/vmware
        % tar -xvf VM*
        % ./ ??? .pl
      - NOTE:  Must restart image to complete
        (Screen will not resize until you do this)
    - CONFIGURE
      Virtual Machine -> Settings -> Options -> Shared Folders
        - !AlwaysEnable
        - !Add:   (Add <<>> and <<>>)
