(See Mac How to for setup)
# BUILD STEPS
## APP SPECIFIC SETUPS
### SPOTIFY
- Do lots of downloads
### PYCHARM
- Get Interpreters setup

## BROWSER SETUPS

### Safari
- Config -> Websites -> {{turn off requests for: notifications, camera, location}}


## FILESYSTEM SETUP


### Very out of date 2023-00-00

sudo mkdir /ob
chown oblinger /ob
mkdir /ob/bin
ln -s /ob/bin /Users/oblinger
ln -s /Users/oblinger/GoogleDrive/cmd /Users/oblinger
ln -s /Users/oblinger/GoogleDrive/cmd /ob
ln -s /Users/oblinger/GoogleDrive/logs /ob
ln -s /Users/oblinger/GoogleDrive/proj /ob
ln -s /Users/oblinger/GoogleDrive/topics /ob
chmod -R 755 /Users/oblinger/GoogleDrive/ob/cmd
xcode-select --install

cat >/Users/oblinger/.bash_profile

source /ob/bin/obrc

^D



## xx NOTIFICATIONS
CAL: everywhere
PHONE: phone, watch
SLACK: mention, 


### MAC SYS PREF --> Notifications
- ON FOR: Calendar, 


### IPHONE > SETTINGS
- CAL: Default Alert Times



## OS PARAMETER CONFIG
###  #build FINDER SETUP
#### Allow Quit Via ⌘ + Q
~# Finder: allow quitting via ⌘ + Q; doing so will also hide desktop icons
defaults write com.apple.finder QuitMenuItem -bool true

#### POSIX PATHS
!!!~# Display full POSIX path as Finder window title
defaults write com.apple.finder _FXShowPosixPathInTitle -bool true

#### Spring Loaded Directories
~# Enable spring loading for directories
defaults write NSGlobalDomain com.apple.springing.enabled -bool true

~# Remove the spring loading delay for directories
defaults write NSGlobalDomain com.apple.springing.delay -float 0

#### List View By Default
~# Use list view in all Finder windows by default
~# Four-letter codes for the other view modes: `icnv`, `clmv`, `Flwv`
defaults write com.apple.finder FXPreferredViewStyle -string "Nlsv"

#### Show Dot Files
      defaults write com.apple.finder AppleShowAllFiles TRUE
      defaults write com.apple.finder AppleShowAllFiles FALSE
      killall Finder

### List 1
~# Fast keyboard repeat rate
defaults write NSGlobalDomain KeyRepeat -int 0


~# Go back to the old power off menu
sudo defaults write /Library/Preferences/com.apple.loginwindow PowerButtonSleepsSystem false

~# Save screenshots to the desktop
defaults write com.apple.screencapture location -string "${HOME}/Desktop"

~# Save screenshots in PNG format (other options: BMP, GIF, JPG, PDF, TIFF)
~#defaults write com.apple.screencapture type -string "png"

~# Disable shadow in screenshots
defaults write com.apple.screencapture disable-shadow -bool true

### LIST 2
~# Display full POSIX path as Finder window title
defaults write com.apple.finder _FXShowPosixPathInTitle -bool true

~# Use list view in all Finder windows by default
~# Four-letter codes for the other view modes: `icnv`, `clmv`, `Flwv`
defaults write com.apple.finder FXPreferredViewStyle -string "Nlsv"

~# Makes default text larger
defaults write NSGlobalDomain AppleDisplayScaleFactor 1.5    ~# change back to 1.0 for std system


