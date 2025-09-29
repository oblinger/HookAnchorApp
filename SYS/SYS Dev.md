

This is my standard development setup for a project called "MYAPP"

File locations:
```
APP FOLDER SETUP is in '~/ob/proj/MYAPP' by default.  Files below are relative to that folder:
   .git               The app's repository
   docs/              The app's documentation folder
   config/            This folder is used for all configuration file, logging file, and user facing info files
   config/MYAPP.yaml  The app's configuration file
   config/MYAPP.log   The app's log file
   justfile
   

SYM LINKS
   ~/ob/kmr/prj/binproj/MYAPP         This is the folder that claude code uses as current directory for this app
   ~/ob/kmr/prj/binproj/MYAPP/code    This is a sym link to the code at:  ~/ob/proj/MYAPP
   ~/ob/kmr/prj/binproj/MYAPP/docs    This is a sym link to the doc folder at:  ~/ob/proj/MYAPP/docs
   ~/.config/MYAPP/                   This is a sym link to the app's config folder at:  ~/ob/proj/MYAPP/config



EXPECTED JUST COMMANDS
   just setup             # Fixes up the dev environment so all symlinks any other dependencies are correctly set for operations
   just build             # Performs an incremental build 
   just rebuild           # So that 'rerun' is ready to execute
   just builddist         # (Optional) builds a distribution .dmg for this application
   just buildrun          # Executes build then rerun
   just rerun             # Ensures the old app is not running, Build the app, if successful then run the app
   just test              # Run the basic app testing
   just testall           # Runs all maintained tests for the app


DEV MODE -- In the configuration file there is a Boolean parameter 'dev_mode' that determines if the system should operate in dev mode,
  if so it should perform Version Checking as explained below


VERSION CHECKING --  Prevents running stale or mismatched binaries during development by performing automated consistency checks at app startup.
  - Justfile automatically updates variable defined in the source code with a build timestamp before every build using sed
  - At App startup two check are performed:
    1. Source Consistency: Compares embedded build timestamp against current source code timestamp to detect uncommitted changes
    2. Binary Freshness: Verifies binary variable was updated before compile by matching embedded timestamp with creation date on the 
       the binary that is now running to within a 60-second tolerance.  
       (Important:  do not assume file location of binary running use OS to find current binaries path.)  creation time matches embedded timestamp 
  - If either check fails an error is added to the log optionally a dialog box is shown to user, and the app is terminated.   
  - If the checks pass, the log file is cleared and the first line should say:  "Running version X.X.X built at YYYY-MM-DD HH:MM:SS"

   
``