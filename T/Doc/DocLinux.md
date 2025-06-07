  [__DocLinux__](__DocLinux__.md)

[[DocTmux]] 
[[DocNvim]] 

# tmpnotes

DOCS:  $ exec  % asRoot  # comment >> append to file  
       -> next step  { do this stuff }  ! click,  "type-this  #> system should print this

pbcopy
nc -vv -l 12345
curl -v -d a=1 -d b=2 http:

xwin

http://stackoverflow.com/questions/4458800/how-to-use-one-instance-of-emacs-as-the-default-text-editor-linux

http://ubuntuforums.org/showthread.php?t=299086

https://mail.google.com/mail/?hl=en&tab=wm#contacts

REGEX  http://www.cheatography.com/davechild/cheat-sheets/regular-expressions/

# tmp notes:
sudo adduser USER
? how to sudo to user?
? how to set user so it can sudo su?

install conky nautilus  apcalc
google-chrome eclipse   1.25  2.46212
  
Total Founder Equity 

 P = 1.25% * 15 + %2.46212


# UNIX Console
## Control
   $ kill -9 PID     # kill process by PID (-9=force)
   $ killall
   $ pkill name      # kill by name
   $ chmod -R 755 files
   $ chgrp -R name files
   $ chown -R webmaster:rails ~rails/.ssh
   $ pidof python | xargs kill              # pidof looks are argv[0]

### FILE TREE OPERATIONS

