
## QUICK REF
  $ git init                     # creates .git
  $ git add .                    # recursively adds files to repository
  $ git commit -a -m "msg"       # commits files (and does the 'add .' too)
  $ git push -u origin master    # ??? Pushes local commit to master
  $ git checkout [bname]         # moves from 1 branch to other  (changes active files)
  $ git branch [-d] [bname]      		  # view/create/delete branch
  $ git status                   # Status of current branch
  $ git pull <file> <branch>
  $ git commit --amend -m "msg"  # re-commit with changes and new message
  $ git clean -d -f              # FORCES removal of files and DIRS
  $ git reset --hard HEAD        # Revert to last commit
## DOCS
GIT MAGIC http://www-cs-students.stanford.edu/~blynn/gitmagic/
## CREATE
### Create New Branch
    $ git branch experimental          # Create new branch (of current branch)
    $ git checkout experimental        # ..and entering into it
    $ git push -u origin experimental  # ..setup stream, and push back git

    
// might be the same as above
    $ git branch -b experimental          # Create new branch (of current branch)
    $ git push -u origin experimental  # ..setup stream, and push back git
### Create a new clone
 git clone git@github.com:franklangston/gpay1.git
 git clone git@github.com:oblinger/Cmd-Alfred-Extension
### Create cloud remote for existing repo
    $ open http://bitbucket.com
      {[create repo w. same name, as existing $PROJ_NAME, and copy repo URL]}
    $ cd /ob/proj/PROJ_NAME
    $ git remote remove origin
    $ git remote add origin git@bitbucket.org:oblinger/PROJ_NAME.git
    $ # 
    $ git push --set-upstream origin master


// NOTE:  error msg:  git@bitbucket.org: Permission denied (publickey)
//        (This means you are trying to login to the 'git' user not 'oblinger')
// On bit bucket (maybe github too?) 
// -1- Create Bitbucket repo with same name as existing $PROJ_NAME

	$ cd /ob/.../PROJ_NAME
	$ git remote set-url origin https://oblinger@bitbucket.org/oblinger/ufpy.git
	$ git push -u origin --all
### Create a remote
    $ git remote remove origin
    $ git remote add origin  https://github.com/oblinger/AnalyticsFireWebsite.git
    $ git remote add chris git@github.com:chrissrogers/gpay1.git
    $ git fetch chris
    $ git merge chris/changes/claim-spot-people-list
### Git checkout
   $ git checkout "c28b02f8be035e9fa27e544b74b3a54ad8777b9d"
   $ git checkout app/file.rb --theirs   # conflict 
   $ git checkout app/file.rb --ours
### git config  # Setup
   $ git config --list         # -> cat .git/config

   $ git config user.name "Dan Oblinger"
   $ git config user.email "dan@oblinger.us"
   $ git config --global alias.co checkout
     #  Creating a new local repository
     #    if needed   $ sudo apt-get install git
     # GET API TOKEN
     $ firefox http://github.com   # github@oblinger.us S5 
       -> Accout Settings -> Account Admin -> [cut API token]
       
     # CREATE A NEW LOCAL REPOSITORY
     $ cd ~/proj/PROJROOT         # cd to the root of the repository
     $ git init                   # creates .git
     $ git add .                  # recursively adds files to repository
     $ git commit -a -m "msg"     # commits files

     $ git remote add origin git@github.com:franklangston/<REPOSTORY>.git  # Tell local about the remote 'origin'
                            # defaults to ssh   ssh://git@github...
     $ git push -u origin master
### Git config GLOBAL settings (do this once per host)
     $ git config --global user.name "Dan Oblinger"       # Tell local your name (so commits are named)
     $ git config --global user.email code@oblinger.us    # and email addr too
     $ git config --global github.user oblinger
     $ git config --global github.token <API TOKEN>
