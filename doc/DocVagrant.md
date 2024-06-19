  [__DocVagrant__](__DocVagrant__.md)



# Dan Notes



# jotter vagrant install notes
## My Notes
#    failed to     pip install scipy
# what do do with the 'forward a local port' section

# Connecting to the couchbase in vagrant


  # ---- Admin Console starting
  $ (ignore port forwarding)
  $ open http://10.10.10.10:8091/

## Ari doc for creating a box
How to build Jotter's Vagrant base boxes with Packer
These instructions will guide developers and operations personnel through creating CentOS 6.5-based Vagrant base box machine images for use with virtual machine provider VirtualBox and for Amazon AWS. These Vagrant base boxes are required for setting up Jotter servers on VirtualBox for development and on Amazon EC2 for production. 
The Vagrant base box creation process in automated using Packer. Packer is an automation tool for creating identical machine images (Vagrant base boxes) for multiple platforms (Virtualbox VM, Amazon EC2, etc.) from a single source configuration.
The current configuration (see ~/jotter/ops/packer/CentOS-6.5.json) results in a CentOS 6.5 based Vagrant base box for virtual machine provider Virtualbox, version 4.3.12 (the latest as of writing). The box is based on a barebone CentOS 6.5 x86_64 Minimal ISO. However, it is provisioned with Virtualbox Guest Additions 4.3.12, as well as the latest versions of Chef and Puppet (we might remove these if we find no use for them in the future).
The definitions are based on the veewee project's definitions for a minimal CentOS installation, but with a few modifications:
•	The disk can grow to 40GB
•	Provides 4GB of swap
•	Fixes slow DNS resolution
Step-by-step guide to building a Vagrant base box for VirtualBox
1.	If you haven't already done so, generate and SSH identity on your host machine and save the public key to your Bitbucket profile. 
1.	Login to https://bitbucket.org/jotter-git. In the upper right corner, click on your account avatar > Manage Account. In the left navigation menu, click on “Security” > SSH Keys and add your SSH key there. 
1.	To setup SSH for Git on OS X, see https://confluence.atlassian.com/pages/viewpage.action?pageId=270827678.
2.	If you haven't already done so, clone required repositories on your host machine:
3.	cd ~
4.	 mkdir jotter
5.	 cd jotter
 git clone git@bitbucket.org:jotter-git/jotter-operations.git ops
6.	Download and install VirtualBox version 4.3.12 from https://www.virtualbox.org/wiki/Download
7.	Download and install an OS X or *nix version of Packer from http://www.packer.io/downloads.html
8.	Unzip the downloaded package into /usr/local/packer
9.	Add '/usr/local/packer' to your path (i.e. append it to your path in ~/.bash_profile: 'export PATH=$PATH:/usr/local/packer')
10.	Build the configured box:
11.	cd ~/jotter/ops/packer
 ./build.sh
The resulting output of a successful build is a Vagrant base box in the following format, CentOS-6.5-x86_64-v<build_date_in_yyyymmdd>.box, placed in the ~/jotter/ops/packer/builds/ directory. 
Step-by-step guide to building a Vagrant base box for Amazon AWS
Icon 
Currently, only builds for VirtualBox VM are configured.
TO DO Add build configuration for Amazon EC2
TO DO Build Vagrant base box for Amazon EC2

## Ari doc for setting up the box on a local machine
How to setup a Jotter server development environment
These instructions will guide developers through setting up a Jotter server development environment on an OS X host machine running a CentOS 6.5 guest as a VirtualBox virtual machine instance. The installation is in large part automated by Vagrant. However, some manual configuration is required on the part of the developer to complete the process. 
The installation requires 2GB of RAM and 40GB of hard disk space. A system with at least 6GB of RAM and 100GB of hard disk space is recommended. Reader familiarity with a Unix/Linux operating system is assumed. The steps below were tested on an OS X 10.9.2 host OS and a CentOS 6.5 guest OS running in a VirtualBox 4.3.12 virtual machine with Guest Additions.
Step-by-step guide
1.	Download and install each of the following applications. 
1.	Install VirtualBox from https://www.virtualbox.org/wiki/Download (download version 4.3.12)
2.	Install Vagrant from http://www.vagrantup.com/downloads.html (download the latest version)
2.	Generate and SSH identity on your host machine and save the public key to your Bitbucket profile. 
1.	Login to https://bitbucket.org/jotter-git. In the upper right corner, click on your account avatar > Manage Account. In the left navigation menu, click on “Security” > SSH Keys and add your SSH key there. 
1.	To setup SSH for Git on OS X, see https://confluence.atlassian.com/pages/viewpage.action?pageId=270827678.
3.	Clone required repositories on your host machine:
4.	cd ~
5.	 mkdir jotter
6.	 cd jotter
7.	 git clone git@bitbucket.org:jotter-git/jotter-operations.git ops
 git clone git@bitbucket.org:jotter-git/jotter-backend.git backend
