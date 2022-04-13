  [__DocSQL__](__DocSQL__.md)


# LOG
## 2013 post gres access
mysqldump -u rails -p---- -h   gpay1_production
http://www.eskimo.com/~ericj/comp/postgres.htm
## 3013 DB ACCESS IN RAILS


ActiveRecord::Base.connection.execute(
  "SELECT * FROM transaction_record WHERE group_purchase_id=479 LIMIT 2")

ActiveRecord::Base.connection.execute(
  "SELECT * FROM purchase_users WHERE group_purchase_id=560 AND user_account_id=1135")


# BASIC SQL COMMANDS

Basic MySQL Commands
What              	How                      	Example(s)
List all databases	SHOW DATABASES;         	SHOW DATABASES;
Create database 	CREATE DATABASE database;	CREATE DATABASE PhoneDB;
Use a database   	USE database;           	USE PhonDB;
List tables in the db	SHOW TABLES;            	SHOW TABLES;
Show the structure of a table	DESCRIBE table;
SHOW COLUMNS FROM table;	DESCRIBE Animals;
SHOW COLUMNS FROM Animals;
Delete a database (Careful!)	DROP DATABASE database;	DROP DATABASE PhoneDB;
 
## CREATE TABLE



# ACTIONS
  $ mysqldump -u rails -p... -h 184.106.167.65 gpay1_production >~/db-bk-12-07-28


# DB-SQLITE3

 $ sqlite3 /db/development.sqlite3

# DB-MySQL

MySQL Command-Line
What     	How                             	Example(s)
Running MySQL	mysql -uusername -ppassword     	mysql -ucusack2RO -pegbdf5s
Importing	mysql -uusername -ppassword < filename	mysql -usomeDB -pblah < myNewDB.sql
Dumping  	mysqldump -uusername -ppassword database [tables] > filename
  mysqldump -ume -pblah myDB > My.sql
mysqldump -ume -pblah myDB table1 
table2 > my.sql

# DB-POSTGRES
  Common Cmds:  http://www.thegeekstuff.com/2009/04/15-practical-postgresql-database-adminstration-commands/
  examples  http://www.ntchosting.com/postgresql/

## Setup
   (see DocMac Postgres)
## Command Line
  $ psql -U rails -h 184.106.167.65 lets2  # open db shell
       \help \q (quit)
  INFO > \list (DBs)  \d+ (describe table)  

## STEPS TO DUMP / RESTORE DB
  $ pg_dump <db-name> > filename                            # Dumps entire DB to file
  $ pg_dump -h 184.106.167.65 lets2 > lets2dump             # Dumps DB on remote machine
  $ dropdb -U postgres -h 184.106.167.65 lets2              # -U adminuser -O owner
  $ createdb -U postgres -O rails -h 184.106.167.65 lets2  
  $ psql -h 184.106.167.65 -d lets2 -f lets2dump            # restore
  $ psql -U postgres -h 184.106.167.65 lets2 < lets2dump
  $ cat db/database.yml  # for DB names
    \l   # lists all DBs

pg_dump -h 184.106.167.65 gpay1_production > $(date +"%Y%m%d%H%M").dump
pg_dumpall -U postgres -h 184.106.167.65 > all_$(date +"%Y%m%d%H%M").dump

  $ psql -h 184.106.167.65 -U postgres      # OPEN CONSOLE CONSOLE
    # Kill all connections to the DB
    > SELECT pg_terminate_backend(pg_stat_activity.procpid) FROM pg_stat_activity WHERE pg_stat_activity.datname = 'lets2';

