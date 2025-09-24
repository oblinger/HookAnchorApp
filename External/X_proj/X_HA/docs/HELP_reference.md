# HookAnchor Help Documentation

# Installation

## SETUP -- Getting HookAnchor

terse notes here explaining what the installer will do automtically
- for karabiner file copying that the 
- config file copying


one short section for 
- permissions explanation of what user must do and why

one short section for 
- steps to install karabiner, then steps inside karabiner to activate macro
    (also talk about alternative of using OPT-A or whatever user wants in config file to )


## UNINSTALL -- Removing HookAnchor

Explain to go to activity monitor to remove processes, then delete from the application folder (dont explain our uninstall script)

# User Documentation

## POPUP -- Hook Anchor Usage Basics

Explain how the basic pop-up works in a very terse way. The user needs to understand as they type in characters. It's selecting commands that match those characters, and then, if a prefix of those command matches, an anchor and a special prefix menu is generated with the elements in that anchor. That's all commands, his prefix matches, and all commands with that same patch name. And that ends up in the prefix menu before the equal sign separator. They can use arrows to navigate around the selection and hitting enter will execute the current selection. Escape or exit escape exit the pop-up can be drugged around by clicking under the input box in the center.



## KEYBOARD -- Keyboard Shortcuts

 BUILTINS -- Using the Command Palette

Explain the built-in commands that have key Binns associated with them. Most of them can be defined by a single line. We should probably show the default key binding for each one. Maybe we name each item in IN parentheses show the key binding and then have a phrase or sentence that explains what it does.

## EDITOR -- Using the Command Editor


And I think we also do automatic inference of action type as well.  A sentence or two describes how that automatic inference works in a very terse way


## ACTIONS -- Built-in Action Types

Briefly explain each of the action types. Have the action type in FOLLOWED by a dash and then a description of that action type. Maybe this is just a list of bullets.


## CREATE -- Creating New Commands, File, Folders
Explain how key bindings trigger new template, creation, and how certain templates have a grammar associated in a sentence or two about how to use the grabber so I understand how to create new things with templates make reference to the template section later on how to design templates.



## ANCHORS -- Working with File Anchors


- once an anchor has been selected it's shown in light gray for some minutes after that and many of the keyboard commands will implicitly operate relative to that anchor if they're executed immediately after opening the app. Also pressing the space key will cause that anchor to be become part of the input box.
- 
, we should talk about how the tree of anchors ties together, and how the tree of anchors ties together
How the anchors affect prefix menus, a fake prefix menus are computed prefix menus are what those into them and the breadcrumbs show me the top, all should be described in very tourist language, but also in a complete way very terse language, but complete

## SCANNER -- Auto-discovering Commands


Have a section that talks about automatic inference of patches.

This section should also talk about the creation of orphans.


# Configuration Documentation

## FILES -- Configuration File Structure

Explain the relevant files. At the top simply have a file structure. Little diagram is a little outline for all the files, with the phrase describing on each one is.

For key files like commands.text, config Yamo and config JS explain the editing that the users allowed to do.

## SETTINGS -- Main Configuration (config.yaml)
Here we have a terse discussion of a number of the important config file parameters in the settings section. We need not cover all of them just the main ones that the user might wanna change, and really there's already some documentation in the config file itself so we may not document everything although a quick bullet on each one of these is not bad kind of one line per each one and then some that are important we might have a second or third line describing some aspect of that setting.

## FLOW -- Understanding Hook Anchor Command Execution

Provide very terse summary of the steps in command execution, so the user types of command it's selected the action of the command is up in actions. The parameters are merged with the command parameters and then expansion is performed and it's dispatched either within the pop-up or within the command server key bindings are direct actions that execute without a command. Provide a stacked flow that shows the steps there maybe each step is a single line and each line is indented in to show that one flows into the next one something like that the very compactly shows how everything fits together.

## EXPAND -- Action Expansion

 document how action expansion occurs, and list one bullet point per variable for all the variables available during action expansion. Also explain how parameters are set for grammar parameters, and how that works. That's probably a little section at the end.

## COMMANDS -- Commands File Format

In the case of the command file that should probably have a little section that explains the format for that file and explains if the user is allowed to edit it anytime they want, but they have to hit the rebuild button before the app will see the changes they make. Configure and config JS or explain below, so nothing needs to really be said specifically hear about those and in the section about commands that text we should probably also notice the backups are made in the sub folder and explain how they can copy those up.

## ACTIONS -- Configuring Custom Actions & Keybindings

Explain how actions relate to commands ..., a very terse section just describing how actions are looked up from the commands and our parameters are and merged.

Then, a bit about how key bindings work.  Explain the user how to create a string for the keyboard shortcut key with modifiers. Try to keep the explanation complete but tears butt tears but shorts.

Final section is how to find how to find new kinds of commands how to find a new kind of command types here, Actually, I would have a little section at the bottom of this section called defining new commands. Since that's more advanced at the top



## TEMPLATES -- Template Configuration 

Don't duplicate code that exists don't duplicate explanations that exist in more detail and other places, but do explain how to create new templates.
Explain if they want a grabber to be with associate with the template how to do that
Explain how to create new files and folders and new content in the file


## JAVASCRIPT -- Scripting Configuration (config.js)

Explain how the JavaScript system works, what variables and parameters are available to the user, and how the functions there connect back to the execution of the whole system. And the methods and functions available to the user in that environment as well or to the code writer in that environment as well.



# Troubleshooting

## LAUNCH -- HookAnchor Won't Start

## POPUP -- Popup Window Issues

## PERMISSIONS -- Accessibility Problems

## SHELL -- Shell Integration Not Working

## KARABINER -- Hotkey Not Functioning

## SCANNER -- Missing Commands

## PERFORMANCE -- Slow Performance

## NETWORK -- Server Connection Issues

## LOGS -- Finding and Reading Logs

## DEBUG -- Running in Debug Mode

## RESET -- Resetting Configuration

## RECOVERY -- Recovering from Errors

## UPDATES -- Updating HookAnchor

## SUPPORT -- Getting Help