## COMMAND-LINE-BASED SETUPS
- > Do 'sudoers'. [[MAC How To#^sudo]]  
- > Do 'ssh-setup'    # not used these days....

## CONTROL PANEL  ???


ACCESSIBILITY
- Pointer Control
	- Trackpad Options
		- + Use Trackpad for dragging
		- 


OS: 
- 3 finger Drag, PowerNeverSleep
- 1Password, Google Drive, Airmail3, emacs
- AppStore purchases

## KEYBOARD
### 2017-??-??  Keybindings
  (not done on retina -- DO35, 2019nope)

~# File /Library/KeyBindings/DefaultKeyBinding.dict   (Created file and dir)
{
/* Remap Home / End to be correct :-) */
"\UF729"  = "moveToBeginningOfLine:";                   /* Home         */
"\UF72B"  = "moveToEndOfLine:";                         /* End          */
"$\UF729" = "moveToBeginningOfLineAndModifySelection:"; /* Shift + Home */
"$\UF72B" = "moveToEndOfLineAndModifySelection:";       /* Shift + End  */
}  # http://lifehacker.com/225873/mac-switchers-tip--remap-the-home-and-end-keys


"^h" = ("insertText:", "ctrl-h pressed");

~# KeyRemap4MacBook
~# DoubleCommand



##### Cocoa-keybindings -- Default.dict
- DETAILED GUIDE http://www.hcs.harvard.edu/~jrus/site/cocoa-text.html
- OS X DOCS      http://developer.apple.com/library/mac/#documentation/cocoa/conceptual/eventoverview/Introduction/Introduction.html


###### UNICODE NAMES
Unicode values for Apple non-letter keys:

Escape:	\U001B
 
Tab:	\U0009
Backtab:	\U0019
 
Return:	\U000A
Enter:	\U000D
 
Delete:	\U007F
 
Up Arrow:	\UF700
Down Arrow:	\UF701
Left Arrow:	\UF702
Right Arrow:	\UF703
 
Help:	\UF746
Forward Delete:	\UF728
Home:	\UF729
End:	\UF72B
Page Up:	\UF72C
Page Down:	\UF72D
 
Clear:	\UF739
 
F1:	\UF704
F2:	\UF705
F3:	\UF706
…	
F35	\UF726
 
Not on Apple keyboards:
 
Menu:	\UF735
Notes:

‘Backtab’ is ‘Shift-Tab’
The key labeled ‘Backspace’ on most PC keyboards is the ‘Delete’ key
The ‘Num Lock’ key on PC keyboards is ‘Clear’
The keys labeled ‘Print Screen’, ‘Scroll Lock’, and ‘Pause’ are respectively ‘F13’, ‘F14’, and ‘F15’
The key labeled ‘Insert’ is the ‘Help’ key
I’m not sure whether the ‘Menu’ key actually does anything, as I don’t own a keyboard with that key on it.

### 2017-??-?? Misc Setup
  (DO35 not done on retina)
---not used yet---
for use with keyremap4macbook

<?xml version="1.0"?>
<root>

    <item>
      <name>F5, F6, and F7 to iTunes music controls</name>
       <appendix>* F5 to Music Prev</appendix>
       <appendix>* F6 to Music Play/Pause</appendix>
       <appendix>* F7 to Music Next</appendix>
     <identifier>remap.pc_fkeys5672musiccontrols</identifier>
      <autogen>--KeyToConsumer-- KeyCode::F5, ConsumerKeyCode::MUSIC_PREV</autogen>
      <autogen>--KeyToConsumer-- KeyCode::F6, ConsumerKeyCode::MUSIC_PLAY</autogen>
      <autogen>--KeyToConsumer-- KeyCode::F7, ConsumerKeyCode::MUSIC_NEXT</autogen>
    </item>

    <item>
      <name>Right-hand function key block to system volume controls</name>
       <appendix>* PrintScreen to Volume Down</appendix>
       <appendix>* ScrollLock to Volume Up</appendix>
       <appendix>* Pause/Break to Volume Mute</appendix>
     <identifier>remap.pc_functionblock2volumecontrols</identifier>
      <autogen>--KeyToConsumer-- KeyCode::PC_PRINTSCREEN, ConsumerKeyCode::VOLUME_DOWN</autogen>
      <autogen>--KeyToConsumer-- KeyCode::PC_SCROLLLOCK, ConsumerKeyCode::VOLUME_UP</autogen>
      <autogen>--KeyToConsumer-- KeyCode::PC_PAUSE, ConsumerKeyCode::VOLUME_MUTE</autogen>
    </item>

</root>

### Set key repeat
      defaults write -g ApplePressAndHoldEnabled -bool false      # ON RETINA  DO35
      defaults write NSGlobalDomain KeyRepeat -int 0

### Docs on arbitrary key remapping
http://matthewpalmer.net/blog/2014/01/19/remap-keyboard-keys-caps-lock-os-x-mavericks/

### Add keyboard shortcuts to a single app (using OS)
     DOCS   http://www.mactipper.com/2008/02/add-keyboard-shortcuts-from-command.html


 $ defaults write com.apple.App NSUserKeyEquivalents '{"Menu Item"="@n";}'

we can make a keyboard shortcut for a lot of different apps. 
We change the com.apple.App to the name of the application (case sensitive) and we use these symbols near the end to represent the modifier keys that one would press:

• @ = Command
• $ = Shift
• ~ = Option
• ^ = Control

~# Add shortcuts for rating songs
defaults write com.apple.iTunes NSUserKeyEquivalents '{"None"="@0";"★"="@1";"★★"="@2";"★★★"="@3";"★★★★"="@4";"★★★★★"="@5";}'

CMD-1 is only key that is not working


## FINDER

### 2023-05-01 Setup
- TAGS: Use minus key to remove them
- MY DESK: Drag "My desk" folder into place on side bar

### FINDER: Favorites bar
      -- Select the MyDesk folder.  
      -- Press  #^T  to add to favorites, or
      -- Drag Folder Icon onto the Finder Favorites Bar to add it.
### FINDER: search should serach only current folder
finder -> #, -> Advanced -> "When Performing Search"  !"Search Current Folder"


### Create file associations
- Finder -> rt! file.xxx -> !!!GET-INFO!!! -> !open-with -> !other
  (select application to launch with)  -> ! CHANGE-ALL


## MAIL 

~# Copy email addresses as `foo@example.com` instead of `Foo Bar <foo@example.com>` in Mail.app
defaults write com.apple.mail AddressesIncludeNameOnPasteboard -bool false

#### SETUP PARAMAETERS TRIED ON 15" 
- OSX -> # L Pref -> Keyboard -> Shortcuts -> Services -> Shortcut -> Files and Folders -> ! CreateDocument
  
### MAIL -- DELETE2ARCHIVE  make delete key do an archive action

To get DEL key to archive:
- OSX -> Mail -> Mail -> Preferences -> 
  - Accounts -> MailBox Behavior -> Trash 
    uncheck "Store deleted messages on server"        <--------- the one that does it!
    ( "move deleted messages to trash"  should remain checked)

- GMAIL -> Setting -> Pop/IMAP settings -> 
  "WHEN MESSAGE IS MARKED AS DELETED" ->
     CHECK "Archive the message"

## HOMEBREW

### 2023-05-02 - Installation on 2023-M2

- Followed these instructions - https://setapp.com/how-to/install-homebrew-on-mac
	xcode-select --install
	/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
	brew analytics off
	brew update


### brew xx Pandoc. (2023-05-03 installed again)

~# https://medium.com/macoclock/how-to-setup-pandoc-and-latex-on-macos-mojave-8e18fa71e816 
brew install pandoc
brew install librsvg python 
brew install --cask basictex



### BREW SETUP
 --  Google "Homebrew"  then cut/paste line into terminal window
 --  install xcode licence as described, then rerun

BREW NOTES
     $ brew install xxxx    # Never as root
     $ brew unlink  xxxx

brew update
brew install python3 / emacs


### brew xx Jupiter

Jupiter Notebook:  https://medium.com/@blessedmarcel1/how-to-install-jupyter-notebook-on-mac-using-homebrew-528c39fd530f

### brew xx clojure
 $ brew install meld clojure

### BREW OLDER
#### brew older xx Utilities
	     ~# These listed with MySql install
	     $ brew install git cmake ack wget redis memcached libmemcached colordiff imagemagick
	       ~# DO35 removed 'brew install curl' since Retina has it
	       ~# DO36 added cmake since sql needed it.
     
#### brew older xx MySQL
	     ~# http://www.frederico-araujo.com/2011/07/30/installing-rails-on-os-x-lion-with-homebrew-rvm-and-mysql/
	     
	     $ brew install mysql
	     ~# configure it
	     $ mysql_install_db --verbose --user=`whoami` --basedir="$(brew --prefix mysql)" --datadir=/usr/local/var/mysql --tmpdir=/tmp
	
	     ~# DONT EXECUTE, but this would cause it to load on boot
	     $ mkdir -p ~/Library/LaunchAgents
	     $ cp /usr/local/Cellar/mysql/5.5.14/com.mysql.mysqld.plist ~/Library/LaunchAgents/
	     $ launchctl load -w ~/Library/LaunchAgents/com.mysql.mysqld.plist


	notes

	Installing MySQL system tables...
	OK
	Filling help tables...
	OK

	To start mysqld at boot time you have to copy
	support-files/mysql.server to the right place for your system

	PLEASE REMEMBER TO SET A PASSWORD FOR THE MySQL root USER !
	To do so, start the server, then issue the following commands:

	/usr/local/Cellar/mysql/5.5.15/bin/mysqladmin -u root password 'new-password'
	/usr/local/Cellar/mysql/5.5.15/bin/mysqladmin -u root -h Daniels-MacBook-Air.local password 'new-password'

	Alternatively you can run:
	/usr/local/Cellar/mysql/5.5.15/bin/mysql_secure_installation

	which will also give you the option of removing the test
	databases and anonymous user created by default.  This is
	strongly recommended for production servers.

	See the manual for more instructions.

	You can start the MySQL daemon with:
	cd /usr/local/Cellar/mysql/5.5.15 ; /usr/local/Cellar/mysql/5.5.15/bin/mysqld_safe &

	You can test the MySQL daemon with mysql-test-run.pl
	cd /usr/local/Cellar/mysql/5.5.15/mysql-test ; perl mysql-test-run.pl

	Please report any problems with the /usr/local/Cellar/mysql/5.5.15/scripts/mysqlbug script!

	k
	- Checkout sourcetree for GIT
	- FUSION: windows (see if I have XP install disks, or build small verison of my system??)
	- FUSION: Ubuntu

	-  VMware Fusion, TextMate?, 
	-  Trigger Words (google apps, using cmd line)
	-  AppleScript (learn it)
	-  Virus Scanner:  Intego VirusBarrier X6
	- QUESTIONS 
	  - Best GIT viewer
	  - Best visual SCP tool
	  - Best remote editor

#### brew older xx postgres
	  XXXXX decided to go to download Postgres.app directly   2015.05  Retina-yosemite


	  ~# first get brew & xcode
	  $ brew install postgres
	  $ echo 'export PATH=/usr/local/Cellar/postgresql/9.4.2/bin:$PATH' >> ~/.bash_profile
	  $ 
	  $ initdb /usr/local/var/postgres

#### brew older xx Mountain Lion Setup

	Apple removed GCC 4.2. We’ll just put it back where make expects it to be. First install the package manager Homebrew. Then in Terminal do:

	$ brew tap homebrew/dupes
	$ brew install apple-gcc42
	$ sudo ln -s /usr/local/bin/gcc-4.2 /usr/bin/gcc-4.2

	~# Need to install Qt
	$ brew update
	$ brew install Qt

#### brew older xx SSHFS
	     Download  OSXFUSE & SSHFS  from http://osxfuse.github.io/

## PYTHON

### 2023-11-17  CONDA - I now use conda for all python builds #build 


### 2023-05-02 Installation 
- Google "python download mac"
- I chose this:  https://www.python.org/downloads/

- Pandas, matplotlib, 

### OLD PYTHON
##### PYTHON SETUP 2019-07-21 - 15" rebuild
	xcode-select --install
	brew install python3
	~# in pycharm setting up new interpreter used venv under /uf by default
##### PYTHON SETUP 2018-10-27
Add required folders (owned by oblinger)
	      sudo su
	         cd /usr/local
	         mkdir include Frameworks
	         chown oblinger include Frameworks
	         chgrp admin Frameworks include
	         chmod 775 include Frameworks
	      ~# install python
	      brew install python    # this will brew update & xcode-select --install
      
##### xx KERAS 

	      ~# https://www.youtube.com/watch?v=LnzgQr14p7s

	python3 --version    # 3.8.1 (2021.04 installed 3.9)
	open https://docs.conda.io/en/latest/miniconda.html

#### xx PYTHON3 on OSX at system level
	- In running my _ fling.py script from keyboard maestro it ran some MAC OS install of python3  (I think this became a system level python3 instance)

##### brew xx python       (16.10.07 approach)
	     $ brew install python
	
	     $ which python
	       /usr/bin/python         # ships with OSX
	       /usr/local/bin/python   # symlink to brewed python
      
##### OLD SETUP  (QL-personal 2015.10)

	    ~# COMMAND-SPACE "XCODE"  (just verify that you can accept any agreements)
	    open a.xcodeproj
	
	    ~# GET BASE PYTHON FOR VIRTUAL ENV TO BUILD FROM
	    open http://python.org/download  # click "Download Python 2.7.10"
	
	    ~# Install EasyInstall (if needed)
	    curl https://bootstrap.pypa.io/ez_setup.py -o - | sudo python
	
	    ~# Install pip         http://stackoverflow.com/questions/17271319/installing-pip-on-mac-os-x
	    sudo easy_install pip
	
	    ~# Install Virtualenv
	    VER=Py2_7_10
	    sudo pip install virtualenv
	    sudo pip install virtualenvwrapper
	    echo 'export WORKON_HOME=~/Envs' >> ~/.bash_profile
	    echo 'source /usr/local/bin/virtualenvwrapper.sh' >> ~/.bash_profile
	    echo "workon $VER" >> ~/.bash_profile
	
	    ~# Create Virtual Env 
	    WORKON_HOME=~/Envs
	    mkdir -p $WORKON_HOME
	    source /usr/local/bin/virtualenvwrapper.sh
	    mkvirtualenv --python /Library/Frameworks/Python.framework/Versions/2.7/bin/python $VER
	    workon $VER
	    pip install virtualenv virtualenvwrapper      # install these *within* the new python env too
	
	    // Removes warning about missing SSL object
	    pip install pyopenssl ndg-httpsclient pyasn1
	
	    // INSTALL common modules
	    pip install openpyxl
	    pip install -U scipy 
	    pip install -U matplotlib
	    pip install -U scikit-learn
	    pip install -U pandas
	
	    pip install pyyaml
  
##### ipython & DataScience packages
	http://penandpants.com/2013/04/04/install-scientific-python-on-mac-os-x/

##### OLD -- Manual installs of python
	python  13.11.06
	http://www.python.org/download/releases/3.2.5/  downloaded the 3.2.5 32bit version

	SciPy Superpack 13.11.07 -- did not seem to work.
	https://github.com/fonnesbeck/ScipySuperpack

	B rew install packages 13.11.10 
	http://www.lowindata.com/2013/installing-scientific-python-on-mac-os-x/

	- python 2.7.5 installed but did not seem to link to usr/local
	- Use virtual-env burrito to install
	  https://github.com/brainsik/virtualenv-burrito
	  $ curl -s https://raw.github.com/brainsik/virtualenv-burrito/master/virtualenv-burrito.sh | $SHELL
	  ~# Just ignored this.  installed all in root env

	$ pip install numpy   # already installed
	~# Need and update
	  $ brew update
	$ brew install gfortran
	$ 

-----
	NumPy  -- http://sourceforge.net/projects/numpy/files/NumPy/
	NumPy 1.6.1 on by default.

	  $ sudo pip uninstall scipy
	  $ sudo pip install scipy==0.13.0

	  $ sudo pip install -U scikit-learn

	  ~# test   http://scikit-learn.org/stable/auto_examples/exercises/plot_iris_exercise.html



	import scipy
> > from scipy import linalg

	A = np.array([[1,2], [3,4]])
> > A
> ray([[1, 2],
	       [3, 4]])
> > linalg.inv(A)

##### OLD Anaconda

	- Loaded 64-bit Anaconda environment from http://continuum.io/downloads 
	  into /ob/pkg/Anaconda-iPython
	- /ob/bin/ipython will launch environment
	- ~/.ipython  configures the environment
	- Added ~/.ipython/profile_default/startup/do_ipython_setup.py

	ipython              # this will launch the ipython console
	ipython-launcher     # this will launch the ipython-launcher app
	~# l ipython-launcher  # also launches the launcher


# LOG
### 2023-05-01  2023-05-M2-Build

- APPSTORE - Install All
- BUILD STEPS
	- Filesystem Steps, Cmdline-based Setups
	- Keyboard, Finder, Mail, Home brew, Python


--> Waited on OS PARAMS

### 2016-??-?? - DID these on 15"... did they work?

	~# Disable the sound effects on boot
	sudo nvram SystemAudioVolume=" "

	~# Set highlight color to green
	defaults write NSGlobalDomain AppleHighlightColor -string "0.764700 0.976500 0.568600"

	~# Increase window resize speed for Cocoa applications
	defaults write NSGlobalDomain NSWindowResizeTime -float 0.001


	~# Copy email addresses as `foo@example.com` instead of `Foo Bar <foo@example.com>` in Mail.app
	defaults write com.apple.mail AddressesIncludeNameOnPasteboard -bool false

	????
	~# Disable press-and-hold for keys in favor of key repeat
	defaults write NSGlobalDomain ApplePressAndHoldEnabled -bool false
	~# ?????
	~# Set a blazingly fast keyboard repeat rate
	defaults write NSGlobalDomain KeyRepeat -int 10

	~# Stop iTunes from responding to the keyboard media keys
	~# launchctl unload -w /System/Library/LaunchAgents/com.apple.rcd.plist 2> /dev/null

	======



	~# PLUS MANY MORE AT
	~# https://github.com/mathiasbynens/dotfiles/blob/master/.osx

	~# Disable the sound effects on boot
	sudo nvram SystemAudioVolume=" "

	~# Disable transparency in the menu bar and elsewhere on Yosemite
	defaults write com.apple.universalaccess reduceTransparency -bool true



	~# Set highlight color to green
	defaults write NSGlobalDomain AppleHighlightColor -string "0.764700 0.976500 0.568600"

	~# Increase window resize speed for Cocoa applications
	defaults write NSGlobalDomain NSWindowResizeTime -float 0.001

	~# ?????
	~# Disable the “Are you sure you want to open this application?” dialog
	defaults write com.apple.LaunchServices LSQuarantine -bool false

	~# ????
	~# Disable smart quotes as they’re annoying when typing code
	defaults write NSGlobalDomain NSAutomaticQuoteSubstitutionEnabled -bool false
	~# ????
	~# Disable smart dashes as they’re annoying when typing code
	defaults write NSGlobalDomain NSAutomaticDashSubstitutionEnabled -bool false

	~# ????
	~# Disable press-and-hold for keys in favor of key repeat
	defaults write NSGlobalDomain ApplePressAndHoldEnabled -bool false
	~# ?????
	~# Set a blazingly fast keyboard repeat rate
	defaults write NSGlobalDomain KeyRepeat -int 0


	~# Stop iTunes from responding to the keyboard media keys
	~# launchctl unload -w /System/Library/LaunchAgents/com.apple.rcd.plist 2> /dev/null

	~# Save screenshots to the desktop
	defaults write com.apple.screencapture location -string "${HOME}/Desktop"

	~# Save screenshots in PNG format (other options: BMP, GIF, JPG, PDF, TIFF)
	defaults write com.apple.screencapture type -string "png"

	~# Disable shadow in screenshots
	defaults write com.apple.screencapture disable-shadow -bool true


	~# Enable HiDPI display modes (requires restart)
	sudo defaults write /Library/Preferences/com.apple.windowserver DisplayResolutionEnabled -bool true


	~# Finder: allow quitting via ⌘ + Q; doing so will also hide desktop icons
	defaults write com.apple.finder QuitMenuItem -bool true

	~# Finder: disable window animations and Get Info animations
	defaults write com.apple.finder DisableAllAnimations -bool true

	~# Automatically open a new Finder window when a volume is mounted
	defaults write com.apple.frameworks.diskimages auto-open-ro-root -bool true
	defaults write com.apple.frameworks.diskimages auto-open-rw-root -bool true
	defaults write com.apple.finder OpenWindowForNewRemovableDisk -bool true

	~# Use list view in all Finder windows by default
	~# Four-letter codes for the other view modes: `icnv`, `clmv`, `Flwv`
	defaults write com.apple.finder FXPreferredViewStyle -string "Nlsv"


	~# Copy email addresses as `foo@example.com` instead of `Foo Bar <foo@example.com>` in Mail.app
	defaults write com.apple.mail AddressesIncludeNameOnPasteboard -bool false




	~# Use plain text mode for new TextEdit documents
	defaults write com.apple.TextEdit -int 0
	~# Open and save files as UTF-8 in TextEdit
	defaults write com.apple.TextEdit PlainTextEncoding -int 4
	defaults write com.apple.TextEdit PlainTextEncodingForWrite -int 4

#### -->xx SPOTLIGHT COMMANDER<--
	     // Add app in /Applications
	     ln -s /Users/oblinger/GoogleDrive/ob/proj/spotlight-commander/SpotlightCommander.app /Applications
	     chmod -R 755 /ob/proj/spotlight-commander/SpotlightCommander.app
	
	     // Link 'cmd' in user folder
	     ln -s /Users/oblinger/GoogleDrive/ob/cmd /Users/oblinger
	
	
	     // Try running ctrl -x 
	     ./ctrl --install   # will add cmdEDIT etc.
	
	     >> CMD-T (open new terminal after installing .bash_profile
	     cmd --install
	
	     // Test CMD-T  cmdedit
	     // Test CMD-T  job

	FIXES
	 Use cmd --debug foo  # this test link

	 CHMOD -- use: chmod -r 755 /ob/cmd   # will set them executable

	 "app from unidnetified developer" -- 
	      Test target app by double-click (not command-line) if you get
	      an error this is why it won't load

#### -->xx MAIL<--
##### MAIL -- GENERAL SETUP
	      -- pref -> viewing -> Move Discarded Messages  !Archive
	              -> use classic        # this is required in order to change font size
	              -> font&color -> font-list !14   # sets 14pt font


	=== CONTACTS, CAL & MAIL ===  (as of 14.08)

	- Settings -> Mail/Contacts/Calendar
	  - Gmail:  sync mail,contacts,calendar,messages,notes
	  - iCloud: sync reminders, safari, findmyphone
	  - AF:     sync gmail calendar

	- Settings -> iCloud
	  - > Account    AppleID = wef234@gmail.com
	                Mail    = daniel.oblinger@icloud.com
	  Check:  Reminders & Safari & FindMyI_phone
	  UNCHECK: ALL ELSE

##### MAIL -- ACCOUNTS  as of 2017-02-05

	=== icloud === oblinger@gmail.com            
	    Checked: icloud-drive(all checked)   Photos (unchecked icloud photo library)  
	    Contact  Calendars  (NOT MAIL)  Reminders   Safari   Notes  Keychain  (not back to mac)  FindMyMac

	=== Exchange === dan.oblinger@aeolusbot.com  (name Daniel Oblinger)  
	    Checked  MAIL  CALENDAR

	=== Google === dan.oblinger@aeolusdev.com
	    Checked  CALENDARS

	=== Google === oblinger@gmail.com            (name Dan Oblinger)  
	    Checked  MAIL  CALENDARS  MESSAGES

	=== Messages === wef234@gmail.com            (account type Jabber)

	XXX Google === dan@anayticsfire.com         (name D AF)  
	    checked CALENDARS

#### -->xx MAIL/CAENDAR<--
####### CALENDAR OVERVIEW

	Calendar -- 
	-- use iCloud 'calendar' as default.  
	-- delete all others "on my mac" or other "iCloud" calendars
	-- include calendars for: oblinger@gmail.com  dan@analyticsfire.com  ??martain?? 

####### Delete Key -->  Archive Key
	   http://thingsofinterest.com/delete2archive/
   
	   Stopped Mail
	   Copied bundle into ~/Library/Mail/Bundles

####### Removing entries from autofill
	   Working with PREVIOUS RECIPIENTS
	- Mail -> Window -> Previous Recipients -> [[remove email address from list]]

####### Sending mail from multiple addresses
	AppleMailApp>Preferences>Accounts>Account Information>Email Address
	    - Add list of all email addresses (SEPARATED BY COMMAS)
	    - Restart Mail then send message to see dropdown
	
	    oblinger@gmail.com,dan@analyticsfire.com,dan@oblinger.us,dan@martianbots.com

> > SETTING THE DEFAULT SEND TO ADDRESS
	    - AppleMail > View > Show Mailbox List 
	    - ! InBox
	      - ((Reorder list of in boxes to put default box at top))

####### CONTACTS Setup



	=== HIGH LEVEL ===   (6/13 on retina; 6/15 updated)
	- Using gmail contacts as master
	- Using empty local contacts book on mac (ensure default addr book is google)

	========
	- SPOTLIGHT "contacts"
	  -> {Delete all contacts on local mac}
	  -> Accounts -> !"+" -> Add CardDav account for oblinger@gmail.com
	  -> General -> DefaultAccount=Google

####### Key for archive to "_in"
######## New Method
	- Ensure _in2 is the FIRST favorites button (in the favorites bar at the top)
	- Create a Keyboard Maestro macro that maps #+'-->' to ^#1
	  (this is in the menus as move to first favorites)
	  -- be sure that this keyboard macro is created in the Apple Mail group which always runs
	     for the mail.app application.

######## Old method
	- pref -> keyboard shortcuts -> !"+"
	  !ApplicationName = "mail.app"
	  !title="_in"
	  ! # + "->"

	(DO35 not on retina)
	http://email.about.com/od/macosxmailtips/qt/How-To-Add-A-Keyboard-Shortcut-For-Archiving-In-Mac-Os-X-Mail.htm


	To set up a keyboard shortcut for moving mail to the "Archive" in Mac OS X Mail:

	Select Apple | System Preferences from the menu.
	Open the Keyboard category under Hardware.
	Go to the Keyboard Shortcuts tab.
	Now select the Application Shortcuts category.
	Click +.
	Select Mail under Application:.
	Type "Archive" (not including the quotation marks) under Menu Title:.
	Click in the Keyboard Shortcut: field.
	Press the key combination you'd like to use for archiving.
	You can use Command-Shift-O, Command-Down Arrow, Command-Alt-M or Command-Control-S, for example.
	Click Add.
	Close the Keyboard preferences.
	Note that, if one of your folders is called "Archive" and you have made it a favorite,
	the keyboard shortcut will apply to this folder instead. 
	It should still work, provided you use the "Archive" folder for archiving.

#### -->xx DOCS<--   NOTES
	https://github.com/mathiasbynens/dotfiles/blob/master/.osx
	    https://github.com/mathiasbynens/dotfiles/blob/master/.osx 
	http://www.mactech.com/articles/mactech/Vol.09/09.08/ScriptAppExample/index.html 
	http://secrets.blacktree.com/
	http://apple.stackexchange.com/questions/400/please-share-your-hidden-os-x-features-or-tips-and-tricks?page=2&tab=votes#tab-top
	tricking out your mail client   http://getpocket.com/a/read/406605693
	Crypto email    https://gpgtools.org/gpgmail/

#### -->xx DEVICE<--
##### AIRPORT
	          (not done on retina)
	       $ sudo ln -s /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport /usr/sbin/airport
	       $ airport -I
	       // DB values
	       $ while x=1; do /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I | grep CtlRSSI; sleep 0.5; done
	
	       // Associate w a network
	       $ sudo networksetup -setairportnetwork en0 Oblio obliooblio

##### POWER
	pmset -g assertions    # the key PreventUserIdleSystemSleep is '1' if something will stop sleeping

	sudo nvram boot-args=iog=0x0   # CAREFUL.  don't know how to undo.
	                               ~# system will sleep in 'clamshell' mode when lid closes.  internal display will also shutdown

	~# Test is clamshell is closed
	ioreg -r -k AppleClamshellState -d 4 | grep AppleClamshellState  | head -1
#### -->xx APPLED-APPS<--
	TEXT EDIT
	$ defaults write com.apple.TextEdit RichText 0
##### i-CAL 
####### Remove entries from autofill
	- ((Remove entries w. bad email addresses in them.))  ((Quit iCal.))
	- Finder -> Retina -> Users -> Oblinger -> Library -> Caches 
	  -> com.apple.iCal -> Previous Recpients.plist

	     -> Root -> Calendar Google.com -> [remove bad email address]
##### i-Tunes  (See music; See How-To->Keyboard below)

#### -->xx SCREEN SHARE<--...
	-- Ensure account is logged into iCloud (in preferences)
	-- Have parents share to ME
        
#### -->xx SCREENSHOT<--
	     defaults write com.apple.screencapture location /ob/data/MyDesk

#### -->xx BUGS<--
##### OSX Memory compression bugs
	~# Notester is running slow. so I disabled memory compression to see if that is the issue
	~# http://superuser.com/questions/668114/disable-compressed-memory-in-mac-os-10-9-mavericks
	$ sysctl -a vm.compressor_mode
	vm.compressor_mode: 4                       # Standard value
	$ sudo nvram boot-args="vm_compressor=1"
	  ((Reboot))
	$ sysctl -a vm.compressor_mode

### 2016-??-??  xx RUBY + RVM + xx SINATRA

2015-09-09 -- install RVM  (from http://usabilityetc.com/articles/ruby-on-mac-os-x-with-rvm/)

  ~# INSTALL RUBY INSIDE RVM INSIDE $HOME/.rvm
  brew install gnupg
  gpg --keyserver hkp://keys.gnupg.net --recv-keys 409B6B1796C275462A1703113804BB82D39DC0E3
  \curl -sSL https://get.rvm.io | bash -s stable --ruby
     //==> should have added line to ~/.bash_profile  and ~/.bashrc

  ~# Reopen terminal
  rvm use 2.2.1         #==> or use which ever version is the lastest
  gem install bundler 

  rvm list      # shows versions.  Shows available ruby environments (use downloaded version in command above)
  which ruby    # shows location.  Should be in your home folder under .rvm
  ruby -v       # shows version.   Should be the loaded version
  gem env       # shows gems.      Shows gems loaded into current environment
     
  gem install sinatra