RSYNC -- Incremental remote file copy.  partial list of options

   $ rsync SRC DESTDIR
      -a   ARCHIVE tries to preserve mod times and permissions, (is recursive, etc.
      -v   VERBOSE 
      -r   DRY_RUN    just shows list of updated files


   $ rsync -av /ob/proj   /Volumns/Stub/ob      # updates backup


  

## Other/twiddles
### script rec.log    # records shell commands to rec.log
### find   -- 
   # scan files that match the first and contain the second
   $ find . -name \*\.py -exec grep xslx {} \;
   $ find . -name '*.org' -print 
   
   -name -iname  (filename pattern, case insensitive pattern
   -path -ipath  (matches against whole path)  regex ??
   -dmin -dtime  (modified min, or days ago)
   -prune        (ignore files that match)
   -size
   -type d (dir) f (regular file)
   \! u (not)
ACTION
   -ok touch '{}' \;       (prompt user)
   -delete  
   -exec touch '{}' \;       (Execute command.  '-ok' will prompt first)
   -print -fprintf ~/log "..." 
      \a alarmBell  \c flushoutput  \f ff  \n \r \t \\ %%   \NNN (octal)
      %p path(full) %f filename(base)  %a accesstime  %c change
      
### grep [options] pattern [file ...]
   $ find . -exec grep -l 'Left' '{}' \;
   OUTPUT:  -c (count)   -l -L (filename. invert)  


### screen -- Disconnectable console screen manager
   $ screen -S name     # Creates and opens a named session
   $ screen -list       # Lists active sessions
   $ screen -r [name]     # Re-attach to named session
   $ screen [-d|detach if needed] [-r screen|reattach] [-R ...or create]


   COMMANDS:  (Press ctrl-A then...)
     c    CREATE new console
     n    NEXT console
     p    PREVIOUS console
     "    LIST available consoles
     ESC  scroll up/down in console
     d    DETACHES console


   # Remove stupid copyright notice
   $ gksudo gedit /etc/screenrc   # remove copyright notice
   > # startup_message off         # remove '#' character

## GUI
### rxvt -- xterm
## Networking
### nc -vv -l 12345    # listen in port 12345
### scp user@host:file user@host:file
    $ scp user@host:file user@host:file
    $ scp user@128.92.52.33:file user@host:file
### nslookup www.hostname.com
### curl -v http://localhost:12345      # post to 
    curl -v -d a=1 -d b=2 http://localhost:12345      # post to
### nc -vv -l 12345   # Listen on port 12345

## Scripting
   $ tee [-a|append] FILE
xwininfo, xdotool, xprop
# UNIX Sys Admin
## Administation
### Startup
    $ bum      # x-windows
    $ rcconf   # command line
### Debian Packages
#### apt-get
     $ sudo apt-get update        # 'updates' package indicies from repositories
     $ sudo apt-get upgrade       # installs new versions of all packages already loaded
     $ sudo apt-get install PKG   # installs named package
     $ sudo apt-get remove  PKG   # removes pkg (but leaves config files)
     $ sudo apt-get purge   PKG   # - also removes pkg config files

   $ sudo apt-get update         #

#### dpkg -- pkg manager for Debian
     $ dpkg -b --build   
     $ dpkg -c --contents  
     $ dpkg -I --info
     $ dpkg -f --field
  
     $ cd ~/Downloads
   
     $ sudo dpkg -i google-chrome-*.deb
## Security
#### SSH stuff
##### Notes
      - logon w/o password (if you .pub key is in authorized_key)


     $ ssh-keygen -t dsa     enter enter enter
       -> ~/.ssh/id_dsa.pub
     $ cat .pub  >> ~/.ssh/authorized_keys  # on remove machine
     $ chmod autkeys 600 ~/.ssh/authorized_keys
     $ chmod 755 ~/.ssh
## ssh w/o login
   mkdir ~/.ssh
   (cat  ~/.ssh/id_rsa.pub  TO-TARGET ~/.ssh/authorized_keys)
   chmod 600 ~/.ssh/authorized_keys
   chmod 700 ~/.ssh

#### SUDO stuff
     $ sudo su        # Root shell
     $ sudo cmd       # 
     $ gksudo CMD     # Run GUI app as root
     $ sudo visudo    # Update user so 
       >> jerome ALL=(ALL) NOPASSWD: ALL

SUDOERS -- Add 'oblinger' to sudoers
 $ emacs /etc/sudoers
   # M-x toggle-read-only
   # Look for root ALL=(ALL) ALL
   # ADD LINE BELOW (maybe it requires TABS ???)
     oblinger ALL=(ALL) NOPASSWD: ALL

##### sudo                # Execute as root
   $ sudo XXX    
   $ sudo -U

##### adduser USERNAME    # Adds a user
##### visudo              # Edits list of user privledges
   $ visudo             # >>>> MUST HAVE A __TAB__ AFTER 'USER'
                        # first allows sudo, second allows w/o password

   user ALL = (ALL) ALL
   user ALL = (ALL) NOPASSWD: ALL

   User_Alias     FULLTIMERS = millert, mikef, dowdy

## Netorking
### ifconfig wlan0 down
   $ ifconfig                # shows existing adapters
   $ ifconfig wlan0 down     # sends adapter down
### iwconfig wlan0            # tells us about the wireless 
### hostname -F /etc/hosts
   # Gets or sets the hostname of this system
    
## Storage
### fdisk / mkfs / mount 
- fdisk -l                   # List disks
- fdisk /dev/hda             # Interactive partitioning cmd
    m - print help
    p - print the partition table
    n - create a new partition
    d - delete a partition
    q - quit without saving changes
    w - write the new partition table and exit

mkfs.ext3 /dev/sdb1        # Format drive
mount /dev/sdb1 /mnt/data  # Mount drive 
emacs /ect/fstab
     /dev/sdb1  /mnt/data ext3 defaults 1 2 
e2label /dev/sdb1 /theName # Label drive

### dd   (copy partition)   
    $ dd if=/dev/hda  of=- | ssh sarang cat > file

## System Files
### /
    /boot     # where the kernel is kept
    /etc      # config files
    /bin      # core programs for linux
    /usr/bin  # programs used by users
    /sbin     # also /usr/sbin 
    /usr      
    /doc
    /home /root
    /dev  /mnt
    /var  : /log /mail /spool

### /etc/hosts  -- mapping from IP addresses to URLs
### /etc/fstab
   <FileSystem>  <Mount>  <Type>  <Options> <Dump> <Pass>
       FileSystem:  /dev/hda2   # Use: 'sudo fdisk -l' to find device name
       Mount        /mnt/xxx    # Note: empty directory should be at '/mnt/xxx'
       Type         # ext3 ext4 ntfs auto
       Options      # noauto,user,noexec,ro,rw,  defaults
       Dump         # 0=NoBackup
       Fsck         # 0=Dont Check w. Fsck
  $ sudo mount -a   # reload the /etc/fstab table

# UNIX SETUP

# UNIX Monitoring
  (See http://www.cyberciti.biz/tips/top-linux-monitoring-tools.html)

  $ top     -- TOP processor usage
  $ vmstat  -- Sys activity, HW, Sys info
  $ w       -- who is logged in & what are they doing
  $ uptime  -- How long has the system been running
  $ ps      -- List PROCESSES    # also pstree
  $ free    -- Free MEMORY.  Memory usage.
  $ iostat  -- Avg CPU & disk IO
  $ sar     -- Sys activity
  $ mpstat  -- Multiprocessor Usage
  $ pmap    -- Process memory usage
  $ netstat -- Net statistics (also see ss)
  $ iptraf  -- Real time net stats
  $ tcpdump -- Detailed netowrk traffic
    # tcpdump -i eth1 'udp port 53'
  $ STRACE  -- Kernel Stats
  $ /proc
  $ Nagios  -- Server and Net monitoring
  $ Cacti   -- Web-based mon toolel
  $ -- KED system guard   <-- more powerful of the two
  $ -- Gnome System Monitor

  $ nmap -- open ports
  $ lof  -- open files
  $ ntop -- like top for net
  $ conky -- another mon for x
  $ vnstat -- hourly log of top
  $ mtr  -- trace route & ping


## Storage
   $ df        # usage by partition
   $ du -s .   # usage by folder

## Internet Tools


# UBUNTU Setup&Pkgs
## Apps

GNOME CONFIG:  gconf-editor, compiz_config, dconf-tools, alacarte
               gnome-panel

DEV:         emacs, eclipse, 
CONSOLE:     spell, screen, git, 
SYS APPS:    synaptic, gparted
PIM:         gnome-do(from startmenu)

Start Menu:  meld, gnome-do

gnome-panel

 $ sudo apt-get update
 $ sudo apt-get upgrade
   ????git-gui  (git-core, git-docs???)

## App Details
#### Firefox
#### Chrome
 # Google:  'chrome on ubuntu'
 $ firefox "http://www.google.com/chrome?platform=linux"
   ! download  ! 32-bit deb
 $ cd ~/Downloads
 $ sudo dpkg -i google-chrome-*.deb
   [if you get a dependency problem then exec line below]
 $ sudo apt-get install libnspr4-0d libnss3-1d libxss1 libcurl3

#### Nautilus
##### Change default file assocaitions
      # Global default can be found in /usr/share/applications/default.list
      $ emacs ~/.local/share/applications/default.list >>
        [Default Applications]
        text/plain=emacsclient.desktop
        text/html=emacsclient.desktop
        text/xml=emacsclient.desktop

##### Use emacsclient
    $ nautilus 
    !rt some-file -> properties -> open with -> add -> use custom command
    "  emacsclient -n 
    ! add
   
#### Gnome-Terminal
- gnome-terminal -> ! [screen menubar] -> Edit -> Profile Preferences
  - Title&Command -> When command exits  ! [Hold the terminal open]
  - Color -> !- [Use colors from system theme]
  - Scrolling -> Scrollback = 10000


#### Emacs
##### Setting EmacsClient as default editor (for Nautilus)
      $ cd ~/.local/share/applications
        defaults.list << '[Default Applications]\\text/plain=emacsclient'

      $ pkill nautilus
      # nautilus
        screenbar -> edit -> preferences => File Management Preferences
          ! single click to open   Executable Text Files -> ! View Exe...


---idea
remove (start-server) from .emacs
use emacsclient "$@" -a "" to start emacs whether it is running or not
If you save emacsclient "$@" -a "" as a script e.g. in editor file then editor -c creates a new frame, editor -t opens new frame in the terminal, and editor FILE visits FILE in an existing frame. It starts emacs server if it is not running.
---

## Installation
### Build USB Installer
### From Web
- Google 'ubuntu download'
- download image
- insert USB stick
- run System -> Administration -> Startup Disk Creator
  - ! Other -> {select .iso file} -> {select USB drive}
- REBOOT, and change bios options load from USB stick
  - !Install Ubuntu
  - {get onto internet during install}

### Core Oblio Install
GNOME-TERMINAL
 - !UbuntuSymbol -> OtherApps -> ``terminal'' -> {drag to icon bar}

INSTALL-OBLIO
 - Gnome-Terminal -> 
   $ mkdir /mnt/udrive
   $ /mnt/udrive/oblio/bin/install-oblio

### Configuring The Environmemt
![Ubuntu Gear] -> SystemSettings
 Screen -> Brightness -> 
   !- "Dim screen to save power"
   TurnOffAfter:     ! 1Hour
   LockScreenAfter:  ! 1Hour
 Mouse -> TouchPad -> ! disable scrolling
 Power -> When lid is closed -> !DoNothing  !DoNothing
       -> When power is critically low -> Hibernate

 # SUDOERS -- Add 'oblinger' to sudoers
 $ sudo visudo
    >> oblinger ALL=(ALL) NOPASSWD: ALL

#### Gnome
##### Keyboard Shortcuts
###### gconf-editor
  $ sudo apt-get install gconf-editor
  $ gconf-editor
  ! apps -> meta-city -> global_keybindings

###### Key names
<Control> <Alt> <Shift> <Super> (win key)
<Up> <Down> <Left> <Right>

Ensure the value is entered by clicking a different entry
 (if you don't, the command may disappear as you change to keybinding_commands)

###### xmodmap
   xmodmap -grammar            # Prints mgs below
   
###### xmodmap -- CapsLock
use xmodmap -e "..."  to test each of the lines below.

 >> ~/.xmodmap                 # Comments may not be allowed in .xmodmap
remove Lock = Caps_Lock        # Removes functionality of capslock
remove Shift = Shift_L
keycode 66 = Shift_L           # Makes key-66 become Shift_L
keycode 50 = Caps_Lock
add Lock = Caps_Lock           # Adds functionality
add Shift = Shift_L

remove Lock = Caps_Lock
keysym comma = comma less


###### Notes key shortcuts
 rc.xml  ???


<keybind key="A-F1">
    <action name="execute"><execute>gnome-panel-control --main-menu</execute></action>
</keybind>
<keybind key="A-F2">
    <action name="execute"><execute>gnome-panel-control --run-dialog</execute></action>
</keybind>

<menu id="root-menu">
  ...
  <item label="Log out">
    <action name="Execute"><execute>gnome-session-save --kill --gui</execute></action>
  </item>
</menu>

  <mousebind button="Right" action="Press">
      <action name="ShowMenu"><menu>root-menu</menu></action>
    </mousebind>

##### Command Line

###### gnome-terminal
   --command
   --tab
   --title 
   --hide-menu-bar
   --geometry=RxC+x+y
   --zoom 1.0
   --working-directory

## Configuration
### Locations
#### ~/.config
     monitors.xml    # sepeate tool

# w500 System
## Displays
### Config
    ~/.config/monitors.xml
    $ xrandr --output VGA-0 --mode 1920x1200 --rotate left --left-of LVDS
    $ xrandr --output LVDS --pos 1200x720
    $ xrandr --output VGA-0 --off

# Misc
#### SMB (samba client)
smbd  <-- 
g
service smb start

smbclient -L sarang
smbclient \\\\sarang\\oblinger
/etc/samba/smb.conf

smbpasswd  

#### REGULAR EXPRESSIONS 
     A regular expression is a pattern that describes  a  set  of 
     strings.  Regular expressions are constructed analogously to 
     arithmetic expressions, by using various operators  to  com- 
     bine smaller expressions. 
 
     Grep understands two different versions of  regular  expres- 
     sion  syntax:   ``basic''  and  ``extended.''   In GNU grep, 
     there is no  difference  in  available  functionality  using 
     either  syntax.   In  other  implementations,  basic regular 
     expressions are less powerful.   The  following  description 
     applies  to  extended  regular  expressions; differences for 
     basic regular expressions are summarized afterwards. 
 
     The fundamental building blocks are the regular  expressions 
     that  match  a single character.  Most characters, including 
     all letters and digits, are regular expressions  that  match 
     themselves.   Any  metacharacter with special meaning may be 
     quoted by preceding it with a backslash. 
 
     A list of characters enclosed by [ and ] matches any  single 
     character  in  that list; if the first character of the list 
     is the caret ^ then it matches  any  character  not  in  the 
     list.   For  example,  the  regular  expression [0123456789] 
     matches any single digit.  A range of ASCII  characters  may 
     be  specified  by  giving  the  first  and  last characters, 
     separated by a hyphen.  Finally, certain  named  classes  of 
     characters  are  predefined.   Their names are self explana- 
     tory,  and  they  are   [:alnum:],   [:alpha:],   [:cntrl:], 
     [:digit:],   [:graph:],   [:lower:],  [:print:],  [:punct:], 
     [:space:],  [:upper:],   and   [:xdigit:].    For   example, 
     [[:alnum:]]  means  [0-9A-Za-z],  except  the latter form is 
     dependent upon the ASCII  character  encoding,  whereas  the 
     former  is portable.  (Note that the brackets in these class 
     names are part of the symbolic names, and must  be  included 
     in  addition  to  the brackets delimiting the bracket list.) 
     Most metacharacters lose their special meaning inside lists. 
     To  include  a  literal ] place it first in the list.  Simi- 
     larly, to include a literal ^ place it anywhere  but  first. 
     Finally, to include a literal - place it last. 
 
     The period . matches any single character.  The symbol \w is 
     a   synonym   for  [[:alnum:]]  and  \W  is  a  synonym  for 
     [^[:alnum]]. 
 
     The caret ^ and the dollar sign $  are  metacharacters  that 
     respectively match the empty string at the beginning and end 
     of a line.  The symbols \< and  \>  respectively  match  the 
     empty string at the beginning and end of a word.  The symbol 
     \b matches the empty string at the edge of a  word,  and  \B 
     matches  the empty string provided it's not at the edge of a 
     word. 
 
     A regular expression matching a single character may be fol- 
     lowed by one of several repetition operators: 
     ?    The preceding item is  optional  and  matched  at  most 
          once. 
     *    The preceding item will be matched zero or more times. 
     +    The preceding item will be matched one or more times. 
     {n}  The preceding item is matched exactly n times. 
     {n,} The preceding item is matched n or more times. 
     {,m} The preceding item is optional and is matched at most m 
          times. 
     {n,m} 
          The preceding item is matched at least n times, but not 
          more than m times. 
 
     Two regular expressions may be concatenated;  the  resulting 
     regular  expression matches any string formed by concatenat- 
     ing two substrings that respectively match the  concatenated 
     subexpressions. 
 
     Two regular expressions may be joined by the infix  operator 
     |;  the  resulting  regular  expression  matches  any string 
     matching either subexpression. 
 
     Repetition takes precedence  over  concatenation,  which  in 
     turn  takes precedence over alternation.  A whole subexpres- 
     sion may be enclosed in parentheses to override  these  pre- 
     cedence rules. 
 
     The backreference \n, where n is a single digit, matches the 
     substring   previously  matched  by  the  nth  parenthesized 
     subexpression of the regular expression. 
 
     In basic regular expressions the metacharacters ?, +, {,  |, 
     (,  and  )  lose  their  special  meaning;  instead  use the 
     backslashed versions \?, \+, \{, \|, \(, and \). 
 
     In egrep the metacharacter  {  loses  its  special  meaning; 
     instead use \{. 
 
 
 
 
 
 
 
 
 
 
 








