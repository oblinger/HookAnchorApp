# HardDrive --


    # Creating partition table
    fdisk <<>>     n (new)   p (primary)  w (write)

    # Format partition
    mke2fs <<>>
    # Create journaling file system
    tune2fs -j <<>>   

    # Add line to <<>>
    <<>>      <<>>   ext3 defaults 0 0

    # Mkdir
    <<>>





    ------------------
    from tessa (build partition)


    Script started on Thu Apr  1 16:06:39 2004
    ]0;tlau@merlin:/home/tlau[root@merlin tlau]# mount
    <<>> on <<>> type ext3 (rw)
    none on <<>> type proc (rw)
    none on <<>> type devpts (rw,gid=5,mode=620)
    none on <<>> type tmpfs (rw)
    <<>> on <<>> type ext3 (rw)
    AFS on <<>> type afs (rw)
    <<>> on <<>> type ext3 (rw)
    ]0;tlau@merlin:/home/tlau[root@merlin tlau]# umount <<>>
    ]0;tlau@merlin:/home/tlau[root@merlin tlau]# fdisk <<>>

    The number of cylinders for this disk is set to 232581.
    There is nothing wrong with that, but this is larger than 1024,
    and could in certain setups cause problems with:
    1) software that runs at boot time (e.g., old versions of LILO)
    2) booting and partitioning software from other OSs
       (e.g., DOS FDISK, OS/2 FDISK)

    Command (m for help): p

    Disk <<>> 16 heads, 63 sectors, 232581 cylinders
    Units = cylinders of 1008 * 512 bytes

       Device Boot    Start       End    Blocks   Id  System
    <<>>             1    232581 117220792+  83  Linux

    Command (m for help): d
    Partition number (1-4): 1

    Command (m for help): n
    Command action
       e   extended
       p   primary partition (1-4)
    p
    Partition number (1-4): 1
    First cylinder (1-232581, default 1): 
    Using default value 1
    Last cylinder or +size or +sizeM or +sizeK (1-232581, default 232581): 
    Using default value 232581

    Command (m for help): v
    62 unallocated sectors

    Command (m for help): p

    Disk <<>> 16 heads, 63 sectors, 232581 cylinders
    Units = cylinders of 1008 * 512 bytes

       Device Boot    Start       End    Blocks   Id  System
    <<>>             1    232581 117220792+  83  Linux

    Command (m for help): w
    The partition table has been altered!

    Calling ioctl() to re-read partition table.
    Syncing disks.
    ]0;tlau@merlin:/home/tlau[root@merlin tlau]# mke2fs -j <<>>
    mke2fs 1.27 (8-Mar-2002)
    Filesystem label=
    OS type: Linux
    Block size=4096 (log=2)
    Fragment size=4096 (log=2)
    14663680 inodes, 29305198 blocks
    1465259 blocks (5.00%) reserved for the super user
    First data block=0
    895 block groups
    32768 blocks per group, 32768 fragments per group
    16384 inodes per group
    Superblock backups stored on blocks: 
        32768, 98304, 163840, 229376, 294912, 819200, 884736, 1605632, 2654208, 
        4096000, 7962624, 11239424, 20480000, 23887872

                   
    Creating journal (8192 blocks): done
    Writing superblocks and filesystem accounting information: done

    This filesystem will be automatically checked every 27 mounts or
    180 days, whichever comes first.  Use tune2fs -c or -i to override.
    ]0;tlau@merlin:/home/tlau[root@merlin tlau]# vim <<>>
    [?1048h[?1047h[?1h=[1;25r[?25h[?25h[27m[m[H[2J[?25l[25;1H"<<>>" 11L, 783C[1;1HLABEL=<<>>[17C/[23Cext3    defaults[8C1 1
    none[20C/dev/pts[16Cdevpts  gid=5,mode=620  0 0
    none[20C/proc[19Cproc    defaults[8C0 0
    none[20C/dev/shm[16Ctmpfs   defaults[8C0 0
    <<>>[15Cswap[20Cswap    defaults[8C0 0
    <<>>[14C/mnt/cdrom[14Ciso9660 noauto,owner,kudzu,ro 0  [7;1H0
    <<>>[16C/mnt/floppy[13Cauto    noauto,owner,kudzu 0 0
    <<>>[7C/saped[10Csmbfs   user,username=ar,password=ar1team,noautoo[10;1H  0  0
    #/dev/hdd[16C/mnt/dano[15Cext3    defaults[8C0 0
    <<>>[15C/mnt/team[16Cext3    defaults[8C0 0


    [?1l>[?25h[?1047l[?1048l]0;tlau@merlin:/home/tlau[root@merlin tlau]# mount <<>> mnt/dav no
    ]0;tlau@merlin:/home/tlau[root@merlin tlau]# cd <<>>
    ]0;tlau@merlin:/mnt/dano[root@merlin dano]# chmod 777 .
    ]0;tlau@merlin:/mnt/dano[root@merlin dano]# ls -al
    [00mtotal 24
    drwxrwxrwx    3 root     root         4096 Apr  1 16:17 [01;34m.[00m
    drwxr-xr-x    7 root     root         4096 Mar 22 18:16 [01;34m..[00m
    drwx------    2 root     root        16384 Apr  1 16:17 [01;34mlost+found[00m
    [m]0;tlau@merlin:/mnt/dano[root@merlin dano]# exit

    Script done on Thu Apr  1 16:55:05 2004
