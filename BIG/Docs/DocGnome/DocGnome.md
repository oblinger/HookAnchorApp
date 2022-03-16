
# Keyboard Shortcuts
## gconf-editor
  $ sudo apt-get install gconf-editor
  $ gconf-editor
  ! apps -> meta-city -> global_keybindings

## Key names
<Control> <Alt> <Shift> <Super> (win key)
<Up> <Down> <Left> <Right>

Ensure the value is entered by clicking a different entry (if you don't, the command may disappear as you change to keybinding_commands)


## xmodmap
   xmodmap -grammar            # Prints mgs below
   


## CapsLock

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




# Notes key shortcuts
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

# Command Line

## gnome-terminal
   --command
   --tab
   --title 
   --hide-menu-bar
   --geometry=RxC+x+y
   --zoom 1.0
   --working-directory