### Setup SSH (repo/sshkeys)
#### Create SSH keypair & register w. github
    #   http://help.github.com/create-a-repo/
    
    # CREATE SSH DIR
    $ mkdir ssh
    $ cd ssh

    # Creating SSH Keypair
    $ ssh-keygen -t rsa -C "oblinger@gmail.com"   # I use "id_rsa" & S5 SHA256:Xp4spuOVd3VWgZh3F9KPpuw7YTqXVFrFLYt02ykv3mQ oblinger@gmail.com
      c5:1f:67:c5:02:69:49:d0:9d:ce:0c:5d:0a:fc:af:11 dan@oblinger.us
      The key's randomart image is:
      ...

    $ chmod 600 . id_rsa 
      
    # Registering on GITHUB (SSH KEY)
    $ firefox http://github.com   # Acc Settings -> SSH Public Keys -> "Add another public key"
    $ cat ~/.ssh/id_rsa.pub       # Paste this files contents into github key text field
    $ ssh -T git@github.com       # Test connection.  'you successfully authenticated, but git hub does not provide shell access
    # Registering on GITHUB (TOKEN)
    $ firefox http://github.com   # Acc Settings -> Account Admin -> [cut API token]
      (Later this token should be put into:  $ git config --global github.token <API-TOKEN> 
#### SSH keypair Instances
##### deploy@paybygroup.com
ssh-keygen -t rsa -C "deploy@paybygroup.com"
Generating public/private rsa key pair.
Enter file in which to save the key (/home/rails/.ssh/id_rsa): 
Enter passphrase (empty for no passphrase): 
Enter same passphrase again: 
Your identification has been saved in /home/rails/.ssh/id_rsa.
Your public key has been saved in /home/rails/.ssh/id_rsa.pub.
The key fingerprint is:
ad:ec:6b:85:1a:8f:ca:fc:49:34:a9:7c:dd:4a:02:08 deploy@paybygroup.com
The key's randomart image is:
+--[ RSA 2048]----+
|                 |
|                 |
|E                |
| . .   . .       |
|  . . + S..      |
|   . +.+.o.      |
|    o +==..      |
|   o oo=o.       |
|    +oo.+.       |
+-----------------+

##### webmaster@paybygroup.com  webmaster pass
7a:b7:c9:71:cb:54:9d:9d:71:a2:99:30:f4:00:f2:a5 webmaster@paybygroup.com
The key's randomart image is:
+--[ RSA 2048]----+
|      . ..+      |
|       o + o     |
|        E o . ...|
|           o + o#|
|        S   + .oo|
|       .     .   |
|      . . o o    |
|       . o B .   |
|          + o    |
+-----------------+
##### webmaster@paybygroup.com --OLD--  (on 11.11.10.tst1 w. webmaster password)
c1:ed:b2:20:d6:c8:81:73:10:e4:e0:ca:07:90:c7:a2 webmaster@paybygroup.com
The key's randomart image is:
 --[ RSA 2048]----
|o++.             |
|=ooo   . .       |
|.=+ o   o .      |
|E .+ +   o       |
|.. .= o S .      |
|  .. . . o       |
|        .        |
|                 |
|                 |
+-----------------

##### dan@oblinger.us  (on ubuntu)
    $ ssh-keygen -t rsa -C  "dan@oblinger.us"   # I use S5
      c5:1f:67:c5:02:69:49:d0:9d:ce:0c:5d:0a:fc:af:11 dan@oblinger.us
      The key's randomart image is:
       --[ RSA 2048]----
      |          .=+# +o|
      |         .  B.=o.|
      |          o..#+. |
      |         . . +E  |
      |        S   .  o |
      |              . .|
      |               o |
      |              .  |
      |                 |
       -----------------

##### webmaster@paybygroup.com ??? P:github password
The key fingerprint is:
cc:69:32:25:13:a3:e3:f5:2d:5e:d4:ce:4c:a4:86:98 webmaster@paybygroup.com
The key's randomart image is:    <-- in the 11.11.10 build
+--[ RSA 2048]----
|      o     .    |
|     . = . +     |
|    o E o + o    |
|   . o B = =     |
|    . o S o +    |
|       = o       |
|        .        |
|                 |
|                 |
+-----------------
#### Creating a new remote repository
     # CREATE A NEW REPOSITORY
     $ firefox https://github.com  # flangston@gmail.com  cm1
     ! New Repository
     ! Admin (with wrench @ top right) -> collaborators -> "oblinger
     $ firefox http://github.com   # github@oblinger.us S5 
       !  [unread message count by oblinger @ top] -> ! [allow push in message]
#### Creating a new local repository
     #    if needed   $ sudo apt-get install git
     # GET API TOKEN
     $ firefox http://github.com   # github@oblinger.us S5 
       -> Accout Settings -> Account Admin -> [cut API token]
       
     # CREATE A NEW LOCAL REPOSITORY
     $ cd ~/proj/PROJROOT         # cd to the root of the repository
     $ git init                   # creates .git
     $ git add .                  # recursively adds files to repository
     $ git config --global user.name "Dan Oblinger"       # Tell local your name (so commits are named)
     $ git config --global user.email code@oblinger.us    # and email addr too
     $ git config --global github.user oblinger
     $ git config --global github.token <API TOKEN>
     $ git commit -a -m "msg"     # commits files

     $ git remote add origin git@github.com:franklangston/REPOSTORY.git  # Tell local about the remote 'origin'
                            # defaults to ssh   ssh://git@github...
     $ git push -u origin master
#### Creating a new checkout of an existing GitHub repository
     # CREATE SSH KeyPair (see above)

     # GET API TOKEN
     $ firefox http://github.com   # github@oblinger.us S5 
       -> Accout Settings -> Account Admin -> [cut API token later when you need it]

     $ mkdir proj  ; cd proj
     $ mkdir gpay1 ; cd gpay1
     $ git init
     $ git config --global user.name "Dan on XXX"                 # Tell local your name (so commits are named)
     $ git config --global user.email code@oblinger.us            # and email addr too
     $ git config --global github.user franklangston
     $ git config --global github.token <API TOKEN>
     $ git commit -a -m "msg"     # commits files     # Needed?  currently fails cause no file to commit

     # Note: edit 'REPOSITORY' below!
     $ git remote add origin git@github.com:franklangston/REPOSTORY.git  # Tell local about the remote 'origin'
     $ git fetch                # just fetch it all
     $ git checkout master      # switch to the master branch
     $ git pull origin master   # not needed
     
     XXX? git checkout master
     $ bundle install
     $ rake db:migrate
## DELETE
### branch
   $ git branch -d some_branch             #   Remove LOCAL branch. warns if unmerged code
   $ git push origin --delete some_branch  # [DOES NOT WARN!] Removes REMOTE branch (on GIT HUB)
   $ git remote prune origin               # Removes LOCAL branches whose remote has been deleted
                                             # did not work 12/5/12
   $ git branch -D some_branch             #   Remove LOCAL branch. DOES NOT WARN
   $ git push [remotename] [localbranch]:[remotebranch]   # pushes local to remote so
   $ git push origin :<branchName>                        # thus this pushes "nothing" to remote branch  & deletes it
cd commits
    $ git add .
    $ git commit -a -m "msg"
### blast local branch
    $ git checkout feature/mgp_api
    $ git fetch
    $ git reset --hard origin/feature/mgp_api
## UNDO
### Reset
    $ git reset --hard HEAD         # blows away curent edits back to last commit
    $ git reset --hard HEAD~1       # blows away curent edits, #and# blows away last commit
    $ git reset        HEAD~1       # blows away the commit, but leave files intact.
    $ git reset --soft HEAD~1       # leaves files intact #and# leaves commit intact.  Just move HEAD back one.
    $ git reset
    $ git revert HEAD               # Creates a new commit that undoes actions of the HEAD commit## git committing
### undo since last commit
    $ git reset --hard              # blows away curent edits back to last commit
### undo edits to one file
    $ git checkout HEAD some_file.rb
### RECOVER LOST COMMITS
   $ git checkout -b recover_lost_commits
   $ git reflog
       e9c17db... HEAD@{3}: commit: Forgot to add 
   $ git merge e9c17db
## RECIPES
### Delete all remote branches merged into master

git branch -r --merged | grep origin | grep -v '>' | grep -v dev | grep -v master | egrep "fix/|feature/" | xargs -L1 | awk '{sub(/origin\//,"");print}'  |
 xargs git push origin --delete
## INFO
   $ git reflog                # LIST of commits
   $ git blame <path>
   $ git status                # Status of git system (branch, relation to local, & to github)
   $ git log                   # Log of commits on current branch
   $ git show-branch           # Lists commits and which branches they are part of
   $ git show                  # shows diffs???
   $ git reflog                # Show log of references committed
     $ git checkout -b brachName shaId   # this will checkout one of these as a branch  ???
   $ git diff
   $ git diff --cached
### Diff
   $ git diff master...topic     # Asymetric changes on topic since branched from master
   $ git diff --name-status master...topic     # Summary of diff by file

### Branches
   $ git branch --no-merged dev  # List branches
   $ git branch --merged master  # List branches that are fully merged into master
## BUGS
### You asked me to pull without telling me which branch you
     want to merge with, and 'branch.dev.merge' in ...

  $ git branch --set-upstream dev origin/dev


http://stackoverflow.com/questions/6089294/git-why-do-i-need-to-do-set-upstream-all-the-time
# === OTHER STUFF ===
## git merging
   # MERGE BRANCH BACK INTO MASTER
   $ git checkout master        # Go back to "master" branch
     $ git pull master # is this needed
   $ git merge branch_name      # Git merge experimental
   $ -- verify integrity --     # Verify bugs not introduced
   $ git commit -a -m "merged"  # Commit merged version
   $ git push -u origin master  # Push master back to github
   $ git checkout branch_name   # OPTIONAL:  If you want to stay on branch

   # PULL REQUESTS
   $ git pull https://github.com/franklangston/gpay1 merch_dash
   $ 
   https://github.com/franklangston/gpay1/pull/231
## git stashing
   $ git stash list / show / drop / pop
   $ git stash
   $ git stash pop
## git push
   $ git push -u origin master
   -u  (set up stream)
   -n  (dry run)
   -f  (force, if it is not a master)




// Examples
## Simple Usage Example
  #   http://carlopecchia.eu/blog/2008/08/26/git-by-example-a-little-tutorial/
  # CREATE A NEW REPOSITORY
  $ cd ~/proj/myProj           # cd to the root of the repository
  $ git init                   # creates .git
  $ git add .                  # recursively adds files to repository
  $ git config --global user.name "Your Name"       # Tell local your name (so commits are named)
  $ git config --global user.email you@example.com  # and email addr too
  $ git commit -a -m "msg"     # commits files

  # UPDATE BRANCH
  $ git branch                 # -> '# Master'   in master branch
  $ touch foo                  # make modificaiton
  $ git add .                  # 
  $ git commit -a -m "new foo" #
  $ git log

  # CREATE NEW BRANCH
  $ git branch experimental    # Create new branch
  $ git checkout experimental  # ..and entering in it
  $ ...make changes...         # add a new model
  $ git add .                  
  $ git commit -a -m "added rating model"

  # PUSH CURRENT BRANCH TO ORIGIN Branch w. same name
  $ git push origin HEAD
 

  # MERGE BRANCH BACK INTO MASTER
  $ git checkout master        # Go back to "master" branch
  $ git merge expeimental      # Git merge experimental
  $ -- verify integrity --     # Verfy bugs not introduced
  $ git commit -a -m "merged"  # Commit merged version
  $ git push -u origin master  # Push master back to github
## Other

  git repo-config --get-regexp user.#    # Verify config

  git init                               # setup git connection
  git add .                              # setup
  git commit -m "commit msg"             # Perform the commit

  git checkout .                         # Rebuilds system in current dir

  $ cd blog
  $ git pull 
    socialcrew

IP:http://50.57.158.140:3000/people/new
root: 6F6TT0jeo
SSH key:socialcrew
azat: 6F6TT0jeo
myPC SSH phrase: (blank)
## Notes
### Other stuff
### Install the most recent version
      http://evgeny-goldin.com/blog/3-ways-install-git-linux-ubuntu/
   $ wget http://kernel.org/pub/software/scm/git/git-1.7.2.3.tar.gz
   $ tar -xvf git-1.7.2.3.tar.gz
   $ cd git-1.7.2.3/
   $ sudo apt-get install libcurl4-gnutls-dev libexpat1-dev gettext libz-dev libssl-dev
   $ make prefix=/usr/local all
   $ sudo make prefix=/usr/local install
   $ which git
   -> /usr/local/bin/git
   $ git --version
   -> git version 1.7.2.3
### Load in a way that reload is possible
   $ git clone git://git.kernel.org/pub/scm/git/git.git
   $ cd git
   $ git tag
     ...
     v1.7.2.3
     v1.7.3-rc0
     v1.7.3-rc1
     v1.7.3-rc2
   $ git checkout v1.7.3-rc2
   $ make prefix=/usr/local all
   $ sudo make prefix=/usr/local install
   $ which git
   -> /usr/local/bin/git
   $ git --version
   -> git version 1.7.3.rc2


   # === LATER ===
   $ git fetch
     ...
     From git://git.kernel.org/pub/scm/git/git
     f17c1de..ac752fa  html       -> origin/html
     525f768..4693a7d  man        -> origin/man
     14d8729..87b5054  master     -> origin/master
     4a2aa5e..7bba3dd  next       -> origin/next
   + ce883eb...31195da pu         -> origin/pu  (forced update)
     46b1f9b..5fba457  todo       -> origin/todo
   # [new tag]         v1.7.3     -> v1.7.3
  $ git checkout v1.7.3
  $ make prefix=/usr/local all
  $ sudo make prefix=/usr/local install
  $ git --version
  -> git version 1.7.3
### From AZOT
   $ sudo apt-get install tcl
   $ sudo apt-get install tk
   $ sudo apt-get install python-software-properties
   $ sudo apt-get install zlib1g-dev
   $ sudo apt-get install build-essential autoconf
   $ cd /usr/local/src
   $ sudo wget http://kernel.org/pub/software/scm/git/git-1.7.5.2.tar.bz2
   $ # or download one from: https://github.com/github/git if kernel.org is down (has happened before)
   $ sudo bunzip2 git-1.7.5.2.tar.bz2 
   $ sudo tar xvf git-1.7.5.2.tar 
   $ cd git-1.7.5.2 
   $ make configure 
   $ sudo ./configure 
   $ sudo make && sudo make install
## AZAT

Goto Cloud servers on Rackspace, create server with Ununtu 10
Log into console with PuTTY using IP and root password

Use following instructions substituting versions to newer ones
http://www.freshblurbs.com/install-rails-3-ruby-1-9-nginx-and-mongodb-debian-lenny
http://freshblurbs.com/building-latest-git-debian-lenny

Use this to commit to git
http://www.alistapart.com/articles/get-started-with-git/
To create Public key follow:
http://help.github.com/linux-set-up-git/
Copy it into GitHub account
Push git master

Newer versions:
http://code.google.com/p/git-core/downloads/detail?name=git-1.7.7.tar.gz
http://www.sqlite.org/sqlite-autoconf-3070800.tar.gz
http://rubyforge.org/frs/download.php/75309/rubygems-1.8.10.tgz

For rake runtime error put gem 'therubyracer', require:'v8' into Gemfile
or use execjs (suppose to be compatible with Win)
For basic RoR project use http://guides.rubyonrails.org/getting_started.html#creating-a-new-rails-project
For zlib error try installing packages before compiling ruby!

More info on Git http://progit.org/book/ch1-3.html

Usefull Linux commands:
remove file $rm filename
copy $cp
ctrl+Z exit server
fg - foreground
bg - background
:q or :qw for vi
to check ssh $ssh -vT git@github.com
change password passwd

configurations on DevelopmentRoRAzat:
IP:http://50.57.158.140:3000/people/new
root: 6F6TT0jeo
SSH key:socialcrew
azat: 6F6TT0jeo
myPC SSH phrase: (blank)
# Apps
- git cola seems to work well for committing/pushing
- gitk seem to work the best for examining history and
- giggle is awesome for watching the diffs.

I use command line for committing, web trac interface for viewing history, in a rare occasions, 
and accept giggle as giggle is what, I think, one needs as a desktop git gui, 
with occasional git cola and gitk.

    # gitk - graphical history browser, in Tcl/Tk, 
      distributed with Git (usually in gitk package)
    # git gui - graphical commit tool, in Tcl/Tk, 
      distributed with Git (usually in git-gui package)
    # QGit - uses Qt toolkit
    # Giggle - uses GTK+ toolkit
    # git-cola - uses PyQt4
    # gitg - GTK+/GNOME clone of GitX
    # tig - text mode interface for git, is GUI and pager, uses ncurses

 # Options
    -a  ALL -- stage files modified and deleted; ignores unknown ones

 # Porcelain Commands
       git-add          --  Add file contents to the index.
       git-am           --  Apply a series of patches from a mailbox.
       git-archive      --  Create an archive of files from a named tree.
       git-bisect       --  Find by binary search the change that introduced a bug.
       git-branch       --  List, create, or delete branches.
       git-bundle       --  Move objects and refs by archive.
       git-checkout     --  Checkout a branch or paths to the working tree.
       git-cherry-pick  --  Apply the change introduced by an existing commit.
       git-citool       --  Graphical alternative to git-commit.
       git-clean        --  Remove untracked files from the working tree.
       git-clone        --  Clone a repository into a new directory.
       git-commit       --  Record changes to the repository.
       git-describe     --  Show the most recent tag that is reachable from a commit.
       git-diff         --  Show changes between commits, commit and working tree, etc.
       git-fetch        --   Download objects and refs from another repository.
       git-format-patch --  Prepare patches for e-mail submission.
       git-gc           --  Cleanup unnecessary files and optimize the local repository.
       git-grep         --  Print lines matching a pattern.
       git-gui          --  A portable graphical interface to Git.
       git-init         --  Create an empty git repository or reinitialize 
                            an existing one.
       git-log          --  Show commit logs.
       git-merge        --  Join two or more development histories together.
       git-mv           --  Move or rename a file, a directory, or a symlink.
       git-notes        --  Add/inspect commit notes.
       git-pull         --  Fetch from and merge with another repository or a local branch.
       git-push         --  Update remote refs along with associated objects.
       git-rebase       --  Forward-port local commits to the updated upstream head.
       git-reset        --  Reset current HEAD to the specified state.
       git-revert       --  Revert an existing commit.
       git-rm           --  Remove files from the working tree and from the index.
       git-shortlog     --  Summarize git log output.
       git-show         --  Show various types of objects.
       git-stash        --  Stash the changes in a dirty working directory away.
       git-status       --  Show the working tree status.
       git-submodule    --  Initialize, update or inspect submodules.
       git-tag          --  Create, list, delete or verify a tag object signed with GPG.
       gitk             --  The git repository browser.

 # Installation

 # GitHub
  - Once repository is open (on github)
    - Use 'wrench' in top bar to control sharing

