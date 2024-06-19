  [__DocRackspace__](__DocRackspace__.md)


file:all.org

## Rackspace Login details
Web:        http://manage.rackspacecloud.com/    U:socialcrew  P:rootpass replace with W..  (correct 12/09)
Email:      oblinger@gmail.com   flangston@gmail.com
LiveChat:   https://manage.rackspacecloud.com/LiveChat.do
ACCT:       630290 
Support:    877-934-0407
Favorite Color:  blue
Sales:  800 961-2888 

Hi Dan!
 
Thanks for your time today.  Here is the contact information for everyone.  Please let any of us know if you ever need assistance.
ndamore@rackspace.com –Nick D’Amore
yolanda.hernandez@rackspace.com – Yolanda Hernandez
michael.wheeler@rackspace.com – Michael Wheeler
 

## Instances

I changed what was prod-lamp to oldhp-lamp and its IP is 184.106.165.117
I changed what was dev-lamp to prod-lamp and its IP is 50.57.167.183
For the moment, the new prod-lamp is pointed at from
 dev.paybygroup.com and paybygroup.com until we get everything squared away.

### prod-lamp [paybygroup.com]  and dev.paybygroup.com
# was dev-lamp [DNS=dev] -- now has the wordpress site -om- dev.paybygroup.c
$ ssh root@50.57.167.183 Za1
$ ssh sc@50.57.167.183

$ mysql       U:root   P:me4RusuH
$ word press  root Za1
  /var/www/wp-config.php  <<-- pw for mySQL  
  /var/www/themes
    
### Prod-paybygoup   <<<< NOT ACTIVE
    --- not currently set as this DNS
$ alias sshpbg='ssh root@184.106.133.188'   Prod-PayByGroupF115HploF
$ http://paybygroup.com

### Prod-lets.paybygroup
$ alias sshlets='echo user ; ssh rails@184.106.133.37'
$ alias weblets='open http://lets.paybygroup.com'
Root is Za1   as of 2/13


### Prod-lets2.paybygroup
$ alias sshrootlets2='echo ZA1 ; ssh root@184.106.134.78' 
$ alias sshlets2='echo ZA1 ; ssh rails@184.106.134.78' 
$ alias weblets2='open https://lets2.paybygroup.com'
q1gC01XDilet2.paybygroup.com

### Prod-db.paybygroup
    $ alias sshdb='echo Za1 ;  ssh root@184.106.167.65'

    # to test DB connection (run from production machine)
    $ mysql -u rails -paSDckd30Je_09xP -h 184.106.167.65

### Test-test.paybygroup (Staging)
$ alias sshtest='echo Za1 ;  ssh rails@50.57.143.251'
$ alias webtest='open http://test.paybygroup.com'

# Full-2011-11-11
# ssh franklangston@50.57.143.251 userpass  (rootpass)
# test.paybygroupG0okK7Gt7
# http://test.paybygroup.com/
# http://50.57.143.251/

### Test-2
$ alias sshtest2='echo lookup pw ;  ssh root@50.57.121.238'
$ alias webtest2='open http://test2.paybygroup.com'

$ ssh root@50.57.121.238  Test21csQT4v4E

### Dev1
$ alias sshdev1='ssh rails@50.57.106.140'
$ alias webdev1='open http://dev1.paybygroup.com'

Dev15M4cCxrL5
$ ssh root@50.57.106.140  Za1
$ http://50.57.106.140/

### Dev2
$ alias sshdev2='echo Za1 ; ssh rails@dev2.paybygroup.com' 
$ alias sshdev2='echo Za1 ; ssh root@50.57.115.142' 
$ alias webdev2='open http://dev2.paybygroup.com'
Dev222Uf0vwAG
$ http://dev2.paybygroup.com
# From  Build-11-11-11-scratch-12-11-09  (note wrong scratch date)
$ ssh root@50.57.115.142   Aq1
# .ssh=Aq1 (dev2)
# 72:e2:e3:26:ec:60:61:93:f7:78:b4:ed:2f:95:4d:fa Dev2@paybygroup.com

### Dev3
$ alias sshdev3='echo Aq1 ; ssh rails@184.106.146.121'
$ alias webdev3='open http://dev3.paybygroup.com'
Dev3StH24b7Be

### Dev4
$ alias sshdev4='echo Aq1 ; ssh rails@184.106.147.27'
$ alias webdev4='open http://dev3.paybygroup.com'
Dev435l3aEdBH


### Dan Dev-Dan
$ alias sshdan='echo rootpass ;  ssh franklangston@184.106.195.10'
$ alias webdan='open http://184.106.195.10'

$ ssh root@184.106.195.10  DansDev0Woi4QkD1
$ ssh franklangston@184.106.195.10  userpass
$ http://184.106.195.10:3000/testing_pegboard/index
(see build for notes on CERTs)

### Frank -- Dev-Frank
$ alias sshfrank='echo userpass ;  ssh root@50.57.119.237'
$ alias webfrank='open http://50.57.119.237'
$ ssh root@50.57.119.237 FranksDevH0Ivs1M1i
 
### WP -- Dev-Wordpress
$ alias sshwp='echo Za1 ; ssh 50.57.227.173'
$ alias webwp='open http://wordpress.paybygroup.com'
$ ssh root@50.57.119.237 FranksDevH0Ivs1M1i
 
$ ssh root@50.57.227.173  Za1    wordpressGA6dG03pn
$ rails@ Za1


http://rubular.com/

### Scratch-Production
$ ssh root@184.106.153.78 Za1

