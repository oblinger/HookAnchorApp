# SETUP
## = NEW COMPUTER - How to sync my setup a new system
- PORT OB TREE TO NEW SYSTEM. (See boot_install for latest)
  USB-C cable & "T" target mode
- Copy ~/ob onto the new machine at /Users/oblinger/ob
- Copy ~/Library/Application Support/BetterTouchTool    (ensure BTT is not running)
- Copy ~/.config/karabiner/karabiner.json to the new system
- xcode & xcode-select --install
- Keyboard Maestro !file stop then start sync of ~/ob/data/KeyboardMaestro-Master/Keyboard Maestro Macros.kmsync
- Ensure ~.zprofile is includes "source ~/ob/bin/obrc"
- Ensure prefs->users->login items-> /Users/oblinger/ob/bin/boot

- if 'km' script does not work, create a new github token for this machine



# HOW TO

## = NOTIFICATION SOUNDS

### Different Sounds on Laptop
- Mail - Biing
- Messages - Dinggggggg (tiny bell)
- WhatsApp - Caa-Lung  (two-syllable hitting wood sound)
- LinkedIn - 
- Gmail - 
## = How to use command line
 [AWESOME COMMAND LINES](https://github.com/herrbischoff/awesome-osx-command-line) 
 
## = KEY CODES - How to find key code numbers

- [[key code]], 

## = CHROME PROMPTS - How to block prompt for notifications and location on chrome
chrome://settings/content/notifications 

Here’s a quick guide to taking back control of Chrome.

Click Chrome > Preferences, or just paste chrome://settings/content/notifications into your browser to skip steps 2-4.
Scroll down and click Advanced
Click content settings
Click Notifications
Next to the Ask before sending (recommended) text, click the toggle button. It should now say Blocked.
## = SUDO - How to setup SUDO w/o PASSWORD (xx sudoers) 

^sudo

  export VISUAL=nano
  sudo visudo     # MUST USE VISUDO (no other editor)

  {[ CHANGE THIS ]}
  ~# User privilege specification
  root    ALL=(ALL) ALL 
  %admin  ALL=(ALL) ALL

  {[ TO THIS ]}
  ~# User privilege specification
  root    ALL=(ALL) ALL
  %admin  ALL=(ALL) NOPASSWD:ALL


^o ^x   (this writes and exits)
## = FAST WAKE - How to setup "fast wake" on osx
  pmset -g   # To see original settings
  sudo pmset -b standbydelay 43200        # how long sleeping before switching memory to disk
  sudo pmset -c standby 0


Active Profiles:
Battery Power		-1*
AC Power		-1
Currently in use:
 standbydelay         43200
 standby              1
 halfdim              1
 hibernatefile        /var/vm/sleepimage
 darkwakes            0
 gpuswitch            2
 disksleep            10
 sleep                60 (sleep prevented by storedownloadd, storedownloadd)
 autopoweroffdelay    14400
 hibernatemode        3
 autopoweroff         1
 ttyskeepawake        1
 displaysleep         5
 acwake               0
 lidwake              1
magi@duality ~> pmset -g
Active Profiles:
Battery Power		-1
AC Power		-1*
Currently in use:
 standbydelay         4200
 standby              0
 womp                 1
 halfdim              1
 hibernatefile        /var/vm/sleepimage
 darkwakes            1
 gpuswitch            2
 networkoversleep     0
 disksleep            10
 sleep                0 (sleep prevented by storedownloadd, storedownloadd)
 autopoweroffdelay    14400
 hibernatemode        3
 autopoweroff         1
 ttyskeepawake        1
 displaysleep         5
 acwake               0
 lidwake              1
## = WEB-PAGE-APPS - How to create standalone web-page apps 

     create Sandboxed Chrome Apps -- create app
     Like FluidApp but for Chrome.  See  https://gist.github.com/demonbane/1065791
     INSTALL:  cut/paste gist; chmod a+x /ob/bin/makeapp

  > feedly notes right below <

  ~# FIND AND CONVERT .png or .icns FOR .png ICON FILE
  $ chrome image.google.com  ((search for .png file for icon))  
    ! --> (copy to /ob/bin2)
  ~# CONVERT
  $ sips -s format png icon_file.icns --out png_file.png      ~# Read more at http://www.simplehelp.net/2010/10/08/how-to-convert-icns-files-to-png-files-in-os-x/#o5BJwxJIVWO16hl1.99
  
  ~# READY CHROME -- close all Chrome browsers
  $ open http://cnn.com   http://gmail.com  about://
    ! --> log BROWSER into wef234@gmail.com

  ~# CREATE
  $ /ob/proj/bin2/makeapp  
     ~# Use lowercase w/o spaces in name
     ~# saves to /Application 
  $ mv /Application/...app  /ob/proj/bin2


- REMOVE --app="..." to chrome app still have its tab bar
- FORCE BACKGROUND TAB   (add this extension to only the F-eedly app.
  check F-eedly script in /MacOS to see that it is using a different
  config folder)

(Seems in Chrome, setting were only saved after I logged into oblinger@gmail.com in Chrome itself)


~======================
CREATING WEF GMAIL.APP
     Create Shortcut  Create WEF  


~# READY CHROME -- close all Chrome browsers

rm -r /ob/proj/bin2/wef.app
~#   Use lowercase w/o spaces in name
~#   saves to /Application 
~#      wef234
~#      https://mail.google.com/mail/
~#      /ob/proj/bin2/wef.png
/ob/bin/makeapp 
mv /Applications/wef234.app/ /ob/proj/bin2




========================
CREATING FEEDLY.APP

~# Feedly -- did not log browser into any account.  (click top right simply says "you")

- Use 'makeapp'
- Copy /Applications/Feedly.app to /ob/bin
- manually edit .../MacOS/Feedly  
   (change user-data to be under  /ob/bin  remove the --app=   this will allow it to show browser tabs)
- log into chrome as oblinger@gmail.com
- Settings
  - advanced   share all except app&extensions
  - set restore tabs when restart 
- install  FORCE BACKGROUND TAB   (add this extension to only the Feedly app.
-  log into feedly as oblinger@gmail.com

## = UNSIGNED APPS - How to install unsigned app
    can't be opened because it is from an unidentified developer
    unknown developer

NO INSTALLER, just app file.
    !rt click OPEN  (this is different than double click and will give option to open

JUST DO IT ONCE  (holding the control key)
 ! (on .dmg file)  -->  ^rt! INSTALL.mpkg  -->   {[in installer now there will be an option to install anyway]}

DONE ONCE FOR ALL TIME (I don't do this)
 ~#space "prefs" -> !SecurityPrivacy -> !General -> !AllowDownloadFromAnywhere

## = LID-CLOSE - How to prevent sleep on lid close
  ---->  Just Use NoSleep
  $ pmset noidle
Preventing idle sleep (^C to exit)...

## = NO-RESTART-AFTER-BOOT - Prevent Apps From Restarting After Reboot

http://www.mikeindustries.com/blog/archive/2012/03/how-to-permanently-prevent-os-x-10.7-lion-from-ever-re-opening-apps-after-a-restart
Quit all of your apps.
Navigate to here: ~/Library/Preferences/ByHost/com.apple.loginwindow.*.plist (whereby * is a bunch of characters)
Click the file, do a File > Get Info (or command-I if you’re a pro), and lock it using the Locked checkbox.

## = ALT-TAB HIDING - Make app have no ALT-TAB presence
http://limitlesschannels.com/code/2013/06/29/how-to-force-osx-app-to-run-as-background-process.html

Add this to its plist:
    <key>LSUIElement</key>
    <true/>

## = BOOT MODES - Boot into special modes

-- Pwr -> hold # r           ---  REOVERY MODE: restore from timemachine, reinstall os, ...
-- Pwr -> hold # _/ r        ---  REOVERY MODE: restore LATEST os
-- Pwr -> hold opt           ---  STARUP MANAGER:  choose what to boot from
-- Pwr -> hold shift         ---  SAFE MODE:
-- Pwr -> hold #  _/  P  R   ---  RESET PRAM: (hold 20 sec, till second chime on older macs) 
-- Pwr -> hold ^  _/  shift  ---  RESET SMC:  (hold 7 sec, till powerdown)
-- Pwr -> hold # v           ---  VERBOSE MODE:
-- Pwr -> hold # s           ---  SINGLE USER MODE:
-- Pwr -> hold t           ---  TARGET DISK MODE:  machine becomes a remote disk for another computer via a USB cable


Reset PRAM/SMC:  https://www.avast.com/c-reset-mac-pram-smc#gref

## = CHANGE COMPUTER NAME - How to change the name of your computer
  

Instead use:  pref > sharing >
https://9to5mac.com/2018/06/20/mac-how-to-change-your-computer-name/
  

   sudo scutil --set HostName NEW_HOSTNAME_HERE.local  ~# Network name
   sudo scutil --set HostName simple.local  # ????reset network name back to computer name

## = IPHONE RELATED - How to setup the IPHONE
### HOW TO HIDE PURCHASED IPHONE APPS

OSX -> iTunes -> 
  (Inside itunes webframe) -> !STORE button 
  (On left menu in frame)  -> Purchased -> !Apps (button) -> ! NotInMyLibrary
  ! "x" on apps to hide them

## = CLONE - How to clone my system 




computer

-- CARBON COPY CLONER
-- IMPT:  Target Drive should be CaseINSENSITIVE & UNencrypted

-- Run Carbon Copy Cloner (on source machine)
-- Boot (press T) on 'target' machine to copy there, or insert drive.
-- Will automatically reused unchanged files.

## = RE-INSTALL OS - How to reinstall the OS

- Reboot & hold  CMD-R    -- installs OS shipped with the machine
- Reboot & hold  OPT-CMD-R  -- install latest OS
- Reboot & hold  shift-opt-R  -- another kind??


## = RENAME APP - How to duplicate / Rename an app

-- Use fluid.app for websites
-- Copy .app file then to rename
-- App rename
    Finder -> !rt -> get_info -> {edit name&ext}   ???
    (does not change the profile used)

## = MAKE APP - How to make a new app using (using ~/bin/makeapp)

(1) Google Image Search for 128x128 icon image
(2) GIMP   open image.  Export as /ob/bin/some-image.png  
(3) cd ~/bin
(4) ./makeapp
      Feedly
      http://feedly.com
      /ob/bin/some-image.png

NOTES: 
- in Finder image will not reflect .png file immdeiately
- the ABSOLUTE location of the chrome app data folder is set in the Feedly.app/Contents/MacOS/Feedly   (script file)
- CFBundleDisplayName MUST EQUAL the CFBundleExecutable in order for
  localization to be used in Finder name (will equal value in /en/

GIMP TRANSPARENT PIXEL .PNG
- Use Cut then paste to new layer (which has checkered 'transparent' background by default?)


-----
Gleaned from using /ob/bin/makeapp

- Icon file must be of type .icns (and I think square 128x128) and called icon.icns in resources (as plist.info is written)

- whatever app folder  .../XXXX.app is double clicked, its name will show in the dock.

- Given an applicaiton  ..../XXXX.app/...../ZZZZ
  Keyboard Maestro seems to index by ZZZZ for an application but then lists app as XXXX.app   # Note this is true even when some other app was double clicked (that links via shell exec to ZZZZ)

- ln -s .../Google\ Chrome  some_other_name      # this will produce a version of chrome that will run (as it is still in same folder, and seems to be separated from normal chrome.
## = CODING - How to get setup for coding
### Scripting
#### Docs
- Scripting Bridge
  http://www.macosxautomation.com/applescript/features/scriptingbridge.html

#### Commands
##### pmset -g    #  -- power management
##### $ system_profiler SPDisplaysDataType | grep Resolution
#### Cocoa Docs

http://cocoadocs.org/?utm_source=feedly#

'
#### Applescript
##### Books


http://www.oreilly.com/catalog/applescpttdg2/
The Definitive Guide by Matt Neuburg, second edition.

http://www.spiderworks.com/books/ashandbook.php
Danny Goodman's AppleScript Handbook Mac OS X Edition.

http://www.oreilly.com/catalog/applescripttmm/
AppleScript: The Missing Manual.

##### Random Examples

on run
   try
	       tell application "System Events"
	           set appList to (id of every process whose visible = true)
	           set firefoxAppList to (id of every process whose name = "firefox-bin")
	       end tell
	       
	       tell application "System Events"
	           repeat with firefoxProcess in firefoxAppList
	               set windowList to (title of every window of process id firefoxProcess)
	               
	               repeat with windowTitle in windowList
	                   tell process id firefoxProcess
	                       set (visible of every window whose visible is true and name is not "Google") to false
	                   end tell
	               end repeat
	               delay 1
	           end repeat
	           
	       end tell
       
   on error err
	       tell me to activate
	       display alert "There was a problem running the script." as critical message err
   end try
end run


##### Finding an Applications Dictionary
- Run ApplescriptEditor
- File -> OpenDictionary -> [Select application]

###  Keyboard Maestro
	     $  osascript -e 'tell app "Keyboard Maestro Engine" to do script "Name or UID of Your Macro"'

###  Scripting
-  Launching app from keystroke  http://www.macosxautomation.com/services/learn/tut01/index.html

###  Examples

~# open default browers
on GetDefaultWebBrowser()
	    set _scpt to "export VERSIONER_PERL_PREFER_32_BIT=yes; " & "perl -MMac::InternetConfig -le " & "'print +(GetICHelper \"http\")[1]'"
	    return (do shell script _scpt)
end GetDefaultWebBrowser
do shell script "open -a \"" & GetDefaultWebBrowser() & "\""


### Commands
#### killall Finder

###  Shortcuts
/@space = finder

### Objective C

https://developer.apple.com/library/mac/navigation/index.html

### System Files
~/Downloads
~/Dropbox

## = MISC NOTES - How to do misc stuff
Karma         sudo npm install -g karma
M-CLI         https://github.com/rgcr/m-cli  (not installed)
MAC-CLI       https://github.com/guarinogabriel/mac-cli/blob/master/README.md

OH-MY-ZSH     https://github.com/robbyrussell/oh-my-zsh
HeliosPaint, TuxPaint
OnyX
Intellij
lein
cursive
ClipMenu
CheetSheet
(Apptivate)  --  could be free alt to KeyMaestro
AppCleaner   -- Cleans up after uninstalled apps and other garbage on HD


AppleMail  Face2Face

- PlainClip  and cliclick  from http://www.bluem.net/en/mac/plain-clip/

- install X11  from http://xquartz.macosforge.org/landing/

NOTESTER ISSUE
  - moved Notester.app into ~/bin/old
  - Installed Java 6 since it is needed for Notester
  - may have seen java refect error during this time

  - changed vm.compression to 1.  notester was frunning at this time
  - always saw java reflect error after this.


- notester notes
  activites: topic spirtual/embetterminet
  camping
- ReOrder battery.



INSTALL R  -- http://www.r-project.org/ -->  !(Download packages CRAN) 
$ R
 >>   install.packages(c("wordcloud","rjson"),repos="http://cran.r-project.org")
> br ew   install tmux

import numpy as np


Intellij-IDEA  --  http://www.jetbrains.com/idea/  
  (Free community edition)

Groovy/Grails Tool Suite
  -- Install Grails  http://grails.org/products/ggts
  -- !"Download GGTS"
  
iPython
 $ b rew install zeromq
 $ b rew install 



intellij ???
teamviewer 8
Python3.2.5 (See notes)
TrueCrypt, Google Earth, Nodejs
b rew install id3lib
 sudo easy_install pip   # to install pip
 sudo pip install mutagen  # provides the mid3v2 bin file

SuperDuper  (just use carbon copy cloner)
little snitch  (paid)





# BUGS

## = BLUETOOTH SWITCHING - How to prevent auto device switching to MacOS

   $ sudo defaults write /Library/Preferences/com.apple.Bluetooth.plist DontPageAudioDevices 1 

other ideas in this thread:
 [https://apple.stackexchange.com/questions/159548/prevent-auto-pairing-for-certain-devices-bluetooth](https://apple.stackexchange.com/questions/159548/prevent-auto-pairing-for-certain-devices-bluetooth)  
 
## = AIRPODS MAX - How to get device switching to work
~# space 

ALL DEVICES:
  -> pref -> Bluetooth -> Airpod Max -> !Options -> off "Automatic Head Detection" 

I-PHONE
  -> settings -> Accessibility -> Touch -> Auto Call Routing    ! Automatic



brew install audioswitcher_osx


## = MESSAGES (SMS) - How to fix SMS sometimes not being received by 'messages' app

Fixes messages so that all sent messages are received on Laptop iMessages

- On I-phone:  Settings -> Messages -> Text Message Forwarding
     (Ensure relevant mac device is listed)

- On Laptop
  command-# messages
  messages -> preferences -> iMessages -> ! Enable Messages in iCloud


## = BREW - How to fix a broken home brew setup

(1) cleanup the brew's git status

CLEAN GIT STATUS
First, open terminal and cd /usr/local/, and git status to see if Homebrew is clean.
if dirty, git reset --hard && git clean -df
then brew doctor, brew update


If still broken, try this in your terminal:
$ sudo rm /System/Library/Frameworks/Ruby.framework/Versions/Current
$ sudo ln -s /System/Library/Frameworks/Ruby.framework/Versions/1.8 /System/Library/Frameworks/Ruby.framework/Versions/Current
This will force Homebrew to use ruby 1.8 from system

## = MESSAGES - How to fix messages when they are not always received on laptop

-- BUG #1: Laptop iMessages sometimes does not receive messages that phone gets
-- BUG #2: Notification will show on laptop but not show in iMessages
-- Not sure if these things fix it

-- On Latop -> # Messages/app -> #, -> iMessage -> ! Enable Messages in iCloud

-- On iPhone -> Settings -> Messages -> ! Send & Receive ->
   ensure all email and phone numbers are checked.



## = SHARE SCREEN - How to Share Screen
.
## = LSOpen ERROR - How to fix LSOpen related errors


LSOpenURLsWithRole() failed with error -10810 for the file /ob/cmd/tracker.app.

-10810 -- That would seem to imply the executable within the package isn't actually executable. 
Where did you get it from? 


This line will rebuild the Launch Services DB
    /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister -kill -r -domain local -domain user

This prompts for allowing to run apps
   sudo spctl --master-disable

## = PIP - How to fix PIP

With Yosemite install somehow pip got broke.  this fixed it:

http://stackoverflow.com/questions/21151695/broken-easy-install-and-pip-after-upgrading-to-os-x-mavericks/21751997#21751997
$ wget http://python-distribute.org/distribute_setup.py
$ sudo python distribute_setup.py
$ sudo easy_install pip

## = WIFI - How to fix WIFI connectivity 

NOTES ABOUT MAC CONNECTIVITY FAILURE

12:10pm 12/10/2014
- difficulty in getting fast internet at capital 360 cafe (Post and Kearny)
- four bars on hotspot
- data is flowing, but between less 1-8KB/s  and occasionally as high as 20KB/s for short periods.
- Power cycled computer and hotspot

- iphone reads -106db (bad signal)   after power cycle it read -80 db (good signal)
- even after power-cycle iphone to mac also has poor throughput   (but seems to be running quickly on native browser)

- Repeated attempts to connect to:  360 cafe, to .oblio and to .ob-phone 
  all yielded connections... but very slow connections.

- Others in the cafe said that 360's wireless was working ok.

- onnecting to my i-phone and turning off wireless instantly game great connectivity.

- performed many resets below

===1/2 HOUR LATER===
- system only worked with iphone on USB
- then all connections slow (even USB) and iphone streaming music was failing.
- went to mac store and internet was fast.




---------
SMC RESET -- system management controller reset

- power off
- press control-option-shift and power  then release all four keys at same time.
- press power to restart

---------
PRAM RESET -- 
- power off
- power on
- hold command-option-P-R
- listen for two chimes during boot
- allow it to boot normally

----------
RESET WIFI
- pref -> wirelesss
  !'-' to remove wifi
  !'+' to re-add the wifi 





remove and add your wifi
## = RESET MAC - How to reset many things on the mac
when wifi was failing

---------
RUN DIAGNOSTICS
- power off
- power on
- hold 'D' key down until diagnostics screen appears

---------
SMC RESET -- system management controller reset

- power off
- press control-option-shift and power  then release all four keys at same time.
- press power to restart

---------
PRAM RESET -- 
- power off
- power on
- hold command-option-P-R
- listen for two chimes during boot
- allow it to boot normally

----------
RESET WIFI
- pref -> wirelesss
  !'-' to remove wifi
  !'+' to re-add the wifi 

## = LID - How to get system to sleep when lid closes
pmset -g                 # See apps that are preventing sleep
pmset -g assertions

sudo pmset -a hibernatemode 3     # Standard (yosemite) smart sleep
sudo pmset -a hibernatemode 5     # Sleep will trigger hibernate
~----
Removed /System/Library/Frameworks/AddressBook.framework/Versions/A/Helpers/AddressBookSourceSync.app/
         contents/MacOS/AddressBookSourceSync  
	 

## = GOOGLE DRIVE - How to fix error: cannot continue syncing ... drive folder is missing
~-- Select google drive icon.  select error message.  click locate.  find folder.