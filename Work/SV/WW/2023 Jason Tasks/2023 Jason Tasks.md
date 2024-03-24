
- [[@Jason Syversen]]


# WW JASON TASKS

## JASON
### 2023-02-13  Systematic QA processing
- We should be automating QA 
- Testing matrix

## JASON Tracking

### J1.1 - Don't force all teams into programs
Right now new customers are teams, which are linked to programs and programs have events. This is nice, but sometimes events are standalone things and the customer could BE the event organizer. Forcing every team into an arbitrary program doesn't always make sense. 

### J1.2 - Scott byers
Scott Byers has designed our backend systems and while he's a fast coder who can right code well and find bugs quickly, he's not in touch with users, doesn't ask for help/input, and usually makes incorrect design decisions from my experience. I've told Brian that Scott should NOT be making any design decisions without sign off from me or Mark because his approaches usually create more problems than they solve. Brian has not managed him strongly enough and it has led to numerous issues. (Database structure, authentication issues, and many others) It's annoying because he literally taught classes on programming and I've talked to people who took them, as well as a former boss and everyone ranted about how smart/good he was. 

==> Will eval [[@Pablo Calofatti]] 	 perhaps another hire?


### J1.3 - Android BG uploader
We need to get the background uploader for Android in production (should be happening imminently) and iOS background uploader developed (not even started yet!) Background uploading is a key feature for customers that experience difficulty waiting for the app and interrupting it as they use the phone.

==> with our flutter strategy we should not hire more native OS.
     but maybe we consider strong consulting??
 

### J1.4 - Support 3rd party video sources
We are going to need to support third party video sources. At a minimum a hand picked wall mounted camera that streams to our systems from a static vantage point... but possibly arbitrary platforms too. While we don't NEED to support arbitrary third party video sources, it would be amazing if we could as we have a universe of existing customers that have existing video they could upload and we could analyze. But we definitely need to move beyond just mobile phones on a tripod to something permanently installed in many venues and nobody has even started on this (other than me doing some research on camera options and Brian looking at ones that support native streaming to AWS)

==> 

### J1.5 - Improve our QA
We need to improve our QA processes. We still have WAY too many bugs slipping through into production. It appears none of the engineers actually use the product, which I get as it's a lot of moving parts and most don't apply to them. But unfortunately those pieces end up slipping through into production and create issues that we keep discovering WAY too late

==> Many fixes here!


### J1.6 - Social video sharing
It's too difficult to share videos onto social media. I've been told that's going to be fixed but no idea when that's happening and I strongly believe that is a critical component to creating some viral growth engines as customers share their clips with others.

### J1.7 - Company watermarking on videos
Similarly, we still don't have a company watermark on the video clips. See #6

### J1.8 - Customer Experience Monitoring
We don't have enough customers using the app regularly. Mark doesn't seem to care about it but I care greatly... we need to figure out why that is. Mark has pointed out that we don't even know which customers sign up and what they experience, I think we need to add some customer tracking software into the app that tells us who signed up, who's using what, and give us feedback that allows us to improve the experience. 

### J1.9 - Dev Productivity
We don't do a good job tracking developer productivity. Obviously stuff like LOC counting is stupid, but I'd love to learn what you did at AF to track productivity and see what we can do to better analyze performance. Obviously we see things like code check ins and stuff that works or not, but quality and design decisions and other things are harder to measure. Especially with a bunch of younger developers I think some of them are less motivated and need some focus help/training (and in some cases to be replaced.) But maybe I'm wrong and they're all amazing. Would love to get your take on productivity but also the team quality/depth across the board. 

### J1.10 - Code Reviews
We need to start doing design reviews, which have not happened (again, an issue stemming from Brian I think) I'd like to review the architecture, key design decisions and make sure that customer requirements are being properly addressed in those decisions. Right now we have Mark making complaints or he and Jack adding stuff into Jira and then we find out later if the code that's in QA solves the problems or not, and it often does or doesn't but then also introduces other issues. I guess this is restating #10... but it's more than just Scott

### J1.11- Slow Feature Progression
There are a bunch of things that are supposed to be coming out but seem to be slow to get the internal wickets (and then still have bugs when they do!) A game summary page, a watch page where people can see the video, a streaming/watch live page, the aforementioned features like watermarking and sharing, the customer management page where they can manage rosters, etc.

### J1.12 - Customer Communications
We don't do a good job communicating with customers. Features, collaboration, community. We don't tell them when games are uploaded or available. Not strictly technical issues but sort of technical issues (as we can use the tech to communicate with people) Really want to do a better job there.