### beta.paybygroup
# Base-2011-11-11
# ssh root@50.57.108.157  rootpass
# ssh franklangston@50.57.108.157  userpass
# http://beta.paybygroup.com
$ cd /var/www/beta.paybygroup/gpay1
$ sudo /usr/sbin/nginx -s reload 

### AzatFlow
$ ssh franklangston@50.57.115.58     AzatFlowBa03J3dEg
$ http://50.57.115.58/

### oldhp-lamp
# (was prod-lamp)
# Serving the homepage:      paybygroup.com
# ssh root@184.106.165.117    rootpass  (Za1)
# ssh sc@184.106.165.117     userpass
# http://paybygroup.com/
# http://184.106.165.117/
~# -->  MySQL, MyPHP passwords, 
#### Getting AZAT passaords
/etc/phpmyadmin
/etc/dbconfig-common/phpmyadmin.conf  <--- in here
/var/www/class/include.php            <--- pw in here

#### HTACCESS
 it is in some 'apache2' folder


#### MySQL
$ http://dev.mysql.com/tech-resources/articles/mysql_intro.html#SECTION0001000000
$ mysql -u phpmyadmin -p
  Maq9yEsP

dbc_dbname='phpmyadmin'
-----
	define('DB_HOST','localhost');
	define('DB_USER','root');
	define('DB_PW','me4RusuH');
	define('DB_NAME','prod');
-----



--- from /etc/dbconfig-common


--- from /etc/  config-db.php
## database access settings in php format
## automatically generated from /etc/dbconfig-common/phpmyadmin.conf
## by /usr/sbin/dbconfig-generate-include
## Sun, 30 Oct 2011 04:43:49 +0000
$dbuser='phpmyadmin';
$dbpass='
$basepath='';
$dbname='phpmyadmin';
$dbserver='';
$dbport='';
$dbtype='mysql';

### qa-lamp
$ ssh root@50.57.73.175 rootpass   (Za1)


## Images
### MySQL-12-01-06
# Ubuntu 11.10 (Oneric Ocelot)
# ssh root@50.57.180.189   MySQL-12-01-06Qo22tI1Ep



# Ubuntu 11.04 (Natty Narwhal) 
# U:root   P:Build-11-11-11P3U1hb2Ru
# ssh franklangston@50.57.178.83   P:userpass
# http://50.57.178.83/   P:userpass
# -> Nginx global install
# -> Git checkout gpay1
# gpay1 In: /var/www/beta.paybygroup/
$ sudo /usr/sbin/nginx   # will start the server
### Build-11-11-11
--- Base-11-11-11
--- Build-11-11-11-full-11-12-15   built 1 day after azat
--- Updates for mysql 2012-01-06
# Ubuntu 11.04 (Natty Narwhal) 
# ssh root@50.57.178.83            P:rootpass
# ssh franklangston@50.57.178.83   P:userpass
# http://50.57.178.83/
# -> Nginx global install
# -> Git checkout gpay1
# gpay1 In: /var/www/beta.paybygroup/
$ sudo /usr/sbin/nginx   # will start the server

### Full-2011-11-11
# build from beta.paybygroup (which is built from Base-2011-11-11)




## Dead Instance/Images
### Build-11-11-10-tst1
    # U:root P:rootpass P:Build-11-11-10-tst1A6p2xNjG1
    # U:franklangston P:userpass
    # ssh franklangston@50.57.108.157

### Build-11-11-10 
    >>> Rackspace build instructions v11.11.11
    # Started with Ubuntu 11.04 Natty Narwhal
    # U:root P:root password    P:Build-11-11-10M5juQ01xF
    # U:user P:user password
    # IP:50.57.106.148
    >>> Rail Build v11.11.11
    >>> Git.org -> Installation -> Create SSH keypair and register on github
### Build-11-10-20
(used for base 11-10-20)
### Base-11-10-20
IP: ssh -l user 50.57.158.194         http://50.57.158.194:3000/people
Build-11-10-20o2UU21chJ
Ubuntu10.04 LTS.  DanO 11.10 build script

### DanDev-1020ij0C6CRf2    50.57.158.211
### DoTest   Build-11-10-20o2UU21chJ
### DoTest2  ssh -l user 50.57.159.137  doTest2m6PBPk66t    

### Azat
DevelopmentRoRAzat:    IP:http://50.57.158.140:3000/people/new
ssh -l root 50.57.158.140  6F6TT0jeo
SSH key:socialcrew
ssh -l azat 50.57.158.140 6F6TT0jeo
myPC SSH phrase: (blank)
  $ cd blog
  $ sudo rails server --port=80
  $ firefox http://50.57.158.140/
  $ firefox http://groupconcerttickets.com
  $ scp azat@50.57.158.140:blog/public/concertMacMiller/images/eventsite2_18.png .



### developentRoRazat
$ ssh root@50.57.158.140


# How To
## Create a new server  v11.11.11
   W: Rackspace -> CloudServers -> ! Create Server
      - Select image, select size,
      - Record  password, IP address
   $ ssh root@IPADDRESS
     $ adduser user
       (enter same password as root)
     $ visudo                            # >>>>MUST HAVE A --TAB-- after 'user'
       >> user   ALL=(ALL) ALL
   $ exit                                # Exit back to local machine
   $ ssh user@IPADDRESS
   





Failing to delete an instance that is in and error state.
We have tried and failed to stop the instance at IP adress:  50.57.160.91
What do we need to do to delete that instance?


