# Dos.Boot-details -- Details of the ms-dos universal booting process

    MBR -- Master Boot Record 
    - First sector on drive. 
    - Contains partition table + Plus code to load Boot Record in partition 
     
    Partition Boot Record 
    - First sector on partition. 
    - Contains code to load init code (e.g. io.sys) 
     
     
     
     
     
    To  understand  BootIt lets first see how your  system  boot 
    process normally works then what BootIt Direct changes. 
     
    The  first  sector of your hard drive is called  the  Master 
    Boot  Record or MBR.  The MBR contains two things,  a  table 
    that  defines the partitions on your hard drive (called  the 
    partition  table) and the code to transfer  control  to  the 
    first  sector  (called  the  boot  sector)  of  one  of  the 
    partitions.   The  partition table can contain  up  to  four 
    entries.   Each  entry  contains information  on  where  the 
    partition  begins  and ends as well as a flag  to  mark  the 
    active  (or bootable) partition and the type of file  system 
    being  used  in the partition.  Here are some  of  the  more 
    common file system identifications: 
     
     ID          File System          ID           File System 
     01    DOS Primary 12-Bit FAT   12/0Ch   FAT32 - LBA 
           (1-15MB) 
     02    XENIX                    14/0Eh   FAT16 - LBA 
     03    XENIX                    15/0Fh   Extended FAT - LBA 
     04    DOS Primary 16-Bit FAT   99/63h   Unix 
           (16-32MB) 
     05    DOS Extended FAT         130/82   Linux Swap/Sun Solaris 
                                       h 
     06    DOS Primary Large FAT    131/83   Linux Native 
           (>32 MB)                    h 
     07    NTFS/HPFS                219/DB   Concurrent DOS 
                                       h 
     09    Coherent                 223/DF   BootIt EMBRM 
                                       h 
    10/0A  OS/2 Boot Manager        235/EB   BeOS 
      h                                h 
    11/0B  FAT32                              
      h 
     
     
    When  control is given to the code in the MBR, it will  look 
    at  each  of the four partitions for the one that is  marked 
    active.  It will load the boot sector of that partition  and 
    transfer control to the boot sector code. 
     
    Boot  sectors  are operating and file system specific.   The 
    area  for the code in the boot sector has just enough  space 
    to look for a certain file, read it in and transfer control. 
    The  name of the file is hard coded in the boot sector code. 
    For  instance, the boot sector for MS DOS will  look  for  a 
    file called IO.SYS. 
     
    There  is  a  special type of partition called  an  extended 
    partition.    This   type  of  partition  contains   logical 
    partitions  called volumes.  Each volume is  preceded  by  a 
    partition  table  in the same format as found  in  the  MBR. 
    This  table  contains  one  entry  for  a  "normal"  primary 
    partition and another extended partition entry that "chains" 
    to  the  next logical volume.  The last volume only contains 
    the primary partition information. 
     
    BootIt Direct works by replacing the MBR code on your  first 
    hard drive.  When you boot the system this new MBR code will 
    load  the rest of the BootIt Direct program.  BootIt  Direct 
    keeps its main program file (BOOTMNUD) on a FAT partition on 
    one of your hard drives. 
     
     
     
