- [[key code]], 
## = INFO
### - Links

  [KBD_Shortcuts](__KBD%20Shortcuts__.md)
  


### _
Jupiter Notebook:  https://medium.com/@blessedmarcel1/how-to-install-jupyter-notebook-on-mac-using-homebrew-528c39fd530f


yoink (drag and drop support)
Virtual Box




### 2021-00-00 - apple support chat 
You're now chatting with Andy.

AndyThanks for contacting Apple Support. My name is Andy. Please give me a moment to look over your information.

AndyHello Dan! How are you today?

Me:hey Andy... heads up.  your website is busted.  specifically if you ask for phone support it just locks up with an infinite spinning cursor   (and I am doing fine!)

Me:on to my laptop

Me:just went in for HW repair of my bluetooth.  thanks!

Me:but I told them about crashes of the OS.  basically a kernel panic, and I must reboot.  

Me:that is still happening ... now with greater frequency.  more than once a day sometimes.

AndyI’m sorry to hear that! When was the last time you did a Time Machine backup of your Mac?

Me:today.

AndyAwesome! Are you chatting in from the Mac in question?

Me:yes

AndyGot it. What I’d recommend doing is reinstalling macOS on your Mac using your Time Machine backup. These steps do require your Mac to be shut down. What’s a good email address to send you the steps?

Me:just did this.

Me:they gave me a clean machine.

Me:I needed to do a time machine restore onto it.

AndyHow did you go about it?

Me:I went onto the recovery partition and told it to do a time machine restore on top of the new OS.  

AndyGot it. Just making sure!

Me:np.

AndyDid you go straight to restoring your TM backup, or did you use it without at all?

Me:my mac is kind of a brick w/o all my software, so I did the backup straight aways.

Me:can we look at the driver that was active when all hell broke loose?

AndyGot it. I’m just curious because it might be a software issue causing it. In which case, we’d have to do a manual backup

Me:check the logs

AndyWe don’t actually check logs over chat

Me:k.  do you know how to direct me?  I think it is pretty quick to look at that crash event right?

Me:but I am a linux guy

AndySorry, as I mentioned, it’s not something we do at this level, so I wouldn’t be able to direct you to the logs

AndyBut what I can do is send you steps on how to run diagnostics on your Mc

AndyMac*

AndyWhat’s a good email address to send those steps to?

Me:yeah, I am sure they did that at the factory.

Me:ok, so

Me:how do we move forward.  I think this is probably hardware.

Me:it almost always happens when the mac is trying to restore from screen saver.

Me:seems like either a driver on restore, or something about sleep state itself.

AndyI can always get you back into the Apple Store, but I’m not entirely convinced it’s hardware. Have you gone into a test user to see if the issue remains?

Me:well the failure occurs after extended use then sometimes on sleep.  my challenge is how to I use it enough during test user... when none of my stuff is there?

AndyYour apps should still be available on the other user. Or you can always set the sleep preferences to really short and see if it will trigger it again

Me:when I do a carbon-copy-cloner of my system's hard drive onto a second Mac, that mac never crashes.

Me:this is a bit-for-bit copy of my hard drive, so if this was a SW issue it seems it would likely crash my other mac.  but it does not

AndyThe software is identical, but is it an identical Mac?

AndyIf not, then it still may be a software issue. That being said, I’m more than happy to get you into an Apple Store if you wish

Me:no.  it is a MBP 13"  so some firmware will be different.

Me:well want I want to do, is to do all actions needed to determine that they need to replace the mother board.  I don't want to go in, and have them send me back to do other testing.

Me:lets do the testing now needed.

AndyI understand. The trick with Logic board issues is that they can’t be directly diagnosed sometimes. It usually is diagnosed by other things going wrong (like what you’re describing) That being said, I’d be happy to get those run. What’s a good email address to send you the steps

Me:oblinger@gmail.com

AndySent! Would you mind checking your email? I’d like to be sure you’ve received it and that the steps make sense to you.

Me:happy to run those tests.  if they are negative (they probably will be since they tested at factory) then are we in a position to do a HW swap?

AndyYep!

Me:sweet.

Me:nothing at the inbox yet

AndyIt won’t be from me directly. It’ll say something like “Thank you for contacting Apple Support"

Me:yes got it.

Me:so I will run these.  and if it finds a HW error, then we replace the HW.  and if it does not find a HW error (most likely case since it was tested at the factory), they we are ready to replace the HW.

Me:correct?

AndyYep!

Me:ok so either way...  still I am happy to run the test.

Me:are you noting all of this in my record?  what should I do after I run the test?

AndyI am leaving detailed notes, yes



correct?

AndyYep!

Me:ok so either way...  still I am happy to run the test.

Me:are you noting all of this in my record?  what should I do after I run the test?

AndyI am leaving detailed notes, yes

AndyEither way, once you’re done, let’s go ahead and get you back into the Apple Store. Let them know the repair center did a full wipe of your hard drive, and that it still happens

Me:ok.  so I am guessing that I can just do a drop off of the machine today.  no need for a genius bar appt right?

AndyI’d feel better if I got you a reservation, just in case

AndyThey might not require it since it’s a repeat, but I’d rather do so just in case

AndyWhat’s your zip code?

Me:94158

Me:my plan is to head out soon after the test and take my chances.. but good to have a reservation on the books too

AndyIt’s kinda like going into the California DMV without an appointment sometimes

AndyDo you have a preferred store?

Me:right  :-)   but if you are willing to just walk away w/o the laptop, usually the guy can take it.

Me:union square

AndyNext available reservation is for next Friday @ 12:15 PM. Shall I schedule that for you?

Me:sure.  but I will be there today  :-)

AndyI know :)

AndyGreat! I’ve gone ahead and scheduled that reservation for you. When you arrive at the Apple Store, be sure to check in for your reservation. You can do this either by using the free Apple Store app on your iPhone, available through the App Store, or by speaking with any available associate, letting them know you’re there for your Genius Bar reservation.

Me:as long as my notes say it is time to repair the machine, I feel comefortable they will take it.

AndyYep! They will say that I sent you

Me:it is only if they don't know what to do that I  will have trouble.

Me:You guys are awesome.

Me:I would never by a different brand... and it is 100% you guys!

Andy aww, thanks! I’m glad we’ve been able to come through for you :)

Me:cheers!
## = xx HOW-TO   xx how to   xx howto =
### - xx PORT OB TREE TO NEW SYSTEM
- Copy ~/ob onto the new machine at /Users/oblinger/ob
- Copy ~.config/karabiner/karabiner.json to new machine
- Ensure Keyboard Maestro is loading ~/ob/data/KeyboardMaestro-Master/Keyboard Maestro Macros.kmsync
- Ensure ~.zprofile is includes "source ~/ob/bin/obrc"
- Ensure prefs->users->login items-> /Users/oblinger/ob/bin/boot

- if 'km' script does not work, create a new github token for this machine

### - AIRPOD MAX - switching
	#space -> pref -> Bluetooth -> Airpod Max -> !Options -> off "Automatic Head Detection" 

brew install audioswitcher_osx


### - BLUETOOTH - Prevent auto device switching to MacOS

   $ sudo defaults write /Library/Preferences/com.apple.Bluetooth.plist DontPageAudioDevices 1 

