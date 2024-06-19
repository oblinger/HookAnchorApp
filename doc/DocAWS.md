.[[DocAWS]], [[AWS Log]],
- [[AWS]] 


# How To

## EC2 DASHBOARD
- [AWS](spot://AWS) -> EC2
### Start Instance
-> Instances -> Instances -> !"dan-pipe..." 
  -> r! Start Instance

## SETUP
### Install Stuff
   - AWS toolbelt
     $ sudo pip install awscli
     
amazon AWS toolbelt   ec2-api-tools   (brew install)




### SSH
#### CREDENTIALS --  * .pem FILE
- Created new pair when creating instance

    $ chmod 400 *.pem       # Requires limited permit

#### ALIASES --  .ssh/config

Host opsg
    HostName 3.237.7.41
    User ubuntu
    ForwardX11 yes
    IdentityFile ~/ob/proj/sv/.creds/cv-ops-general.pem

#### LOGIN --  $ ssh -A -i ~/.ssh/cv*.pem ubuntu@3.74.93.28

    $ eval "$(ssh-agent -s)"

### BUILD ENV

#### GIT
    # git clone alg2 & data
    $ git submodule update
    
    
#### conda ???? build


## CONNECT 
    # Start "dan-pipe..." Instance
    $ _edit ~/.ssh/config     {add IP} 
    $ ssh -A -i ~/.ssh/cv*.pem opsg
