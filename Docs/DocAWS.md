  [__DocAWS__](__DocAWS__.md)
- [[AWS]] 


# How To
## INSTALL
   - AWS toolbelt
     $ sudo pip install awscli
## Setup Jotter backend on AWS
   LOGON  https://438223740058.signin.aws.amazon.com/console   1pass
      ??? does this find the right one???  - !EC2  !LaunchInstance  !AWS Marketplace   "centos 6.5"  {[must be EBS to boot]}
   - https://aws.amazon.com/marketplace/pp/B00IOYDTV6/ref=mkt_wir_centos65
     !continue   EC2instance = !M3-large

   SSH
   $ ssh jotter002.bonsai.com               # j002  x3
   $ cd jotter/backend/cgw; sudo ./run
   $ cd jotter/backend/pac; sudo ./run
   $ cd jotter/backend/brain; sudo ./run
   http://jotter002.bonsai.com:8080         # joclient
     !open  !connect  !authenticateDevice 

# CHAPTERS
## AWS TOOLBELT
   $ aws ec2 describe-instances

# zzzz** Console Actions
### Logon 
    - https://438223740058.signin.aws.amazon.com/console   1pass
    - bitbucket.org  O+S6


## Random notes
pick centos 6.5  

launch w. console

!launch in east

m3 large


- Find an instance to launch
- Launch an instance


> Find images in CentOS image (must be EBS boot)
  - centos ec2 image 
  - https://aws.amazon.com/marketplace/pp/B00IOYDTV6/ref=mkt_wir_centos65



    chmod 600 .pem

export EC2_KEYPAIR=${HOME}/.ec2/jotter-us-east-1.pem
ssh -o 'StrictHostKeyChecking no' -i $EC2_KEYPAIR -l root ec2-54-86-169-146.compute-1.amazonaws.com
# notes

# sec group  -- config firewall in router
! action  change sec group    DMZ

# assign IP addres
54.86.91.166  == jotter002.bonsai.com


# amazon AWS toolbelt   ec2-api-tools   (brew install)