other ideas in this thread:
 [https://apple.stackexchange.com/questions/159548/prevent-auto-pairing-for-certain-devices-bluetooth](https://apple.stackexchange.com/questions/159548/prevent-auto-pairing-for-certain-devices-bluetooth)  
 
### - xx MESSAGES (SMS) not always being received

Fixes messages so that all sent messages are received on Laptop iMessages

- On I-phone:  Settings -> Messages -> Text Message Forwarding
      (Ensure relevant mac device is listed)

- On Laptop
  command-# messages
  messages -> preferences -> iMessages -> ! Enable Messages in iCloud


### - [AWESOME COMMAND LINES](https://github.com/herrbischoff/awesome-osx-command-line)
.
### - CHROME (block prompt for notifications and location)
chrome://settings/content/notifications 

Here’s a quick guide to taking back control of Chrome.

Click Chrome > Preferences, or just paste chrome://settings/content/notifications into your browser to skip steps 2-4.
Scroll down and click Advanced
Click content settings
Click Notifications
Next to the Ask before sending (recommended) text, click the toggle button. It should now say Blocked.
### - SUDO w/o PASSWORD (xx sudoers)
  export VISUAL=nano
  sudo visudo     # MUST USE VISUDO (no other editor)

  {[ CHANGE THIS ]}
  # User privilege specification
  root    ALL=(ALL) ALL 
  %admin  ALL=(ALL) ALL

  {[ TO THIS ]}
  # User privilege specification
  root    ALL=(ALL) ALL
  %admin  ALL=(ALL) NOPASSWD:ALL

### - Setup "fast wake" on osx
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
### - Create standalone web-page apps 
     create Sandboxed Chrome Apps -- create app
     Like FluidApp but for Chrome.  See  https://gist.github.com/demonbane/1065791
     INSTALL:  cut/paste gist; chmod a+x /ob/bin/makeapp

  > feedly notes right below <

  ~# FIND AND CONVERT .png or .icns FOR .png ICON FILE
  $ chrome image.google.com  ((search for .png file for icon))  
    ! --> (copy to /ob/bin2)
  ~# CONVERT
  $ sips -s format png icon_file.icns --out png_file.png      # Read more at http://www.simplehelp.net/2010/10/08/how-to-convert-icns-files-to-png-files-in-os-x/#o5BJwxJIVWO16hl1.99
  
  ~# READY CHROME -- close all Chrome browsers
  $ open http://cnn.com   http://gmail.com  about://
    ! --> log BROWSER into wef234@gmail.com

  ~# CREATE
  $ /ob/proj/bin2/makeapp  
     # Use lowercase w/o spaces in name
     # saves to /Application 
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

### - Install unsigned app
    can't be opened because it is from an unidentified developer
    unknown developer

NO INSTALLER, just app file.
    !rt click OPEN  (this is different than double click and will give option to open

JUST DO IT ONCE  (holding the control key)
 ! (on .dmg file)  -->  ^rt! INSTALL.mpkg  -->   {[in installer now there will be an option to install anyway]}

DONE ONCE FOR ALL TIME (I don't do this)
 #space "prefs" -> !SecurityPrivacy -> !General -> !AllowDownloadFromAnywhere

### - PREVENT SLEEP ON LID CLOSE
  ---->  Just Use NoSleep
  $ pmset noidle
Preventing idle sleep (^C to exit)...

### - Prevent Apps From Restarting After Reboot

http://www.mikeindustries.com/blog/archive/2012/03/how-to-permanently-prevent-os-x-10.7-lion-from-ever-re-opening-apps-after-a-restart
Quit all of your apps.
Navigate to here: ~/Library/Preferences/ByHost/com.apple.loginwindow.*.plist (whereby * is a bunch of characters)
Click the file, do a File > Get Info (or command-I if you’re a pro), and lock it using the Locked checkbox.

### - Make app have no ALT-TAB presence
http://limitlesschannels.com/code/2013/06/29/how-to-force-osx-app-to-run-as-background-process.html

Add this to its plist:
    <key>LSUIElement</key>
    <true/>

### - xx Boot into special modes

-- Pwr -> hold # r           ---  REOVERY MODE: restore from timemachine, reinstall os, ...
-- Pwr -> hold # _/ r        ---  REOVERY MODE: restore LATEST os
-- Pwr -> hold opt           ---  STARUP MANAGER:  choose what to boot from
-- Pwr -> hold shift         ---  SAFE MODE:
-- Pwr -> hold #  _/  P  R   ---  RESET PRAM: (hold 20 sec, till second chime on older macs) 
-- Pwr -> hold ^  _/  shift  ---  RESET SMC:  (hold 7 sec, till powerdown)
-- Pwr -> hold # v           ---  VERBOSE MODE:
-- Pwr -> hold # s           ---  SINGLE USER MODE:
-- Pwr -> hold # t           ---  TARGET DISK MODE:  machine becomes a remote disk for another computer via a USB cable


Reset PRAM/SMC:  https://www.avast.com/c-reset-mac-pram-smc#gref

### - Change HomeName
  

Instead use:  pref > sharing >
https://9to5mac.com/2018/06/20/mac-how-to-change-your-computer-name/
  

   sudo scutil --set HostName NEW_HOSTNAME_HERE.local  # Network name
   sudo scutil --set HostName simple.local  # ????reset network name back to computer name

### - IPHONE
#### HOW TO HIDE PURCHASED IPHONE APPS

OSX -> iTunes -> 
  (Inside itunes webframe) -> !STORE button 
  (On left menu in frame)  -> Purchased -> !Apps (button) -> ! NotInMyLibrary
  ! "x" on apps to hide them

### - CLONE 




computer

-- CARBON COPY CLONER
-- IMPT:  Target Drive should be CaseINSENSITIVE & UNencrypted

-- Run Carbon Copy Cloner (on source machine)
-- Boot (press T) on 'target' machine to copy there, or insert drive.
-- Will automatically reused unchanged files.

### - RE-INSTALL OS

- Reboot & hold  CMD-R    -- installs OS shipped with the machine
- Reboot & hold  OPT-CMD-R  -- install latest OS
- Reboot & hold  shift-opt-R  -- another kind??


### - Duplicate / Rename an app

-- Use fluid.app for websites
-- Copy .app file then to rename
-- App rename
    Finder -> !rt -> get_info -> {edit name&ext}   ???
    (does not change the profile used)

### - MAKE APP -- NOTES on duplicating an app (using ~/bin/makeapp)

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
### - CODING -
#### Scripting
##### Docs
	- Scripting Bridge
	  http://www.macosxautomation.com/applescript/features/scriptingbridge.html

##### Commands
###### pmset -g    #  -- power management
###### $ system_profiler SPDisplaysDataType | grep Resolution
##### Cocoa Docs

	http://cocoadocs.org/?utm_source=feedly#

	'
##### Applescript
###### Books


	http://www.oreilly.com/catalog/applescpttdg2/
	The Definitive Guide by Matt Neuburg, second edition.

	http://www.spiderworks.com/books/ashandbook.php
	Danny Goodman's AppleScript Handbook Mac OS X Edition.

	http://www.oreilly.com/catalog/applescripttmm/
	AppleScript: The Missing Manual.

###### Random Examples

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


###### Finding an Applications Dictionary
	- Run ApplescriptEditor
	- File -> OpenDictionary -> [Select application]

####  Keyboard Maestro
	     $  osascript -e 'tell app "Keyboard Maestro Engine" to do script "Name or UID of Your Macro"'

####  Scripting
	-  Launching app from keystroke  http://www.macosxautomation.com/services/learn/tut01/index.html

####  Examples

	~# open default browers
	on GetDefaultWebBrowser()
	    set _scpt to "export VERSIONER_PERL_PREFER_32_BIT=yes; " & "perl -MMac::InternetConfig -le " & "'print +(GetICHelper \"http\")[1]'"
	    return (do shell script _scpt)
	end GetDefaultWebBrowser
	do shell script "open -a \"" & GetDefaultWebBrowser() & "\""


#### Commands
##### killall Finder

####  Shortcuts
	/@space = finder

#### Objective C

	https://developer.apple.com/library/mac/navigation/index.html

#### System Files
	~/Downloads
	~/Dropbox

### - MISC NOTES -
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
>  >>   install.packages(c("wordcloud","rjson"),repos="http://cran.r-project.org")
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



## = APPS =
### 2020-07-23 - Applications folder

- 1password
- Antivirus for Mac   (BitDefender)
- Barender 3
- Be Focused Pro
  - Beyond Compare<
- Divvy
- Dropbox
- Emacs
- Evernote
  - Express Scribe Transcription
- Firefox
- Flux
  - Focus time
- Folding Text
  - Goban<
- Goodsync
- Google Chrome
- Google Drive File Stream
  - Google Earth<
- Grammarly<
- Image Capture ???
- iStat Menus
- Keyboard Maestro
- Kindle
- Microsoft ...
- Neu
  - One Drive?
- Opera
- Paintbrush
- Paprika
- Plain Clip
- Pocket
- Pycharm
  - Python 3.8
- QuickRes
- Quicktime Player
- SimpleNote
- Skype
- Slack
  - Sourcetree
- Speedtest
  - Spider
- Spotify
- SpotlightCommander
- Stay
- Sublime
- VLC
  - Voice Memo
- WhatsApp
- What Size
- Xcode
  - Y! home
- Zoom.us
### --- (Appstore) ---

### xx Airmail 3
x
### xx Divvy


setup 2019.04
Compass: N S E W   Left Right Center (on East Side)

u NW   i NE   o NL   p NR
j W    k full l E    ; EC
n SW   m SW   , SL   . SR






Setup

  u NW   i NL   o NR   p NE
  j W    k full l EL   ; E
  n SW   m S   , SR   . SE
### xx Neu
      Copy Templates To Folder  /Users/oblinger/Library/Application Support/com.elegantchaos.neu/Templates


TO INSTALL
- Install 'neu' app
- Finder > Finder > Services > ServicesPreferences
  - Services > "Files and Folder"  (sub list on the right)
   ! "Create and Open Document"   (!rt show-in-finder  if you want to see the app that will trigger)

TO TRIGGER MENU
- !rt (on any folder) > Services > Create and Open Document 
- TO EDIT (while in menu !rt 'edit')

### xx WhatSize

Disk Inventory; Tree Size; DiskInventory; TreeSize

alternative

### --- (Free) ---

### xx Chrome 

  #, -> PrivacyAndSecurity -> SiteSettings -> Permissions -> Location (turn off)    -> notificaions (turn off)

   ---- OLD CHROME APPS ----
- G:Rapportive, 
- FROM GOOGLE:  SelectToGetMaps, SendFromGmail, GoogleVoice
- DRIVERS:      G:DivX (link within QuickTime)

### xx Dropbox

### xx Emacs
     ALSO USE:  https://emacsformacosx.com   # in order to get an Application/Emacs
     brew install emacs   # used 2019.07 15" pro HW rebuild

	cat >/Users/oblinger/.emacs
	(load-file "/ob/bin/emacs.el")
 
     ;; OPTIONAL:   brew services start emacs 


     ;; Old method...
     $ brew install --with-cocoa --srgb emacs
     $ brew services start emacs  # if you want it to launch at login

### xx Firefox
- 1password, AdBlocker Ultimate, Session Sync, Tree Style Tabs

~-~~
URL:  about:config

~ LOAD IN BACKGROUND:
  browser.tabs.loadDivertedInBackground --> TRUE

~ GLOBAL DEFAULT ZOOM
  browser.zoom.siteSpecific  --> FALSE   # restart firefox
  layout.css.devPixelsPerPx  --> -1/0 --> 8.0   # This scales EVERYTHING

### xx Flux (CNET)

### xx Google drive

### xx Slack

### xx Skype

### xx Spotify 
(https://www.spotify.com/us/download/mac/)
### xx Obsidian

- Go to obsidian.md   click the download button

### xx Opera

### xx PaintBrush (from CNET)

### xx PlainClip (Mac Update)
   https://www.macupdate.com/app/mac/13732/plain-clip
### xx NoSleep     (Use MAC update.  select commandline options)

### xx VLC

### xx Zoom
.

### nope xx SourceTree (from sourcetree associated with Atlassian)

### --- (Paid) ---

### Audio Hijack
**Name:** Daniel A Oblinger  
**Code:** STCK-XY9Z-VVG9-8FND-KN2U-XBYK-DGYE-UW4J-NJWP

$64 9/17/2022  
### xx Launchbar

LaunchBar 6[Download](https://obdev.at/launchbar/download.html)

16RG19Z2W0-717NJ-49SN9BUKF0

### xx Flotato Pro

wef234@gmail.com
830F8D7B-62630928-7E53630E-022BADCD-896D4DD2

### xx Apple Care

Product	MACBOOK PRO (15-INCH, 2017)
Hardware Serial Number	C02V811RHTD9
Coverage End Date	September 18, 2020
Agreement Number	970075117051737

### xx 1Password   [[Doc 1Password]]  

### xx Bartender

See 1pass

### xx FoldingText

Download:  from Hogs Bay, then try to "BUY" enter wef234@gmail.com 94110 (already paid)
Date:   2015-02-11
Email:  wef234@gmail.com
Paypal

Tools I Never used
https://github.com/RobTrew/txtquery-tools/tree/master/ftdoc%20url%20scheme%20and%20FTCopyAsURL

### xx iStatMenus   (App Store)
 -or- Mac Update w. 1pass -- I think I bought the family pack


Family Plan $25 on 10/6/2017 version 6.0
    wef234@gmail.com
    GAWAE-FBDE7-9GLJ5-JF8WF-KWEM2-UMFUN-3Z6Z5-V7UQC-CQV3K-7YH5R-D6GSN-2JFC3-JGD5W-ARML5-UCA4


Bob Daniels
1525-5152-4553-2415-4446

http://bjango.com/mac/istatmenus/

Order ID : BJA150724-3934-13312 
Paid In Full : Jul 24, 2015
Bob Daniels 
3403 Cesar Chavez Av Apt C 
San Francisco, CA 94110 
United States

### xx Karabiner-Elements  [[Karabiner Notes]] 

See ~/.config/karabiner/karabiner.json

INSTALL:
- Just drag drop the ~/.config/karabiner/karabiner.json to the new system

NOTES:
- I dramatically lowered:  basic.simultaneous_threshold_milliseconds: 1    (from 50 and maybe it was even larger before this)
  This fixed typing TAB-SHIFT too fast.
- I imported "Caps Lock --> Hyper Key ..."  and "Map Tab ..."    (from [here](https://ke-complex-modifications.pqrs.org/#emulation-modes))   and then I swapped their actions directly in my karabiner.json file.

### xx Keyboard Maestro

SETUP FILE:    !file>StartSyncingMacros     /ob/topics/Config/KeyboardMaestro-master


[[Doc Keyboard Maestro]] 


https://sites.fastspring.com/stairways/order/complete;jsessionid=F4452DB320FCF3523BD735502721EFA8?csid=1345138

2019-11-03  (also in 1password)
License: Keyboard Maestro Version 9 for 1 user(s)
Username: wef234@gmail.com
Serial: J66TWFJX75P5B5GQLW
OrderID: STA191103-5219-67395


License: Keyboard Maestro Version 5 for 1 user(s)
Username: wef234@gmail.com
Serial: GKE9D54F4013MCXEPZ
OrderID: STA120222-5239-81187

UPGRADE TO KM 6
License: Keyboard Maestro Version 6 for 1 user(s)
Username: wef234@gmail.com
Serial: 8ALJ06Z6JQUQJMG037
OrderID: STA130822-6052-46175

UPGRADE TO 7:  Order ID: STA151103-3159-70321
License: Keyboard Maestro Version 7 for 1 user(s)
Username: wef234@gmail.com
Serial: D4GU34F0FY6W23CHAG

#### setup
-- Export -- ! all macros --> #a -> file -> export as library
-- Import 

### xx Micrsoft xx Office
-- 1 password login to Microsoft wef234@gmai.com.  click 'install office'

~-~~-~-~
Microsoft office 365 personal edition 6/26/2018 purchased (auto paypal billing each year)

Your order number: 3203845939 
Once your order is processed, we'll send a receipt and any related instructions to: wef234@gmail.com. 


~-~~-~-~

STATUS:  2016-10-09 -- this is the activate version 

      Link:  http://www.microsoft.com/mac/trial
 
Product Key: P3QV7-GQC7D-KFW2T-R4FB7-V3CVH   <-- used on 2016.10
Product ID:  03314-041-1788576-02207       
             03314-041-1788576-02636 (shown as product id on 2016-10-09 activation)

???   Microsoft Office 2011 for Mac Home and Student OEM - Activates on 1 Computer
Web:  https://us-dc2-order.store.yahoo.net/yhst-29435688063266/ymix/MetaController.html?ysco_key_order_id=yhst-29435688063266-109137&ysco_key_event_id=1&ysco_key_store_id=yhst-29435688063266&ysco_key_oidc=ab2021cd96b3feff5f00&sectionId=ysco.confirm
Order Date:	02/02/2012
Order Number:	yhst-29435688063266-109137
     

This email is to confirm the receipt of your recent order from Discount Mountain Software Yhst-29435688063266

DO NOT REPLY TO THE EMAIL, if you have any questions or concerns please contact us at customerservice@discountmountainsoftware.com or 1-866-641-2563. Thank you and have a great day.

You can always find out the current status of your order by going to
https://order.store.yahoo.net/OS/stat?yhst-29435688063266+109137+ab2021cd96b3feff5f00

Date     Thu Feb  2 16:16:34 MST 2012
Ship to  Dan Oblinger
        1325 15th Street, NW #903
        Washington DC 20005
        US United States
        4156566182
Bill to  Dan Oblinger
        703-623-2486
E-Mail   wef234@gmail.com (emailed)
Via      Free Ground
Payment  Pay]}l



Please follow the link below to download the trial version for Microsoft Office 2011 Home & Student for Macs.  You will want to use the key code in this email to fully activate your product rather than any key code Microsoft may provide you with on their website.  Please let me know if you have any questions.  Thanks and have a great day!
 

Please follow the link below to download the trial version for Microsoft Office 2011 Home & Student for Macs.  You will want to use the key code in this email to fully activate your product rather than any key code Microsoft may provide you with on their website.  Please let me know if you have any questions.  Thanks and have a great day!
 
Link:  http://www.microsoft.com/mac/trial
 
Product Key: P3QV7-GQC7D-KFW2T-R4FB7-V3CVH
 
 
Thank you,
 
Tom Sarau
www.DiscountMountainSoftware.com
1730 Blake St Suite 430
Denver, CO 80202
866-641-2563 Toll Free
303-531-2095 Fax

=================

FROM DAN NEWCOME:  2MXWR-QF9P7-PRWDT-PD7C6-8GW7P
/ob/large/install/mac/2013-MS-Office

--------------
  
### xx PyCharm purchase
      (Goto https://account.jetbrains.com/licenses  W1+1pass)
________
HOW TO ALLOW CASE SENSITIVE FILE SYSTEM
PyCharm -> menu -> help -> EditCustomProperty  (then add line below)

idea.case.sensitive.fs=true
________

~-~~
eStore Order Confirmation
Order Number: A1251610

one-year 2018.3 'till 12/2/2019



~-~~

11/01/2016  2-year licence   thru -- November 13, 2017

A261257


455265855. You will receive a separate order confirmation by e-mail.
Purchase Information:

PyCharm Professional Edition - Personal License - New (#300603481) 1 Unit(s)

=========
PyCharm  4.5  
User name: Daniel Oblinger
Upgrades subscription expires November 13, 2015
~
545867-14112014
00000GrGRU!SXqnQXFaZdHyNN6XNHF
uNmYEuHZ8W4buGYLqpmFzhPYBb6"Hq
lCr2RkH5yIhXiHbIMUembWCzIrCvEc

~
WebStorm  7.0  
User name: Daniel Oblinger
Upgrades subscription expired February 28, 2014
~
545867-01032013
00001h9w9XUGXLeyGD9ThjMffRydta
HmiFE2IzgZMh1wukqegzN!35fat7gl
W96Jj1cn4IriOzITDx5XtZG4HZx6Bc


~
PAID UPDATE 2015-07-21 -- Order Number: A227077 
   === Upgrades subscription expires November 13, 2015

Reference number: INVCZ239143
Order reference: A227077
Tax point date: 22.07.2015
Issue date: 22.07.2015
Due date: 22.07.2015
Paid via: Credit Card
Payment Date: 22.07.2015
Transaction: 4414375364780816

    https://account.jetbrains.com/licenseAgreements/order/cannq7am4ta4dpxh1opf8g05s

Daniel Oblinger

545867-14112014
00000GrGRU!SXqnQXFaZdHyNN6XNHF
uNmYEuHZ8W4buGYLqpmFzhPYBb6"Hq
lCr2RkH5yIhXiHbIMUembWCzIrCvEc

____________________



-----------

User Name: Bob Daniels

===== LICENSE BEGIN =====
634462-04022014
000016s1TRjAU2clC"a0sD1SGdu3u7
1giGoI"q"YK2UDyeiA1bgpURUBvUXK
!92B"lym8rxfAB8OYcmer5d"tDcdRJ
===== LICENSE END =====

========LICENSE DETAILS========

Type: Personal License
Reference No*: LC-634462-E461655345
Date of Issue: 4 February 2014
Number of Authorized Users: 1

#### Please quote this reference when contacting JetBrains

===========LICENSEE============

Name: Bob Daniels
Customer ID: 634462
Address: 3495 Mission Street  Apt J, San Francisco 94110, California United States

=======SOFTWARE PRODUCT========

Product Name: PyCharm Professional Edition
Licensed Version: the current version and any new product release which is made generally available before 3 February 2015

The software is shipped electronically and is available for download from:
http://www.jetbrains.com/pycharm/download/


=========INSTALLATION==========

Please see below for information on your order. We have received your order. 
Please note your reference number, which is used to identify your order: 

REF# 461655345. You will receive a separate order confirmation by e-mail.

### xx QuickRes
      -- MacUpdate  

 bought on 9/21/2017
 no registration id shown

Gumroad, Inc.
Office address
225 Valencia St Suite A 
San Francisco, CA 94103 
United States
VAT ID
EU826410924
Email
support@gumroad.com
Web
gumroad.com
Receipt
Date
September 21st, 2017
To
Edgar Gumstein
123 Gum Road
San Francisco, CA 94107
United States
Item purchased
QuickRes 4.4.2
Price
$14.99
Card
PAYPAL *oblinger@gmail.com

### xx Stay  (App Store)
I think I also purchased it directly too.
https://cordlessdog.com/stay/

Name:			daniel oblinger
Email Address:		wef234@gmail.com
Serial Number:		C2MM5CIPI2O-5UK6EIB7ARG-C4MJBKVXFOU-I5QDGYBCVWF-N27KL2WQPHU-R5GFJ6ADI7K-LFHRYUB7JSY
CDOG170804-3612-53157

Living Room: 1692x3008  LG-2560x1440  LCD-2560x1440


LEFT DISPLAY:
- SIDE:  		Opera, Firefox, 
- SIDE-DN:	Calendar
- MAIN:  		Obsidian, Chrome, Safari, Mail,    Discord, PyCharm
- MAIN-UP:	Zoom

RIGHT DISPLAY
- LEFT-UP:	Messages, Slack, 
- LEFT-LOW:	Terminal, Finder
- RIGHT:		Spotify, 



### xx Sublime
download from CNET
upgraded on 9/21/2017  (Sublime Text 3)

----- BEGIN LICENSE -----
Bob Daniels
Single User License
EA7E-1118803
73AADC58 114E85C1 B1DAC9D3 61068335
98E4E260 044B3FF6 6E772E47 B01D2241
9B195594 EE77379B C719D32E A7A0BAF8
D47E06B4 871CB2C3 250890AD 8657F72F
44847659 D6BF2554 A21361E7 41909B3B
54F0FA73 11B1F3BD 4C631350 4E14B519
B9EDA2B9 E0DDCDDD E1FB07FD 9631883F
601A22E2 FEEA9E84 F6EF080C 14C91747
------ END LICENSE ------


~-~~
DOCS
-  http://www.rockettheme.com/magazine/1319-using-sublime-text-2-for-development <--- IMPT
-  http://steverandytantra.com/thoughts/three-months-with-sublime-text-2
-  http://opensoul.org/blog/archives/2012/01/12/getting-started-with-sublime-text-2/
-  http://net.tutsplus.com/tutorials/tools-and-tips/sublime-text-2-tips-and-tricks/


- Download Sublime Text 2
- Saved project file in /ob/data/config/sublime
- Cntl-`  paste txt from   http://wbond.net/sublime_packages/package_control/installation

import urllib2,os; pf='Package Control.sublime-package'; ipp=sublime.installed_packages_path(); os.makedirs(ipp) if not os.path.exists(ipp) else None; urllib2.install_opener(urllib2.build_opener(urllib2.ProxyHandler())); open(os.path.join(ipp,pf),'wb').write(urllib2.urlopen('http://sublime.wbond.net/'+pf.replace(' ','%20')).read()); print 'Please restart Sublime Text to finish installation'


- Soda Light Theme
- All Autocomplete


### NOPE xx Gemini 2 (file duplicates finder)
https://sites.fastspring.com/macpaw/order/complete
id879701667977odr
### NOPE xx Choosy
Thanks for purchasing Choosy!

Your license key is: 5a0573928155231e30aca3fc5b2d2a946ea4739156

To activate Choosy enter this key, along with your email address
 (wef234@gmail.com) on the General tab in Choosy's settings in your Mac's System Preferences.

### NOPE xx Fluidapp

### NOPE xx Textmate
REG:      Daniel Oblinger <wef234@gmail.com>
INVOICE:  0VE8T-T7VOP
Download: https://download.macromates.com/

To make it a registered copy you need to enter (copy/paste) the information below into the registration dialog which meets you at startup.
Be aware that the owner name must be entered EXACTLY as stated on the next line.
Owner name: Daniel Oblinger
License key (four lines):

JHHV7BKNSUM7VLJCC5JVRP7RPQJX46BNNNHWQPXKD5SA5DJVP6NO-
MJGZETZF6KTJ4NZBOGWVTL3ZCIPIOCCBG2AP5Z5AQLZE5RNJHECT-
GGZZZVEYXJQVLZEFWOTXTIMXUDCPG4PFKBKJZAYWSDDNM6X2EFTS-
Y6EZTAC5KPNXNWR5FGK4TBHDFNTELPH5JIYBEKVLVKQ7AHQZY

### NOPE xx Transmit
Your order ID: 1323133844655088 
Estimated Shipping Time: Immediately
Your Transmit 4 serial number:
R5SS-LQFX-HEWC-SPWR-NGWK-D


Your order ID: 1324155908930045 
Estimated Shipping Time: Immediately
K5ML-JS9D-7W59-P6HB-2MP6-Y

### NOPE xx Webstorm


User Name: Daniel Oblinger

===== LICENSE BEGIN =====
545867-01032013
00001h9w9XUGXLeyGD9ThjMffRydta
HmiFE2IzgZMh1wukqegzN!35fat7gl
W96Jj1cn4IriOzITDx5XtZG4HZx6Bc
===== LICENSE END =====


========LICENSE DETAILS========

Type: Personal License
Reference No*: LC-545867-E437455715
Date of Issue: 1 March 2013
Number of Authorized Users: 1

  Please quote this reference when contacting JetBrains

===========LICENSEE============

Name: Daniel Oblinger
Customer ID: 545867
Address: 331 Higdon Av. Apt 2, Mountain View 94041, California United States

=======SOFTWARE PRODUCT========

Product Name: WebStorm
Licensed Version: the current version and any new product release which is made generally available before 28 February 2014

### NOPE xx VMware Fusion
=== VERSION 7 ===
KEY:  01296-0Q115-H8A72-032HM-140J4
Download: http://my.vmware.com/   vmware@obinger.us+W1   (green download button on left)
Order Date: 12/14/2014 
Order Number: 9745163436 
Payment Method: MasterCard 
Order Total: $69.99 
The charge(s) will appear on your credit card as "DRI *VMware".


Order Date: 12/14/2014 
Order Number: 9745163436 
Payment Method: MasterCard 
If you paid by credit card, the charge(s) will appear on your credit card as "DRI *VMware".

Billing Address:
Nick Allen
n/a
507 Homer Ave
Palo Alto, CA 94301
US

=== VERSION 4 ===
KEY:  M129N-F9145-48MYC-A31RH-8D5MH
Customer Number: 150394234809 
Order Number: 9633200919 
Order Date: December 5, 2011 6:09:47 PM CST
Order Date: 12/5/2011 
Order Number: 9633200919 
Payment Method: Visa 
Order Total: $49.99 
The charge(s) will appear on your credit card as "DRI*VMware ".
After payment receipt, an email with order details and license information will be sent to the email provided during checkout.


------
XP from DanA:  
Licence:   R926H-CQ234-GH998-F97Q8-DRHRG
(See /ob/data/install/vmware/windowsxp to find matching ISO image)

### --- (Other App Notes) ---
.
### xx Keymap4macbook
http://pqrs.org/macosx/keyremap4macbook/source.html#t3p1


-- Getting keycodes

To investigate a key code, turn on debug mode. Type the following command in Terminal.app

$ sudo sysctl -w keyremap4macbook.debug=1
Then, type the following command in Terminal.app

$ sudo tail -f /var/log/kernel.log

### xx Notester
- Eclipse -> File -> Eclipse -> Java -> RunnableJARfile 
- #-space "jar bundle"  (add file to the /ob/data/notester/mac folder)
- FOR NOTESTER I needed to install Java 8 (the JRE I think; but I searched Java JVM; and version said ti woluld not work w. chrome, which I ignored)
### xx Postgres
   open http://postgresapp.com/
     # download 'Postgres.app' into /Applications
   echo 'export PATH=/Applications/Postgres.app/Contents/Versions/9.4/bin/:$PATH' > ~/.bash_profile
   source ~/.bash_profile
   pip install psycopg2   # this is the Postgres specific driver


   ??? psql postgres -c 'CREATE EXTENSION "adminpack";'  # Optional.  for PgAdmin UI
   open http://www.pgadmin.org/download/macosx.php
     # download 'pgAdmin3' into /Applications


### ===LIST OF APP (NAMES ONLY)===
---- GENERAL APPS ----   G=oogle, P=aid, A=ppStore 
BIG:     P:MsOffice, P:VMwareFusion
INFO:    G:Kindle, P:NetNewsReader
UTIL:    G:Chrome, G:Firefox, G:Dropbox
         A:Growl, G:GrowVersionDetective -> !skype.app -> !UpgradeFW
PROD:    G:Skype, G:Skitch, G:TeamViewer, 
PIM++    G: Evernote (4 mac)

---- DEV TOOLS ----
         G: Aptana, G:iTerm2, G:TotalTerm, P:PeepCode, ?:XCode, P:TextMate
         P:Transmit

---- UTILITIES ----
Hidden:  P:SizeUp, Growl, 
INFO:    G:MenuMeters, G:DiskInventoryX, 
MANAGE:  P:filesync
CNTL:    P:TotalFinder, P:Alfred, P:KeyboardMaestro, P:unDock
SCRIPT:  P:fluidapp,  G: Platipus, G:rooSwitch
MEDIA:   P:AudioHiJack, P:TuneUp, P:SizzlingKeys(4iTunes)
???      G:AndroidFiletransfer,  
         G:Keyremap4mac, slidepad
         G:perian, 
>>>      G:MindNode, G:Notational Velocity (orgMode), Reeder
         ?:GetCloak

---- SYSTEM / DRIVERS ----
- QuickTime, DivX, G: Silverlight
- Driver: WD portapac, 
#### (Retina List)  
   P=Paid, G=Google, A=AppStore

-  Total Finder
-  Disk Order
a  Battery Time

SPECIAL INSTALL
-  Total Finder      
-  Timeboxed         --  Licence file in /ob/large/install
-  Private Eye       --  Monitors Net Traffic
-  X11               --  Click /Applications/Utilities/X11.app (to install)
a  Xcode
a  Xcode-CmdLnTools  --  Xcode->CMD,->Downloads->!install

COPIED FROM OLD MAC
-  Alfred
-  Aquaemacs
-  Bartender         --  Moves icons from menubar
-  CalendarBar       --  Adds Date to menubar
-  ClipoboardHistory  --  
-  Cobook            --  CAPS-A Q contacts
-  DiskInventoryX    --  Treesize
-  Divvy             --  Window placement utility
-  Dropbox
-  Eclipse
-  Evernote
-  Feedly            --  RSS Reader
-  Firefox           --  
-  Fluid             --  Creates app that opens given URL in safari
-  Folding Text      --  Markdown viewer
-  Gimp              --  Image editor
-  Google Chrome
-  Google Drive      --
-  Hip Chat
-  Keyboard Maestro  --
-  Keynote
-  Kindle
-  Microsoft Office  --
-  mSecure           --             
-  Notation Velocity --  Markdown editor
-  Omni Outliner
-  Picasa            --  
-  Platypus          --  UTIL to create OSX apps (I think)
-  Pocket            --  Read It Later App
-  Postgres          --  Database server
-  Quick Res         --  Changes screen resolutions
-  Skitch
-  Spotify           --  
-  Subline Text      --  git-binary 
-  Textmate
-  Timeboxed         --  Menubar visual timer
.  Total Finder      --  Replacement for OSX Finder
-  Transmit          --  Remove File Transfer
-  VLC               --  Media File Viewer
-  VMWareFusion      --  Windows Emulation
-  Web Storm         --  JS dev environment
-  Yahoo Messenger
UTILITIES FOLDER
-  No Sleep
-  XQuartz           --  Execute X11.app to install

#### Installs on Old MacAIR (not listed obove)
  W:node.js  QuickRes  CheetSheet  Nimbuzz  MemubarCountdown  PrivateEye
  Witch   Docless  GoogleNotifierForMac  SendToKindle
  CircusPoniesNotebook   Hearld  aLaunch.app
  // slate https://github.com/jigish/slate  //  uTunes (iTunes controller)

### --- (Old) ---

### xx Timeboxed
/ob/large/install/Timeboxed has licence file

### xx TuneUp
edits tags on your music collection by auto loading
wef234@gmail.com
TuneUp Bundle Lifetime XJMN-AVRI-VCLF-1391883


### xx Sizzling Keys
control music player using keyboard
(bought in app store)


used to use these

### xx Google Earth   (now just in web browswer)

### xx Onyx         (X11 server I think)

### xx Alfred

Licence Email:  wef234@gmail.com
Licence Code:
A1SL040cy0DkUS7QSpEBrlVEsqTxN/BkVMC5CImq
Wqf3JIcay8YKV7trigUuiKhCHNfDJ/WtuvW7k/a8
vV2wk8/SmIDTzE0nIYeW2bj/K6DGZI/fZiCUpicS
r1wPKO8GhdDvPV+1/4uYvYBvYs7D1y+22QdIey5+

=====
INVOCATION:
 /ob/boot  used to invoke Alfred from console, so console script
 operate as expected from within alfred.

====








PayPal Transaction ID:27G1954492611052UMerchant:Running 
with CrayonsDate:Mon 23 Jan 2012 05:57:55 GMT
Order Information


Order ID:7299178Subtotal:£15.00

=====================================================
Alfred Powerpack Receipt
~=====================================================

Thank you for buying the Alfred Powerpack. On your bank statement,
this transaction will appear as a payment to "Running with Crayons Ltd".

You will soon receive your license code in a separate email.

~=====================================================

ORDER SUMMARY: ---------------------------------------------
 * Order #:             7299178
 * Date:                Mon 23 Jan 2012 05:55:30 GMT
 * Order Total:         £15.00


PURCHASING INFORMATION: ------------------------------------
Email:
   wef234@gmail.com

Billing Address:
   Daniel Oblinger
   331 Higdon Av. Apt 2
   Mountain View, CA 94041
   United States
   4154949499

### xx Audio HiJack Pro
  (Records any audio.  see music for more info.  )
Name: Dan Oblinger
Code: IPRT3-ZH2C-463D-KV2Z-BXMM-V4GW-4N2Z-D622-FY8Y
http://rogueamoeba.com/audiohijackpro/download.php
To enter your license key and unlock Audio Hijack Pro:
1) Open Audio Hijack Pro.
2) Click the Audio Hijack Pro menu and select the 'License' item.
3) Follow the instructions in the dialog.
Note: Make sure to enter both your name AND your code EXACTLY as they appear in this email. Copying and pasting each piece of the license is the easiest way to enter it.
(See music.org for config)

### xx Balsamiq

Name: Dan Oblinger
Key: eJzzzU/OLi0odswsqnFJzFPwT8rJzEtPLaoxNDEwMzYxMjA2NTAwqHGuMQQAMzENVA==

Your order number is: 356326

### xx Canon Pixma ip110

-- Install "Full driver & softare / mac"  From  https://www.usa.canon.com/internet/portal/us/home/support/details/printers/inkjet-single-function/ip-series/ip110

Chrome -> File -> Print
  Destination !Change  !Canon iP110

Chrome -> File -> Print

(1) Paper in
(2) Cords connect & power on

--> If it does not print
  #L  prefs  -> printers  -> print queue
  
  --> see if print queue is paused

### xx Carbon Copy Cloner 4

Registration name:Dan Oblinger
Registration email:wef234@gmail.com
Registration code:
GAWAE-FAKMR-TAHVD-T4PRS-DJLWZ-U93FZ-S6S26-PRVQC-CQNN6-2UDEZ-VHE2T-AAVCA-NR42U-DRQXT-PL99
Number of licenses:[1]

### xx Clean My Mac 3

https://sites.fastspring.com/macpaw/order/complete
id288611206645odr

### xx Enqueue
- $10 app store.
- music playback 'attached' to itunes

### xx Flip4Mac
media player (for browswer)


The information for your products/services 
Flip4Mac Player 
Serial Number
AA1B000001-36EY-5JWR-1DDJ-10J4-BGB5

### xx Intelli J     (was JS IDE ???)

### xx Kalidascope
      (Now prefer beyond compare 2019-08-14)
      File comparison / merging tool

      https://sites.fastspring.com/blackpixel/order/invoice/BPXL150512-2167-12173
Daniel Anthony Oblinger
wef234@gmail.com
BPXL150512-2167-12173
1	Kaleidoscope

====
Order ID : BPXL160607-2632-68199 

### xx Little Snitch
Here is your license key for Little Snitch 3:

    Daniel Oblinger
    ===========================
    33RG19Z2W0-715KS-BZSX6UTUK2
    ===========================

How to enter the license key:

(1) Open the "Little Snitch Configuration" application from your
    Applications folder.
(2) Choose "Little Snitch Configuration" > Preferences and select
    the Registration pane.
(3) Click the Register button to open the registration form.
(4) Enter the above license key into the form (Use Copy and Paste
    to prevent typing errors).
(5) Click the OK button.

If you have not downloaded Little Snitch yet, please go to our
web site at http://www.obdev.at/littlesnitch/download.html and
download it.

### xx NetNewsWire
      read it later-ish; new feed; ...

If you have any questions about your order, please contact us:
Website: http://blackpixel.com
E-mail: info@blackpixel.com
Phone: 206-216-3923

order # ST85816116
placed 3/21/2012 3:17:34 PM
item	qty.	price	total
NetNewsWire 3.3 (SKU22343923978) (Macintosh) 
  Registration Information: 
     Serial Number: NNW32ESEL-9FCM-HJ9A-24QP-AQPS-DN61 

### xx OmniOutliner

Order Number: OS847326
Order Date: July 04, 2012
Amount Due: $0.00

Dan Oblinger
331 Higdon Ave  Apt 2
Mountain View CA 94041
US
wef234@gmail.com


OmniOutliner 3 License / $39.99 / 1 / $39.99
Quantity Discount / $0.00

OmniOutliner 3
1 seat license

          License Owner: Dan Oblinger
          License Key: FZUH-ZYRG-JRCV-QHIH-MUAA-EDT

### xx OmniGraffle 6
OmniGraffle 6
1 seat license

          License Owner: Dan O
          License Key: MQPA-PBVK-YLJI-DYIL-TMQM-HDW

### xx PathFinder 7
Product: Path Finder 7
Licensed to: Dan Oblinger
License: PF7:GAWAE-FDFPQ-FQNHB-J756S-HPADX-5DNB8-QNL2Y-PZNAC-CQG9L-SXK9U-RXRKQ-KKPLE-S6TSF-CPP34-79JM-21UU1
Number of Users: 1

2015-05 Tried this again briefly.  seemed like I did not want to use it.  also many of my scripts expect the finder.
        Uninstalled after only a bit of time (not a deep trial -- it could be useful for doing incremental capy)

### xx PeepCode
- LINK:   https://peepcode.com/orders/AP42a-12fqnt
- 
Your transaction ID for this payment is: 4VA883875L624622Y.
Topfunky Corporation


206-276-5474

### xx SizeUp
moves windows around using keyboard
Wef234@gmail.com  (See install directory for licence)

Feb 27, 2012 21:30:13 PST | Transaction ID: 4RS53682YE4088727

Hello Dan Oblinger,

You sent a payment of $13.00 USD to Irradiated Software, LLC
 (support@irradiatedsoftware.com)


It may take a few moments for this transaction to appear in your account.

Total: $13.00 USD

Payment: $13.00 USD
Payment sent to: support@irradiatedsoftware.com

Invoice ID: 14fmdqhx7zu69wal6920kejsw0ggs4gwgo00gc4

----------------------------------------------------------------



### xx Total Finder
=========== NO LONGER USING ===============
VER:   Works with v1.4.9  on Retina DO35
Web:   https://sites.fastspring.com/binaryage/instant/totalfinder
Key:   GAWAE-FC78T-2U42V-4KME7-D7ZMT-Z6GDU-EC9T5-AH29C-CR8Z9-JHGU6-KS9AP-HAMXT-NX99U-6UTPX-8HEM

Your license information is: 
License Name: "Daniel Oblinger"
License Key: "GAWAE-FC78T-2U42V-4KME7-D7ZMT-Z6GDU-EC9T5-AH29C-CR8Z9-JHGU6-KS9AP-HAMXT-NX99U-6UTPX-8HEM"

Order ID : BA120209-5241-17228 
Paid In Full : Feb 9, 2012


IMPORTANT:
Please make sure you copy and paste the name and the key without the quotes.

### xx unDock
(app store)
### TimeBoxed (broken now)

### Timebar

### WATCH - JustPressRecord

### WATCH - DayOne

### Fitbit

### Disk Inventory

### xx GrandPerspective
      DONT USE

      USE WhatSize

NOT GOOD FOR FOLDERS!!
Then got WhatSize

## = BREW =
### xx Pandoc
   # https://medium.com/macoclock/how-to-setup-pandoc-and-latex-on-macos-mojave-8e18fa71e816 
    brew install pandoc
    brew install librsvg python 
    brew install --cask basictex

## = OTHER HARDWARE =
### - iPHONE
#### App arrangement

	=== SCREEN1 ===
	1st:  Calendar  Kindle    Yelp      G-maps
	2nd:  Clear     Spotify   FOOD      Google
	3rd:  P-UTIL    MUSIC     LOCALE    INFO
	4th:  UTIL      PLAY      COMM      -photos-
	5th:  Prefs

	=== SCREEN2 ===
	1st:  GAMES     PHOTO     HEALTH    ACTIVITES
	2nd:  TRAVEL    WATCH     TOTRY     ALEXA
	3rd:  BGAMES    APPLE


	iPhone APP reload
	TOP:          Elements, Orbitz, 
	ACTIVITIES:   Sosh, EventSeeker, Trulia
	TO TRY:       Food Sense, MyNoise, Routsey, Camera360, Documents, VSCO, Instapaper, Wallpapers, Brushstroke, 
	              Cleanup Dups, Pulse, YouTube, Evernote, Tempo, Path, Amazon, AirBnB
	PHOTO:        Panorama, Afterlight, 360, FastCamera, Camera+, PixelWakker
	WATCH:        Decibel, Calcbot, NikeRun Joy, RunKeeper, NoomCoach





#### Setup
	- gChord keyboard 
	=== MAYBE THIS IS **PHONE** SETUP???

	- Settings -> Mail/Contacts/Calendar
	  - Gmail:  sync mail,contacts,calendar,messages,notes
	  - iCloud: sync reminders, safari, findmyphone

	- phone -> contacts -> groups
	  be sure to UNCHECK the "all on my phone" checkmark  (else it will create contacts locally)

	- Settings -> iCloud
	  - > Account    AppleID = wef234@gmail.com
	                 Mail    = daniel.oblinger@icloud.com
	  Check:  Reminders & Safari & FindMyIphone
	  UNCHECK: ALL ELSE


	- Settings -> Email/Contacts/Calendars
	  - iCloud sync  (Just safari, Documents, and FindMyPhone)
	  - GMAIL  oblinger@gmail.com  !mail  !Contacts  !Calendars

### - Canon iP110 (Portable Printer)  2015 (bought while at Bayside)

-- Turn on printer (solid power light)
-- Press wifi button; hold for TWO flashes of the power light.
   --> Should cuase wifi light to flash
-- ON iPhone.  
   -- AppStore download "Canon PRINT inkjet/SELPHY"
   -- register printer .... "mononoke" ...   SHOULD SAY "Sent the settings"
-- ON Mac
   -- Download Canon print drivers
   -- #prefs  ->  printers&scanners  ->  !"+"
      -- ! "Canon iP110 series   Bonjour"
      -- use:  ! "AirPrint"

## = BUGS =
### MESSAGES not always received on laptop

-- BUG #1: Laptop iMessages sometimes does not receive messages that phone gets
-- BUG #2: Notification will show on laptop but not show in iMessages
-- Not sure if these things fix it

-- On Latop -> # Messages/app -> #, -> iMessage -> ! Enable Messages in iCloud

-- On iPhone -> Settings -> Messages -> ! Send & Receive ->
   ensure all email and phone numbers are checked.



### Share Screen
.
### LSOpen


LSOpenURLsWithRole() failed with error -10810 for the file /ob/cmd/tracker.app.

-10810 -- That would seem to imply the executable within the package isn't actually executable. 
Where did you get it from? 


This line will rebuild the Launch Services DB
    /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister -kill -r -domain local -domain user

This prompts for allowing to run apps
   sudo spctl --master-disable

### BROKEN PIP

With Yosemite install somehow pip got broke.  this fixed it:

http://stackoverflow.com/questions/21151695/broken-easy-install-and-pip-after-upgrading-to-os-x-mavericks/21751997#21751997
$ wget http://python-distribute.org/distribute_setup.py
$ sudo python distribute_setup.py
$ sudo easy_install pip

### 2014-12-10 -- WIFI CONNECTIVITY ISSIE

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
### RESET MAC -- RESET MANY THINGS
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

### System will not sleep when lid closes
pmset -g                 # See apps that are preventing sleep
pmset -g assertions

sudo pmset -a hibernatemode 3     # Standard (yosemite) smart sleep
sudo pmset -a hibernatemode 5     # Sleep will trigger hibernate
~----
Removed /System/Library/Frameworks/AddressBook.framework/Versions/A/Helpers/AddressBookSourceSync.app/
         contents/MacOS/AddressBookSourceSync  
	 
### fixing brew
(see Brew section)
### GOOGLE DRIVE -- google drive cannot continue syncing because your google drive folder is missing
~-- Select google drive icon.  select error message.  click locate.  find folder.
# App Ecosystems
## STARTING FROM PRISTINE INSTALL
OS: 
- 3 finger Drag, PowerNeverSleep
- 1Password, Google Drive, Airmail3, emacs
- AppStore purchases

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

~# Display full POSIX path as Finder window title
defaults write com.apple.finder _FXShowPosixPathInTitle -bool true


~# Use list view in all Finder windows by default
~# Four-letter codes for the other view modes: `icnv`, `clmv`, `Flwv`
defaults write com.apple.finder FXPreferredViewStyle -string "Nlsv"

~# Makes default text larger
defaults write NSGlobalDomain AppleDisplayScaleFactor 1.5    # change back to 1.0 for std system


> Do 'sudoers'
> Do 'ssh-setup'

## xx OSX SETUP + Apple Apps + xx macos
### -->xx FINDER<---
#### FINDER: Favorites bar
      -- Select the MyDesk folder.  
      -- Press  #^T  to add to favorites, or
      -- Drag Folder Icon onto the Finder Favorites Bar to add it.
#### FINDER: search should serach only current folder
finder -> #, -> Advanced -> "When Performing Search"  !"Search Current Folder"

#### Show Dot Files
      defaults write com.apple.finder AppleShowAllFiles TRUE
      defaults write com.apple.finder AppleShowAllFiles FALSE
      killall Finder

#### Create file associations
- Finder -> rt! file.xxx -> !!!GET-INFO!!! -> !open-with -> !other
  (select application to launch with)  -> ! CHANGE-ALL

### -->xx KEYBOARD<--
#### Keybindings
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



###### Cocoa-keybindings -- Default.dict
- DETAILED GUIDE http://www.hcs.harvard.edu/~jrus/site/cocoa-text.html
- OS X DOCS      http://developer.apple.com/library/mac/#documentation/cocoa/conceptual/eventoverview/Introduction/Introduction.html


####### UNICODE NAMES
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

#### Misc Setup
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

#### Set key repeat
      defaults write -g ApplePressAndHoldEnabled -bool false      # ON RETINA  DO35
      defaults write NSGlobalDomain KeyRepeat -int 0

#### Docs on arbitrary key remapping
http://matthewpalmer.net/blog/2014/01/19/remap-keyboard-keys-caps-lock-os-x-mavericks/

#### Add keyboard shortcuts to a single app (using OS)
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

### -->xx OS PARAMETERS<---

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

## FINDER 

~# Finder: allow quitting via ⌘ + Q; doing so will also hide desktop icons
defaults write com.apple.finder QuitMenuItem -bool true

~# Display full POSIX path as Finder window title
defaults write com.apple.finder _FXShowPosixPathInTitle -bool true

~# Enable spring loading for directories
defaults write NSGlobalDomain com.apple.springing.enabled -bool true

~# Remove the spring loading delay for directories
defaults write NSGlobalDomain com.apple.springing.delay -float 0

~# Use list view in all Finder windows by default
~# Four-letter codes for the other view modes: `icnv`, `clmv`, `Flwv`
defaults write com.apple.finder FXPreferredViewStyle -string "Nlsv"

## MAIL 

~# Copy email addresses as `foo@example.com` instead of `Foo Bar <foo@example.com>` in Mail.app
defaults write com.apple.mail AddressesIncludeNameOnPasteboard -bool false

#### SETUP PARAMAETERS TRIED ON 15" 
- OSX -> #L Pref -> Keyboard -> Shortcuts -> Services -> Shortcut -> Files and Folders -> ! CreateDocument
  
### MAIL -- DELETE2ARCHIVE  make delete key do an archive action

To get DEL key to archive:
- OSX -> Mail -> Mail -> Preferences -> 
  - Accounts -> MailBox Behavior -> Trash 
    uncheck "Store deleted messages on server"        <--------- the one that does it!
    ( "move deleted messages to trash"  should remain checked)

- GMAIL -> Setting -> Pop/IMAP settings -> 
  "WHEN MESSAGE IS MARKED AS DELETED" ->
     CHECK "Archive the message"

## DID these on 15"... did they work?

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

### -->xx SPOTLIGHT COMMANDER<--
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

### -->xx MAIL<--
#### MAIL -- GENERAL SETUP
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

#### MAIL -- ACCOUNTS  as of 2017-02-05

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

### -->xx MAIL/CAENDAR<--
###### CALENDAR OVERVIEW

Calendar -- 
-- use iCloud 'calendar' as default.  
-- delete all others "on my mac" or other "iCloud" calendars
-- include calendars for: oblinger@gmail.com  dan@analyticsfire.com  ??martain?? 

###### Delete Key -->  Archive Key
   http://thingsofinterest.com/delete2archive/
   
   Stopped Mail
   Copied bundle into ~/Library/Mail/Bundles

###### Removing entries from autofill
   Working with PREVIOUS RECIPIENTS
- Mail -> Window -> Previous Recipients -> [[remove email address from list]]

###### Sending mail from multiple addresses
AppleMailApp>Preferences>Accounts>Account Information>Email Address
    - Add list of all email addresses (SEPARATED BY COMMAS)
    - Restart Mail then send message to see dropdown

    oblinger@gmail.com,dan@analyticsfire.com,dan@oblinger.us,dan@martianbots.com

> SETTING THE DEFAULT SEND TO ADDRESS
    - AppleMail > View > Show Mailbox List 
    - ! InBox
      - ((Reorder list of in boxes to put default box at top))

###### CONTACTS Setup



=== HIGH LEVEL ===   (6/13 on retina; 6/15 updated)
- Using gmail contacts as master
- Using empty local contacts book on mac (ensure default addr book is google)

========
- SPOTLIGHT "contacts"
  -> {Delete all contacts on local mac}
  -> Accounts -> !"+" -> Add CardDav account for oblinger@gmail.com
  -> General -> DefaultAccount=Google

###### Key for archive to "_in"
####### New Method
- Ensure _in2 is the FIRST favorites button (in the favorites bar at the top)
- Create a Keyboard Maestro macro that maps #+'-->' to ^#1
  (this is in the menus as move to first favorites)
  -- be sure that this keyboard macro is created in the Apple Mail group which always runs
     for the mail.app application.

####### Old method
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

### -->xx DOCS<--   NOTES
https://github.com/mathiasbynens/dotfiles/blob/master/.osx
    https://github.com/mathiasbynens/dotfiles/blob/master/.osx 
http://www.mactech.com/articles/mactech/Vol.09/09.08/ScriptAppExample/index.html 
http://secrets.blacktree.com/
http://apple.stackexchange.com/questions/400/please-share-your-hidden-os-x-features-or-tips-and-tricks?page=2&tab=votes#tab-top
tricking out your mail client   http://getpocket.com/a/read/406605693
Crypto email    https://gpgtools.org/gpgmail/

### -->xx DEVICE<--
#### AIRPORT
          (not done on retina)
       $ sudo ln -s /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport /usr/sbin/airport
       $ airport -I
       // DB values
       $ while x=1; do /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I | grep CtlRSSI; sleep 0.5; done

       // Associate w a network
       $ sudo networksetup -setairportnetwork en0 Oblio obliooblio

#### POWER
pmset -g assertions    # the key PreventUserIdleSystemSleep is '1' if something will stop sleeping

sudo nvram boot-args=iog=0x0   # CAREFUL.  don't know how to undo.
                               # system will sleep in 'clamshell' mode when lid closes.  internal display will also shutdown

~# Test is clamshell is closed
ioreg -r -k AppleClamshellState -d 4 | grep AppleClamshellState  | head -1
### -->xx APPLED-APPS<--
TEXT EDIT
$ defaults write com.apple.TextEdit RichText 0
#### i-CAL 
###### Remove entries from autofill
- ((Remove entries w. bad email addresses in them.))  ((Quit iCal.))
- Finder -> Retina -> Users -> Oblinger -> Library -> Caches 
  -> com.apple.iCal -> Previous Recpients.plist

     -> Root -> Calendar Google.com -> [remove bad email address]
#### i-Tunes  (See music; See How-To->Keyboard below)

### -->xx SCREEN SHARE<--...
-- Ensure account is logged into iCloud (in preferences)
-- Have parents share to ME
        
### -->xx SCREENSHOT<--
     defaults write com.apple.screencapture location /ob/data/MyDesk

### -->xx BUGS<--
#### OSX Memory compression bugs
~# Notester is running slow. so I disabled memory compression to see if that is the issue
~# http://superuser.com/questions/668114/disable-compressed-memory-in-mac-os-10-9-mavericks
$ sysctl -a vm.compressor_mode
vm.compressor_mode: 4                       # Standard value
$ sudo nvram boot-args="vm_compressor=1"
  ((Reboot))
$ sysctl -a vm.compressor_mode

## xx HOMEBREW  xx brew
### BREW SETUP
     --  Google "Homebrew"  then cut/paste line into terminal window
     --  install xcode licence as described, then rerun

BREW NOTES
     $ brew install xxxx    # Never as root
     $ brew unlink  xxxx

brew update
brew install python3 / emacs

### BREW FIX

(1) cleanup the brew's git status

CLEAN GIT STATUS
First, open terminal and cd /usr/local/, and git status to see if Homebrew is clean.
if dirty, git reset --hard && git clean -df
then brew doctor, brew update


If still broken, try this in your terminal:
$ sudo rm /System/Library/Frameworks/Ruby.framework/Versions/Current
$ sudo ln -s /System/Library/Frameworks/Ruby.framework/Versions/1.8 /System/Library/Frameworks/Ruby.framework/Versions/Current
This will force Homebrew to use ruby 1.8 from system

### brew xx clojure
     $ brew install meld clojure
### brew older xx Utilities
     # These listed with MySql install
     $ brew install git cmake ack wget redis memcached libmemcached colordiff imagemagick
       # DO35 removed 'brew install curl' since Retina has it
       # DO36 added cmake since sql needed it.
     
### brew older xx MySQL
     # http://www.frederico-araujo.com/2011/07/30/installing-rails-on-os-x-lion-with-homebrew-rvm-and-mysql/
     
     $ brew install mysql
     # configure it
     $ mysql_install_db --verbose --user=`whoami` --basedir="$(brew --prefix mysql)" --datadir=/usr/local/var/mysql --tmpdir=/tmp

     # DONT EXECUTE, but this would cause it to load on boot
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

### brew older xx postgres
  XXXXX decided to go to download Postgres.app directly   2015.05  Retina-yosemite


  # first get brew & xcode
  $ brew install postgres
  $ echo 'export PATH=/usr/local/Cellar/postgresql/9.4.2/bin:$PATH' >> ~/.bash_profile
  $ 
  $ initdb /usr/local/var/postgres

### brew older xx Mountain Lion Setup

Apple removed GCC 4.2. We’ll just put it back where make expects it to be. First install the package manager Homebrew. Then in Terminal do:

$ brew tap homebrew/dupes
$ brew install apple-gcc42
$ sudo ln -s /usr/local/bin/gcc-4.2 /usr/bin/gcc-4.2

~# Need to install Qt
$ brew update
$ brew install Qt

### brew older xx SSHFS
     Download  OSXFUSE & SSHFS  from http://osxfuse.github.io/

## xx PYTHON
#### PYTHON SETUP 2019-07-21 - 15" rebuild
xcode-select --install
brew install python3
~# in pycharm setting up new interpreter used venv under /uf by default
#### PYTHON SETUP 2018-10-27
      # Add required folders (owned by oblinger)
      sudo su
         cd /usr/local
         mkdir include Frameworks
         chown oblinger include Frameworks
         chgrp admin Frameworks include
         chmod 775 include Frameworks
      # install python
      brew install python    # this will brew update & xcode-select --install
      
#### xx KERAS 

      # https://www.youtube.com/watch?v=LnzgQr14p7s

python3 --version    # 3.8.1 (2021.04 installed 3.9)
open https://docs.conda.io/en/latest/miniconda.html

### xx PYTHON3 on OSX at system level
- In running my _ fling.py script from keyboard maestro it ran some MAC OS install of python3  (I think this became a system level python3 instance)

#### brew xx python       (16.10.07 approach)
     $ brew install python

     $ which python
       /usr/bin/python         # ships with OSX
       /usr/local/bin/python   # symlink to brewed python
      
#### OLD SETUP  (QL-personal 2015.10)

    # COMMAND-SPACE "XCODE"  (just verify that you can accept any agreements)
    open a.xcodeproj

    # GET BASE PYTHON FOR VIRTUAL ENV TO BUILD FROM
    open http://python.org/download  # click "Download Python 2.7.10"

    # Install EasyInstall (if needed)
    curl https://bootstrap.pypa.io/ez_setup.py -o - | sudo python

    # Install pip         http://stackoverflow.com/questions/17271319/installing-pip-on-mac-os-x
    sudo easy_install pip

    # Install Virtualenv
    VER=Py2_7_10
    sudo pip install virtualenv
    sudo pip install virtualenvwrapper
    echo 'export WORKON_HOME=~/Envs' >> ~/.bash_profile
    echo 'source /usr/local/bin/virtualenvwrapper.sh' >> ~/.bash_profile
    echo "workon $VER" >> ~/.bash_profile

    # Create Virtual Env 
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
  
#### ipython & DataScience packages
http://penandpants.com/2013/04/04/install-scientific-python-on-mac-os-x/

#### OLD -- Manual installs of python
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
  # Just ignored this.  installed all in root env

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

  # test   http://scikit-learn.org/stable/auto_examples/exercises/plot_iris_exercise.html



import scipy
> from scipy import linalg

A = np.array([[1,2], [3,4]])
> A
ray([[1, 2],
       [3, 4]])
> linalg.inv(A)

#### OLD Anaconda

- Loaded 64-bit Anaconda environment from http://continuum.io/downloads 
  into /ob/pkg/Anaconda-iPython
- /ob/bin/ipython will launch environment
- ~/.ipython  configures the environment
- Added ~/.ipython/profile_default/startup/do_ipython_setup.py

ipython              # this will launch the ipython console
ipython-launcher     # this will launch the ipython-launcher app
~# l ipython-launcher  # also launches the launcher

## xx RUBY + RVM + xx SINATRA

2015-09-09 -- install RVM  (from http://usabilityetc.com/articles/ruby-on-mac-os-x-with-rvm/)

  # INSTALL RUBY INSIDE RVM INSIDE $HOME/.rvm
  brew install gnupg
  gpg --keyserver hkp://keys.gnupg.net --recv-keys 409B6B1796C275462A1703113804BB82D39DC0E3
  \curl -sSL https://get.rvm.io | bash -s stable --ruby
     //==> should have added line to ~/.bash_profile  and ~/.bashrc

  # Reopen terminal
  rvm use 2.2.1         #==> or use which ever version is the lastest
  gem install bundler 

  rvm list      # shows versions.  Shows available ruby environments (use downloaded version in command above)
  which ruby    # shows location.  Should be in your home folder under .rvm
  ruby -v       # shows version.   Should be the loaded version
  gem env       # shows gems.      Shows gems loaded into current environment
     
  gem install sinatra