8.	Begin automatic virtual machine installation process. Note, the first time you run this script it will take a while to download the approximately 400MB VM image and provision it with an additional 225 MB of downloaded libraries.
9.	cd ops/vagrant/virtualbox_vm
10.	  
11.	 # <edit> Vagrant file, modifying all instances of “<your_username>”; e.g. jsmith
12.	 vi Vagrantfile
13.	  
14.	 # This will take a while the first time it's executed    
15.	 vagrant up
16.	     
17.	 vagrant ssh
 sudo bash 
18.	Configure Postgres.
19.	# Start up postgres
20.	 service postgresql-9.3 initdb    
21.	 service postgresql-9.3 start
22.	 chkconfig postgresql-9.3 on
23.	  
24.	 # Create a database-user account.
25.	 useradd -c 'Jotter DB User' -m jotdbusr 
26.	  
27.	 # Set user jotdbusr's password to Fiat42_Jin79_Rul15   
28.	 passwd jotdbusr   
29.	 
30.	 # Create a database account for the user.
31.	 su - postgres
32.	 psql template1
33.	 CREATE USER jotdbusr WITH PASSWORD 'Fiat42_Jin79_Rul15';
34.	 CREATE DATABASE jotter;
35.	 GRANT ALL PRIVILEGES ON DATABASE jotter TO jotdbusr;
36.	 \c jotter
37.	 CREATE EXTENSION postgis;
38.	 CREATE EXTENSION postgis_topology;
39.	 ^D
 exit  
Edit /var/lib/pgsql/9.3/data/pg_hba.conf to allow users to use db stored passwords. Insert the following at line 75:
# Allow any IP to connect, with a password:
 host    all         postgres    127.0.0.1/32                  ident
 host    all         postgres    ::1/128                       ident
 local   all         postgres                                  peer
 host    all         all         0.0.0.0          0.0.0.0      md5
 host    all         all         ::1/128                       md5
 local   all         all                                       md5
# Restart postgres
 service postgresql-9.3 restart
40.	Configure user and group.
41.	# Add jotter group
42.	 groupadd jotter
43.	  
44.	 # Add user
45.	 useradd -c '<Firstname> <Lastname>' -G jotter -m <username>
 passwd <username>
46.	Edit /etc/sudoers to setup sudo users. Uncomment lines 105 and 108:
47.	## Allows people in group wheel to run all commands
48.	 %wheel ALL=(ALL)       ALL
49.	  
50.	 ## Same thing without a password
 %wheel       ALL=(ALL)       NOPASSWD: ALL
# Add sudo users to wheel group.
 usermod --append --groups wheel <username>
51.	Setup /etc/sysconfig/iptables. Edit and insert the following at line 11 (leaving existing lines in tact):
52.	-A INPUT -m state --state NEW -m tcp -p tcp --dport 80 -j ACCEPT
53.	 -A INPUT -m state --state NEW -m tcp -p tcp --dport 443 -j ACCEPT
54.	 -A INPUT -m state --state NEW -m tcp -p tcp --dport 4443 -j ACCEPT
55.	 -A INPUT -m state --state NEW -m tcp -p tcp --dport 4984 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 8080 -j ACCEPT
Lines 8 though 18 of /etc/sysconfig/iptables should look like this after the change:
-A INPUT -p icmp -j ACCEPT
 -A INPUT -i lo -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 22 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 80 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 443 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 4443 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 4984 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 8080 -j ACCEPT
 -A INPUT -j REJECT --reject-with icmp-host-prohibited
 -A FORWARD -j REJECT --reject-with icmp-host-prohibited
 COMMIT
56.	Setup /etc/sysconfig/ip6tables. Edit and insert the following at line 11 (leaving existing lines in tact):
57.	-A INPUT -m state --state NEW -m tcp -p tcp --dport 80 -j ACCEPT
58.	 -A INPUT -m state --state NEW -m tcp -p tcp --dport 443 -j ACCEPT
59.	 -A INPUT -m state --state NEW -m tcp -p tcp --dport 4443 -j ACCEPT
60.	 -A INPUT -m state --state NEW -m tcp -p tcp --dport 4984 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 8080 -j ACCEPT
Lines 8 though 18 of /etc/sysconfig/ip6tables should look like this after the change:
-A INPUT -p ipv6-icmp -j ACCEPT
 -A INPUT -i lo -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 22 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 80 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 443 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 4443 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 4984 -j ACCEPT
 -A INPUT -m state --state NEW -m tcp -p tcp --dport 8080 -j ACCEPT
 -A INPUT -j REJECT --reject-with icmp6-adm-prohibited
 -A FORWARD -j REJECT --reject-with icmp6-adm-prohibited
 COMMIT
# Restart iptables
 service iptables restart
61.	Configure rsyslog to have global jotter logging. Edit /etc/rsyslog.conf and insert the following at line 35 (leaving existing lines in tact):
62.	# JOTTER
63.	 if $syslogtag == 'jotter' then /var/log/jotter_all.log
 if $syslogtag == 'jotter' and $syslogseverity <= 4 then /var/log/jotter_alert.log
64.	Roll the new logs. Edit /etc/logrotate.d/syslog and insert the following at line 6:
65.	/var/log/jotter_all.log
 /var/log/jotter_alert.log
After a successful change, /etc/logrotate.d/syslog lines 1-13 should look like this:
/var/log/cron
 /var/log/maillog
 /var/log/messages
 /var/log/secure
 /var/log/spooler
 /var/log/jotter_all.log
 /var/log/jotter_alert.log
 {
     sharedscripts
     postrotate
         /bin/kill -HUP `cat /var/run/syslogd.pid 2> /dev/null` 2> /dev/null || true
     endscript
 }
66.	Configure connection between host and guest operating systems.
67.	exit    # To get out of root shell
68.	 exit    # To exit Vagrant shell
 vagrant halt
Edit ./Vagrantfile to setup shared directories. Uncomment line 7 and modify instances of <your_username> to the username created on the CentOS server above.
    config.vm.synced_folder "../../../backend/", "/home/<your_username>/jotter/backend/", owner: "<your_username>", group: "wheel"
As super user, edit /etc/hosts on your host (OS X) machine and insert the following line...
10.10.10.10     <your_vm_hostname>    # e.g. <username>001.jotter.vm
...somewhere between the following lines:
255.255.255.255 broadcasthost
 ### INSERT HERE ###
 ::1             localhost
# Reboot the VM
 vagrant up
69.	Configure Couchbase.
a. Open up a shell and forward a local port as follows: 
70.	ssh -L28091:localhost:8091 \
71.	     -L24985:localhost:4985 \
     <your_server_hostname> # e.g. <username>001.jotter.vm
b. Visit the following URL in a browser: http://localhost:28091/index.html and setup Couchbase with default configuration options. Set the following credentials for the Adminstrator account:
Username: Administrator
 Password: fiborcal72Tut
c. Visit http://localhost:28091/index.html and
1.	
1.	Delete the “default” bucket: Data Buckets > Expand chevron adjacent to “default” > Click Edit > Click Delete lightbox;
2.	Create bucket beer-sample-shadow (with ram quota of 100MB).
3.	Create bucket jotter (with ram quota of 1024M).
72.	Initial setup of Jotter server tree.
73.	ssh <your_username>@<your_vm_hostname>    # e.g. ssh <username>@<username>001.jotter.vm 
74.	 cd
75.	 cd jotter
 mkdir -p etc var/log
76.	Configure Jotter server.
77.	cd ~/jotter/etc
 cp ~/jotter/backend/etc/jotter-config.json ./
Edit ./jotter-config.json, changing line 18 from...
    “rootdir": "/var/log/jotter" 
...to...
    “rootdir": "/home/<your_username>/jotter/var/log”
78.	Setup the Jotter database.
79.	export JOTTER_CONFIG=~/jotter/etc/jotter-config.json
80.	 export PYTHONPATH=~/jotter/backend/pylib
81.	 cd ~/jotter/backend/ops
82.	 ./patchdb
83.	  
84.	 # Load the couch design docs
 ./loadddocs
85.	Run Jotter server programs.
In shell #1, run the Sync Gateway 
export TZ=US/Pacific
 cd ~/jotter/backend/sgw && ./run
In shell #2, run the Client Gateway 
cd ~/jotter/backend/cgw && ./run
In shell #3, run PAC 
cd ~/jotter/backend/pac && ./run
In shell #4, run the Brain 
cd ~/jotter/backend/brain && ./run
86.	Test everything through the web-based Jotter Client Gateway Test. 
1.	Navigate to http://<your_username>001.jotter.vm:8080
2.	Use the buttons on the Test Client interface to test communication with the server
1.	Click GenKeyPair
2.	Click Open
3.	Click Connect
4.	Click AuthenticateGoogle. This will open up a new window requiring you to login with your Google credentials and providing the app permissions to access your Calendar, Contacts, etc. After acceptance, you will be redirected to a page which contains a Google Authorization Code in a textfield. Copy this authcode and close the window.
5.	Back in the “Jotter Client Gateway Test” page, paste the code from the previous step into the “code:” field adjacent to the AuthenticateGoogle button, and click the AuthenticateGoogle button again.
At this point you should have seen stream of JSON responses in the Jotter Client Gateway Test page as well as corresponding verbose output in the shells setup in the step above. This indicates successful communication between the client and server.
 
For more information on Couchbase procedures, see URL (TBD).
For more information on Web-based test clients, see URL (TBD).
 



# Virtual Box
  # This this the VM software that Vagrant is running on.
  - ~# -space VirtualBox

# HowTo
## Setup Snipits
   $ vagrant                          # will show core commands
   $ vagrant init chef/centos-6.5     # this will create VagrantFile in current folder
   $ vagrant up                       # setup vagrant from VagrantFile in cwd
   $ vagrant ssh
   # See Jotter/vagrant/VagrantFile for example

## Setup  (Virtual Box, PyCharm)

  $  (install VirtualBox & Vagrant)
  $  add VagrantFile

  # Add this to your VagrantFile if chef_solo is missing
    config.omnibus.chef_version = :latest


  # Do this if you want to use 'omnibus' in your vagrant files
  $ vagrant plugin install vagrant-omnibus

## Run for Jotter backend
   $ cd ~/jotter/src/dev
   $ vagrant up
   $ vagrant ssh
   $ ls /ob
  
