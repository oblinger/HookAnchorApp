

# LOG

### 2024-07-18  Juan GCP setup

- https://drive.google.com/file/d/1dZib2clQGfgrM_LUh17iEtkYbJMueJC9/view
- https://drive.google.com/file/d/1-prSTNsGlO7OySh0AA39mCM7mFvxcC88/view
   ixm-sgeg-wxy (2024-07-18 15:44 GMT-7) - Transcript

#### Attendees

Daniel Oblinger, Daniel Oblinger's Presentation, Juan Barajas


This editable transcript was computer generated and might contain errors. People can also change the text after it was created.

Daniel Oblinger: All back to…

Juan Barajas: Okay.

Daniel Oblinger: Bring this up. So I'm at the tonsil here. Let's go.

Juan Barajas: Okay. Yeah.

Daniel Oblinger: I'm not sharing it.

Juan Barajas: Yeah.

Daniel Oblinger: Screen there do it.

Juan Barajas: Okay, you could launch an instance and then you could also SSH into it using your own terminal for that. I think you need to set up the Google Cloud CLI.

Daniel Oblinger: Let's go to Google Cloud CI.

Juan Barajas: Okay. Let me get the link.

Juan Barajas: G Cloud CLI

Juan Barajas: there I sent the link in the meeting chat.

Daniel Oblinger: Okay, how many copy it?

Daniel Oblinger: Nice, okay.

Daniel Oblinger: Is there a download button here?

Daniel Oblinger: Macos

Daniel Oblinger: Okay, what do I do with this SDK?

Juan Barajas: I'm not sure. I haven't installed it with.

Daniel Oblinger: You have done yourself.

Juan Barajas: With my course now.

Daniel Oblinger: I see you do on a Linux.

Juan Barajas: Yeah.

Daniel Oblinger: Let's get this out already.

Juan Barajas: what it looks like it's tar

Daniel Oblinger: Looks like it's the same. I've already untarred it. Just I've already got it a folder here in my downloads directory.

Juan Barajas: okay.

Daniel Oblinger: So let me just bring up. our

Daniel Oblinger: not that okay, the

Daniel Oblinger: okay, do I need to move this someplace, or I just run it from here and it's gonna actually Just be stored someplace permanently. Looks like it wants it to be in my root directory. Is that right?

Juan Barajas: I think you can just exit out of that folder and then copy paste that command, but I don't think It needs to be.

Daniel Oblinger: Is it going to be permanent does it this because it's my download folder now is this gonna stay on my machine because I wouldn't want to permanently leave it there.

Juan Barajas: Not sure. Let's see. Let's check my installation.

Juan Barajas: Chic cloud

Daniel Oblinger: probably

Juan Barajas: chic

Juan Barajas: so it looks like under the

Meeting ended after 00:04:19 👋

  
**



#### ixm-sgeg-wxy (2024-07-18 16:06 GMT-7) - Transcript



Daniel Oblinger, Daniel Oblinger's Presentation, Juan Barajas, Juan Barajas's Presentation


This editable transcript was computer generated and might contain errors. People can also change the text after it was created.

Daniel Oblinger: Okay, so here we just have run docker

Daniel Oblinger: from fresh GCC instant got it. let's just Go ahead and take it from there.

Juan Barajas: okay, I think I'm adding a to-do note for me to add Konda and also set up Docker because It's not properly set up I think for our reposit people. want

Daniel Oblinger: It's up to you. What I'd like to do is to I don't mind setting it up with you now and saves you time just to go ahead and do it now, but on the other hand walking through manually isn't gonna get it set up. Do you want to actually just pause this recording and…

Juan Barajas: yeah.

Daniel Oblinger: I'll watch your screen while you set it up. It's probably more efficient and I'll take notes on the setting up. It's nice to have those notes on the setup as well. But I know what you're saying. What I'd like to do is get it to the point where whatever the instance is that people are going to be using we have notes for what you do from there. So you want to share your screen and I'll watch you get it set up. All…

Juan Barajas: Yep. Yeah. Yeah, I can do that.

Daniel Oblinger: Good deal. I'll stop sharing. I'll keep recording.

Juan Barajas: Hey.

Juan Barajas: Let me do that. It's gonna be a little bit more involved because I'm gonna update. Some other stuff as we go. but

Daniel Oblinger: Yeah, that's

Juan Barajas: so What you have to do or what I did to create to create an instance. Template is spin up VM and then tell it basically just spin up the VM and then once it's in a state where you want to freeze it to create a template you create an image and then you create an image from the disk of the VM that you just spun up and then you create a template from that image. so yeah,…

Daniel Oblinger: okay.

Juan Barajas: so the one that I used for this template is this one base pipeline Runner, so Let me just start at the end.

Daniel Oblinger: gaining a new gcp image

Juan Barajas: and let me open the instructions to set up Docker and to install.

Juan Barajas: While it's on the condensed fine

Juan Barajas: And then here I created a page. to set up gcp It's coach gcp setup. And I have the instructions for Docker here. So we're gonna follow those.

Juan Barajas: base pipeline Runner

Daniel Oblinger: Are these docs somewhere that I can get to?

Juan Barajas: Yes, they are on. CV

Daniel Oblinger: It's drop the link into the chat here. I' go there. I'll edit your notes instead of creating my own notes. Yeah. Why?

Juan Barajas: great. Yeah.

Juan Barajas: right, so The instance is now up based by plane Runner. but it typically do is I click on this. And then I view G Cloud command. Copy it.

Daniel Oblinger: And that's the G Cloud command to do what?

Juan Barajas: Who SSH into the instance? gcode compute SSH specifies the zone Yeah.

Daniel Oblinger: I Got it.

Daniel Oblinger: The gcp setup so I want to make a note here. So this is because you've got CLI and Docker written here. But what I think I want to do is I want to say create new image. So you would describe that is what we're doing right now.

Juan Barajas: yeah. Yeah create image.

Daniel Oblinger: create new gcp image and then The high level steps are to basically first run existing image.

Juan Barajas: whatever instance you want to create the image from yeah.

Daniel Oblinger: I'm sorry who update into desired state

Daniel Oblinger: three is they image of this of instance or create template

00:05:00

Daniel Oblinger: that image that's weird, but I got it.

Juan Barajas: Yeah. a little weird Okay, I'm gonna install mini condom. by running their script

Juan Barajas: I'm not sure if this is going to install it for my user or for every user. So maybe that's something that we need to check.

Daniel Oblinger: Why don't we build this image that you're building now? Why don't we have a single user? which is sports Visio something like that. And then we create a sports Vizio user and then we pseudo to that way every user can pseudo to the same one.

Juan Barajas: Yeah.

Daniel Oblinger: We let's just stop and think about it for a second. But my feeling is we don't want to be in a world where we are Putting stuff in our user directory If Google is doing that to us, we're gonna confuse ourselves to death if we have a bunch of everybody logs in has different credentials and things aren't lining up. I ought to be able just sudo right now, how do I create a new user on my little machine here? I should be able to camera. What's the latest command and create a new user?

Juan Barajas: You said let me look up.

Juan Barajas: Yeah you sir add so let's use her. Add SV.

Daniel Oblinger: That sounds good.

Juan Barajas: to use Randy user I wonder if there's a command

Juan Barajas: reference to use another user as reference And then I'm going to use my user.

Daniel Oblinger: I see. Yeah.

Juan Barajas: For permissions and whatnot.

Daniel Oblinger: right

Juan Barajas: specifically Orange see you surround.

Daniel Oblinger: For Linux user ad. Is there a way to use another user as a reference so that all the permissions and stuff gets copied over.

Juan Barajas: On this we create a group or something.

Daniel Oblinger: Did you show me that command?

Daniel Oblinger: Yeah, it's reference. So it's pseudo user ad reference existing user new user dash reference. so

Daniel Oblinger: so whatever're I guess. what does it say John?

Juan Barajas: John I just put John everywhere.

Daniel Oblinger: Really?

Juan Barajas: Yeah.

Daniel Oblinger: That's weird. Why do you do that?

Juan Barajas: Yeah, it's good. I did it once and then when I bought a new computer, I just did the same thing and it stuck.

Daniel Oblinger: But you name yourself the wrong name? Why?

Juan Barajas: nothing wrong name is just instead of Juan saw other people could have an easier time saying John so it's just created John. It's stuck. So I just use it everywhere.

Daniel Oblinger: When you're in other companies, do you call yourself John?

Juan Barajas: it's one always. Yeah.

Daniel Oblinger: Okay, yeah. I mean Juan's pretty easy dude, even press white boys. We can have a Juan, you…

Juan Barajas: Yeah.

Daniel Oblinger: there's definitely some names that are trouble but that's not one of them.

Juan Barajas: Greg and Greg probably has a bigger issue.

Daniel Oblinger: Yeah, I think you could do pseudo user reference.

Juan Barajas: Okay, I just added the user. I think you apparently can do something like

Daniel Oblinger: Are you doing another console that I'm not seeing?

00:10:00

Juan Barajas: no, it's on the same. It's this one, right? Because hello.

Daniel Oblinger: Yeah.

Juan Barajas: I think it looks like you can do something like V at AI pipeline launcher. Nope. The project property must be set up all the project ID.

Daniel Oblinger: that would only work if that user existed on that machine, it wouldn't create one on you as you did it that would be weird.

Juan Barajas: Yeah, but I think I already created the user.

Daniel Oblinger: I see.

Juan Barajas: Yeah, let's see. Any project this is each metadata.

Juan Barajas: So I ran a sudo user ad SV before logging out. So hopefully that works. permission

Daniel Oblinger: but You didn't do the reference trick, though. You created it you.

Juan Barajas: I didn't do the reference though. Let's see because I didn't see. Thing is I didn't find it on the documentation There's no preference. breath No.

Juan Barajas: because when you do shame on there is

Daniel Oblinger: I got it. Let me just ask. The version of Linux that I'm running doesn't have a dash reference.

Juan Barajas: yeah breath.

Daniel Oblinger: What do I use instead?

Juan Barajas: mmm

Juan Barajas: looking to

Juan Barajas: nope.

Juan Barajas: mmm

Daniel Oblinger: By the way, the system is asking me do I have a network proxy you'd like to set in gcloud? I don't really have a net proxy right? I'm just on a router locally on my machine, but there's no proxy here. That would be for a VPN, right? Yeah,…

Juan Barajas: I think so. Yeah.

Daniel Oblinger: so the answer is no.

Daniel Oblinger: So it's deep it wants to default. Yeah. Yes a network proxy is typically you.

Daniel Oblinger: Yeah, it's weird that they're defaulting me to yes, but it's no.

Juan Barajas: it's

Daniel Oblinger: It says Network diagnostic failed it. network errors detected

Daniel Oblinger: I don't think I want to continue.

Juan Barajas: nope. Mmm.

Daniel Oblinger: I'll be back in a minute.

Juan Barajas: Okay. Yep.

Daniel Oblinger: 

Daniel Oblinger: so if you

00:15:00

Daniel Oblinger: It's a very good she are anyway.

Daniel Oblinger: yeah coward how much later? So you said you'll probably already?

Daniel Oblinger: Yeah back.

Juan Barajas: Okay.

Juan Barajas: No log so far.

Daniel Oblinger: but you're trying to how do you

Juan Barajas: So the username if you do. The same thing except you do At the image you can get into the Ubuntu user.

Daniel Oblinger: Know let's just do that.

Daniel Oblinger: Let's just decide we use our Ubuntu in User. It's great. Yeah.

Juan Barajas: so we can just

Juan Barajas: yeah. Yeah, we can do that.

Daniel Oblinger: And maybe is the Ubuntu user and…

Juan Barajas: Okay.

Daniel Oblinger: admin on the machine?

Juan Barajas: No, but I think you can just do sudo into everything.

Daniel Oblinger: That's great. and is it a pseudoer?

Juan Barajas: it doesn't ask you permission. I think to do anything about txt

Juan Barajas: file taxi route. Yeah.

Daniel Oblinger: That's all right later. We can worry about it. I don't like running. Production code we're not assistant production code. so matter I don't like running production code from that because somebody can basically be able to hack that, pseudos

Juan Barajas: Yeah.

Juan Barajas: Maybe we should definitely Focus on other things I think currently we're storing the AWS. parameters in a file inside of here and everything so there are definitely some security aspects. Yeah.

Daniel Oblinger: Mess massive hole.

Juan Barajas: So, let me continue installing things. uniconda

Juan Barajas: There we go.

Daniel Oblinger: How did you install many condo?

Juan Barajas: Just copy pasted the quick commandment line install from their web page.

Daniel Oblinger: send me just these are really more for personal notes. Put that in the link to that in the chat.

Juan Barajas: Yep.

Juan Barajas: It is.

Daniel Oblinger: Thanks.

Juan Barajas: Then we're gonna do this.

Juan Barajas: again

Juan Barajas: There We have come back. Yep. and the other thing that I wanted to do is

Juan Barajas: Set up docker.

Juan Barajas: And to do so I think it should be relatively simple. which is to

Juan Barajas: run this command with this parameter

Juan Barajas: and We go. I thinking theory we should be able to.

00:20:00

Juan Barajas: download our latest image which should be on artifacts. registry and then SPI

Daniel Oblinger: when you do a mini-conda, you just do a conduit in it and kind of never remembers the environment that it's in so if you wanted to be in a particular environment You would need to put in your Bash conductivate. And is that

Juan Barajas: Yeah.

Juan Barajas: But yeah, that's right.

Daniel Oblinger: is that typical to people put on the activate in their Bash?

Juan Barajas: Yeah with Gonda. Yeah. I know there are some tools that can automatically activate it for you or if you use virtual environments, you can just prefix everything with. The tool that I'm using to manage the project run whatever and it's gonna automatically use the virtual environment. But we do it manually I think most people do it manually.

Daniel Oblinger: Got it.

Juan Barajas: At least us. Yeah.

Daniel Oblinger: what's happening here?

Juan Barajas: so I configured to set up the docker credentials to be able to pull our image and I just entered the artifact registry which is the place where we're uploading our pipeline images. And then just right Show pull command and copy this command.

Daniel Oblinger: And what is that? and that Command right there is pulling a Docker image Onto this machine.

Juan Barajas: Just to see that.

Daniel Oblinger: Not it just copying.

Juan Barajas: Yep.

Daniel Oblinger: It's not gonna do anything with it, but it's just pulling it down.

Juan Barajas: Yeah, that's right. Yeah.

Daniel Oblinger: And is it pulling it into a Docker registry on this machine?

Juan Barajas: It's pulling it to the local. Storage with Docker is looking for images. Not sure if you call it a registry.

Daniel Oblinger: Got it,…

Juan Barajas: But yeah.

Daniel Oblinger: and I actually can't tell is that already in your Docker Docs?

Juan Barajas: it's here, which is Docker first install it make sure you have

Daniel Oblinger: instead is calling this Docker. Let's make that a full sentence for what this is. This is

Juan Barajas: Okay, set up. for playing soccer images

Daniel Oblinger: it's not just pulling it. it's set up Linux for Docker run. that's what this is, right?

Juan Barajas: this first part is but the rest of it is specifically addressing the problem that Docker by default doesn't pull from our private registry.

Daniel Oblinger: Okay, but the meta task that's being solved by this item is set up Linux To run Docker images that's this whole category.

Juan Barajas: what the Run Sports video Docker images Yeah.

Daniel Oblinger: Okay, great. That's great. let's say that set up. posix

Daniel Oblinger: to Rhine SV Docker images

Juan Barajas: Yeah, Yeah, I was being I think. Because specifically what this does it does set you up for downloading the images from pulling them from our people because running it you can just run it with whatever Docker installation.

Daniel Oblinger: I see so politics to download

Juan Barajas: Yeah.

Juan Barajas: And then this should work. Hopefully, nope. connection permission denied I don't know why that is.

Juan Barajas: For your Tucker. He was East 4 Docker packaged.

Juan Barajas: on

Juan Barajas: yeah, but it's not letting me hold it.

Juan Barajas: I think the VM has to let me check some settings. because by default I don't think this is the case, but it doesn't allow you to use some gcloud apis for security. Let me check if that's the problem.

00:25:00

Juan Barajas: 

Juan Barajas: Okay, let me change that.

Juan Barajas: So before changing any config. You need to stop the instance.

Juan Barajas: It for it to stop.

Daniel Oblinger: And what you're trying to change here, you think that the permissions might be wrong on the default permissions might be wrong for doing the activities you want to do here.

Juan Barajas: Yeah, I think so.

Daniel Oblinger: that and here I have installed Docker desktop on Mac, but Even if I install the docker desktop on Mac, I couldn't really run the code very well on my machine. since I can't run the GPU, right?

Juan Barajas: I think you could if you use a config Yeah.

Daniel Oblinger: Right, but I can't do very much with the CPU config. I'm probably better off spending my time getting my machine running using.

Daniel Oblinger: A probably better you running on the gcp.

Juan Barajas: Yeah, probably. Yeah.

Juan Barajas: I just changed this setting hopefully that's the problem full access to all cloud apis.

Juan Barajas: I mean you could also pull the image and use CPU only for local development or a quick debugging or things like that. But yeah, it's not gonna be very useful.

Juan Barajas: And let's start the image again.

Juan Barajas: Lexus yep we wait.

Daniel Oblinger: Wait, you're starting what did you change the permission I kind of missed it.

Juan Barajas: Yeah, I stopped it. I clicked on edit. And then I changed this setting here. Under API in identity management Cloud API allow full access to all cloud apis.

Daniel Oblinger: Got it, and that's weird that it would not be set by default.

Juan Barajas: I think it's for security reasons. So if you want to enable something you have to explicitly enable it.

Daniel Oblinger: Got it.

Juan Barajas: So I think it makes sense. Okay.

Juan Barajas: And if we enter again. We should hopefully now be able to pull.

Juan Barajas: And no, okay. That was not the issue.

Juan Barajas: Yeah, I think I'm just gonna troubleshoot this offline

Daniel Oblinger: I don't mind. I'm kind of curious to see the troubleshooting.

Juan Barajas: Okay.

00:30:00

Juan Barajas: Let's see. because I think that's the only thing you need. She caught off configured docker.

Daniel Oblinger: Can you paste the error that you just got the permission denied air that you just got?

Juan Barajas: Yeah, for example.

Daniel Oblinger: Let's see…

Juan Barajas: look

Daniel Oblinger: if I can find something on it.

Juan Barajas: it's used.

Daniel Oblinger: Paste the command unit and the arrow a whole thing.

Daniel Oblinger: Thanks.

Juan Barajas: Welcome.

Juan Barajas: Let me see if the same thing happens. Actually, hold on.

Juan Barajas: That's under all going to darker. Let me see the same thing happens with my user.

Juan Barajas: say that this

Juan Barajas: and I do this.

Juan Barajas: all

Juan Barajas: Okay, so my user can pull things. It's just the Ubuntu user that you're not do it for some reason. Yeah.

Daniel Oblinger: I see.

Juan Barajas: of confuseducker

Juan Barajas: 22

Juan Barajas: experience

Juan Barajas: Yeah, that's weird.

Juan Barajas: She clown.

Daniel Oblinger: Here it's noticing that your current user may not have the permissions to access the docker demon. If you look at my screen the very top one here, I wonder that. that would actually

Juan Barajas: all

Daniel Oblinger: If that were not true of the Ubuntu user that would actually be the product.

Juan Barajas: you're right.

Daniel Oblinger: That would be a problem.

Juan Barajas: Yeah, you're right. Let's see. Yeah. Yeah, I think that's it.

Daniel Oblinger: to be precise Chad GPT is right, but I'll take

Juan Barajas: pretty clever. Yeah.

Juan Barajas: Okay, Yeah I performed the post-install instructions, but on my user back when I did that. So yeah, that's correct. But it doesn't tell you anything about it right doesn't. if it says soccer ball Permission the night will turn. no. No, that's right. Yeah, I should have read the message. Yeah. I think I did it didn't read it. correctly

Juan Barajas: because I thought the problem was that the permission was with the actual pulling of the thing. No. No, it's not the case. with accessing the payment

Juan Barajas: So, there we go. log back in

Daniel Oblinger: I see it was actually telling us that it couldn't access the Damon. What is this user model?

Juan Barajas: yeah, yeah.

Daniel Oblinger: I'm up familiar with that command. the user mod, right?

Juan Barajas: Which one? this one

Juan Barajas: I think it's from modifying a user and then here's just saying had it to the group docker.

00:35:00

Daniel Oblinger: I see really but users are only in one group, right? So when you add a user to a group you're removing them from another group. files are only in one group user can. right

Juan Barajas: Yes files. That's right. I think users can be in multiple groups.

Daniel Oblinger: Got it. Yeah, I definitely not my world.

Juan Barajas: Yeah, there we go. Look a pool and that works. cool so we're ready finally and then we're gonna be able to stop the VM at exactly the state that you want to leave it and then just wait.

Juan Barajas: again So, let's see.

Juan Barajas: Counting for we have the CLI setup. Create new gcp on instance. Yeah.

Daniel Oblinger: Create new gcp instances is kind of the umbrella, I was thinking of that as the umbrella.

Daniel Oblinger: Outline you might even put it above the other stuff. And then really what you're doing here. These are the sub. Things if that's the way you want to organize it.

Juan Barajas: I think it's fine there because I think it in theory. It's independent of everything else. You just so happens that we want it Docker inside of it with a CLI, but you could very easily just not choose to have those things when creating an image.

Daniel Oblinger: I say okay then these are some categories if you want to do those things got it.

Juan Barajas: Yeah, if you want to do it. Existing save image of this instance create template.

Daniel Oblinger: Yeah.

Juan Barajas: Yeah. It's pretty much.

Juan Barajas: So there by plane base pipeline Runners stopped we go to the image section.

Juan Barajas: We create an image.

Juan Barajas: I think I'm just going to delete the previous one. Because it's no longer that Base pipeline Runner just gonna delete it.

Juan Barajas: And I'm going to create another one that is named the same.

Juan Barajas: this pipeline Runner

Juan Barajas: I have to wait a little bit until this is deleted there we go.

Juan Barajas: It's pipeline Runner. Why?

Daniel Oblinger: I wonder if it just is not flushed out of the cash yet.

Juan Barajas: Yeah.

Daniel Oblinger: It's not listed.

Juan Barajas: It's not listed.

Daniel Oblinger: I would just give it another name…

Juan Barajas: Okay.

Daniel Oblinger: but still something very similar to that base pipeline image.

Juan Barajas: It looks like it's letting me know it used it for an error.

Daniel Oblinger: Okay. Good.

Juan Barajas: Right away.

Juan Barajas: And yeah, you can create an image from a disk from a snapshot from another image or from other stuff in this case. We're gonna use this. base pipeline Runner and create

Daniel Oblinger: Yeah. Yeah,…

Juan Barajas: and then we wait for the image to create and…

Daniel Oblinger: this is the stuff. I actually would.

Daniel Oblinger: 

Juan Barajas: So on.

Daniel Oblinger: So this is create new image.

Daniel Oblinger: Yeah. We get a chance it would be good to actually write that because the options that somebody else wouldn't necessarily use disk for example, but it's

Juan Barajas: Yeah, it's just our state.

Juan Barajas: Save image let's create image. from

Daniel Oblinger: yeah.

Juan Barajas: extents this

Juan Barajas: Source disc

Daniel Oblinger: select so image or you're really selecting. Yeah, first the first step is actually select the image, but I guess it's obvious.

00:40:00

Juan Barajas: So again, name So it is source and click the instance. this

Juan Barajas: and the drop down menu. Yeah.

Daniel Oblinger: That's just yeah, but What screen are you on when you do that select image name? That's the most important thing.

Juan Barajas: okay, let's go to

Juan Barajas: the images tab

Daniel Oblinger: That's good.

Daniel Oblinger: That's enough just as long as you get to the right the end the people…

Juan Barajas: wait

Daniel Oblinger: who are gonna read this or they know but they may not know the right tab.

Juan Barajas: Yeah.

Daniel Oblinger: Let's go.

Juan Barajas: Hopefully it takes a little bit. to create the image

Juan Barajas: create template from the image or just prepare this and say go to the distance simply have

Juan Barajas: it's templates.

Juan Barajas: and creating system plate

Juan Barajas: It's name of template.

Daniel Oblinger: interesting that ec2 listed in there

Juan Barajas: You see too…

Daniel Oblinger: I thought it did.

Juan Barajas: where you too.

Daniel Oblinger: E2.

Juan Barajas: Yeah, the instance like yeah.

Daniel Oblinger: Which insists paper we're using?

Juan Barajas: were you seeing gpus? and L4 so at G2 instance

Daniel Oblinger: That's the information we want to capture.

Juan Barajas: yeah.

Juan Barajas: shows now template

Juan Barajas: truce create settings

Juan Barajas: to settings

Juan Barajas: location where you seeing us East I think Us East 4. If I'm not mistaken.

Juan Barajas: no uses one.

Juan Barajas: Us East one

Juan Barajas: then Should be.

Juan Barajas: one before

Juan Barajas: she

Daniel Oblinger: This is that five fifty one sixteen. That's ly. yeah, it's monthly.

Juan Barajas: this one And then this is the same spec that we're using on AWS, but there's also this option that is slightly cheaper. Maybe we may consider.

Daniel Oblinger: just

Juan Barajas: problems from the moment is this one? three two standard 16

Juan Barajas: under what this?

Juan Barajas: Change it.

Juan Barajas: change

Daniel Oblinger: this thing is saying network connection problems may be due to proxy or firewall settings. Is there any chance that my router is somehow

Juan Barajas: Maybe do you have any fancy setup?

Daniel Oblinger: No, I don't understand my router.

00:45:00

Daniel Oblinger: Let me just asking.

Daniel Oblinger: I'm trying to set up a Google cloud on my Mac OS does the latest version of Mac OS have any kind of proxy settings or network filtering that my kid in the way our firewall.

Daniel Oblinger: Macos does have built-in firewall and network settings that could potentially interfere with network connections. Here are a few things to check to ensure they are not causing issues. One firewall settings go to system preferences security and privacy firewall. Make sure the firewall is either turned all or properly configured to allow incoming connections for your applications, including G Cloud to the proxy settings. Go to system preferences select your Active network connection EG Wi-Fi or ethernet and click Advanced go to the proxies tab and ensure no proxy settings are enabled unless you specifically need them three Network filtering.

Daniel Oblinger: Check if you have any network filtering or security software such as little snitch or an antivirus program with network protection features installed that might block or filter Network traffic by ensuring these settings are correctly configured. You can avoid potential Network related issues. while setting up Google Cloud on your map

Juan Barajas: Okay, I think that should be it.

Daniel Oblinger: great if you encounter a

Daniel Oblinger: she wants to talk to anybody.

Daniel Oblinger: I don't think it's anywhere at any Luhan my system

Juan Barajas: Yeah in theory That's the instance template creation.

Daniel Oblinger: Let me try it on my side for a second.

Daniel Oblinger: I'm going to disconnect my hard link. I don't.

Daniel Oblinger: I'm not sure if I should not. Just wash my hands and go any crap. It's not gonna make any difference.

Daniel Oblinger: Yeah.

Daniel Oblinger: You have to be on this call. pretty much

Daniel Oblinger: I pulled myself off of my hard Network. But I don't seem to have any proxies or anything like that going on.

Juan Barajas: Whichever is it showing up.

00:50:00

Daniel Oblinger: It's basically I'll just share my screen for just a second.

Daniel Oblinger: You can see down here. it's saying that it can't reach lots of things HTTP lived to cannot reach accounts Google calm. I mean with the hack the only thing is those were I just pulled off my hard.

Juan Barajas: mmm

Daniel Oblinger: Link, I'm just on Wi-Fi now. I don't know if it confused it to run.

Daniel Oblinger: I don't know why I wouldn't be able to get those.

Daniel Oblinger: With there's no reason yet.

Juan Barajas: cannot

Daniel Oblinger: It wants to default to here for G Cloud, but I've got no network proxy to give it. So if I hit yes, I don't know what it's gonna do. I'm not gonna have anything to get it.

Juan Barajas: yeah, if you click so, that's the error that shows up if you click no.

Daniel Oblinger: No, I haven't clicked now.

Juan Barajas: 

Daniel Oblinger: Yeah, I think the error was the reachability error there I can rerun it. with my Wi-Fi

Daniel Oblinger: Wait, it's just passed. That's weird.

Juan Barajas: What?

Daniel Oblinger: I pulled my hard link out. I wonder if there's some kind of firewall in the router if you come through a hard link. It seems like to be fewer limits on a hard link. So I guess I can try this and…

Juan Barajas: that's

Daniel Oblinger: see what happens.

Daniel Oblinger: So you have your side? your image is

Daniel Oblinger: Here, I think I unshared your screen.

Juan Barajas: my screen

Daniel Oblinger: Yeah, I think I unshared it Sorry about that.

Juan Barajas: okay, let me we share it and See, so we're pretty much done, but I can launch a new instance from the newly created template.

Daniel Oblinger: Okay, or maybe I launch it and a new instance from the …

Juan Barajas: yeah. Okay, yeah.

Daniel Oblinger: let's go ahead and do it that way. I'll share all right tires screen share Get the chats you can see.

Daniel Oblinger: So here I am. Let me reload this I should find your new.

Daniel Oblinger: I don't want the embass runner. I want to run yours. So I should really stop this one, right?

Juan Barajas: Yeah, you should probably even delete it. Yeah.

Daniel Oblinger: just delete it. Okay.

Daniel Oblinger: And then I want to go to VM in instance templates, right? Okay.

Juan Barajas: Yep.

Daniel Oblinger: And then I should find Bass Run. And I'm gonna say create VM.

Daniel Oblinger: Okay, and then here I'll say.

Daniel Oblinger: And Baseline Runner, by the way here. It says Would you like to log in? Yes. And then I guess I go to Chrome Beta. of course physio

Daniel Oblinger: to you allow all that stuff.

Daniel Oblinger: authenticated

Daniel Oblinger: Okay, I'll switch back.

Daniel Oblinger: He's probably telling me the names already in Dan based pipeline just called that. east 4 gpu-1 G2 Everything is the way I want it. I just go ahead and So I hit create instance creating.

00:55:00

Juan Barajas: Yep.

Daniel Oblinger: the pipeline

Daniel Oblinger: so this is the moment this is

Daniel Oblinger: I'm on this. Just to remind me, okay?

Daniel Oblinger: I guess I'll reload again and see if it's up. Are you able to do it from your side?

Juan Barajas: Launch the incense.

Daniel Oblinger: yeah, do I say start here or is it already starting by itself?

Juan Barajas: I think it starts up by default.

Daniel Oblinger: Got it some of the do anything. then I just…

Juan Barajas: I don't think so. Just wait a little.

Daniel Oblinger: but once I get it, I'm gonna click SSH. Is that right?

Juan Barajas: I think so. Yeah.

Daniel Oblinger: yeah, okay cool and then down below looks like it's trying to init my Google Cloud environment here.

Daniel Oblinger: Were you able on your side to create a new instance of this?

Juan Barajas: mmm

Daniel Oblinger: of this image

Juan Barajas: I'm just checking the cost of the storage because it looks a little high. it's a hundred and two dollars a month, which I'm not sure if

Juan Barajas: there's a cheaper option.

Daniel Oblinger: For every one of these images.

Juan Barajas: Yeah, because every time you create one of those if you just leave it there, I think even if it stopped they're gonna bill you for storage.

Juan Barajas: So I don't know either we choose a smaller.

Juan Barajas: Storage or just a cheaper one? I think there may be a cheaper option.

Daniel Oblinger: Smaller storage what I don't really understand what it means isn't it the size of the image that you're being charged for?

Juan Barajas: So each time you create an image. It's going to attach a disk Are one terabyte maybe SSD disk and I think even if you stop the instance that disk is going to persist until you delete the instance.

Daniel Oblinger: I see.

Juan Barajas: Yes, I'm gonna check.

Juan Barajas: balanced persistent disk

Juan Barajas: At least they think that's the case. Let me look up.

Juan Barajas: Doing yeah.

Juan Barajas: storage

Daniel Oblinger: 

01:00:00

Daniel Oblinger: but now I

Daniel Oblinger: I didn't want to just ssh in this Dan though. I really want to or maybe I can just let's see if I can so. sort of Sue

Daniel Oblinger: unto

Daniel Oblinger: or no that I want to pseudo suit Ubuntu right?

Juan Barajas: I think so.

Juan Barajas: Did it work?

Daniel Oblinger: 

Daniel Oblinger: It looks like it.

Juan Barajas: it did. Yeah.

Daniel Oblinger: Yeah, let's do CD tilde. And there I am and then I'm Mr. Ubuntu. Okay, so let me take notes here. So I want run Docker image from fresh from

Daniel Oblinger: Bass What do you call it runner base? pipe

Juan Barajas: a pline Runner base pipeline Runner. Yeah.

Juan Barajas: with dashes

Daniel Oblinger: Okay.

Daniel Oblinger: and then from here. Pseudo Sue. Ubuntu CD Tilda then what?

Juan Barajas: Okay.

Daniel Oblinger: would I what? We probably shouldn't have just created the instance yet because we probably want to go ahead and get this instance already set up so that it's already connected So we can do it get pull and…

Juan Barajas: Yeah.

Daniel Oblinger: everything should be set up, right?

Daniel Oblinger: Yeah, Should we do that now or should we just redo yours and actually add that in there? seems like

Juan Barajas: was the problems that I think I'm gonna have to jump off or briefly

Daniel Oblinger: no problem.

Juan Barajas: But I could do it I could do it tomorrow.

Daniel Oblinger: Yeah. what do I need to do here just to do it myself. Is it trivial to do the

Daniel Oblinger: am I going to basically go and things I would be doing it as my own user. We almost want to create a user on.

Daniel Oblinger: Almost we want to create a git user. I don't know. What we would want to do here. Would we want to get user to bun to?

Juan Barajas: if you just clone the repo with the generic Sage command, as long as you're SSA agent has access and is hosting your key. It should work. So if the Upstream URL is generic enough, you can just forward ussh agent and just get pull and get status and get whatever.

01:05:00

Daniel Oblinger: But from here, how would I forward the SSH agent?

Juan Barajas: I'm not sure. in your console,…

Daniel Oblinger: right

Juan Barajas: you can provide an extra argument that is arguments or SSH jargs or something and then you can just add the dash a

Daniel Oblinger: just for a second while we're here. It's asking me pick Cloud project to use. I assume I'm doing AI worker launcher. Yeah.

Juan Barajas: Yep.

Daniel Oblinger: And what the heck is this project thing?

Juan Barajas: So gcp organizes itself into organizations and projects the organization being Sports Physio and then the project AI work launcher you can add.

Daniel Oblinger: And then all the resources I use are getting build to that project.

Juan Barajas: Yeah, yeah I think in this case if you specify a project at the very beginning it just sets up your default dash project option for every command that you do but you can do specify some command to run for a different project and then scope it that way.

Daniel Oblinger: God got it, but let's think for a second how we want to arrange this common. In environment because I do think it would be really valuable to our team if we could just grab a fresh instance boom pull it the whole thing. But in that case we need to be able to get pull without screwing around with credentials forwarding stuff. So maybe we create a sport. I think we have a sports Vizio user admin or something like that for Sports Physio

Juan Barajas: But why do you say it?

Daniel Oblinger: I wonder go ahead.

Juan Barajas: What's the motivation?

Daniel Oblinger: what I'm wanting to do is set it up so that this Environment that we're doing either it's inside the Google either it's inside the docker image itself is the credentials I want to have the credentials to get pool not being added at the time that we create this thing or…

Juan Barajas: Yeah.

Daniel Oblinger: at least if we're having them at the time we created it. We're adding it in a common way.

Juan Barajas: Yeah, that's why I was mentioning the forwarding ussh agent. So that's exactly what I'm doing actually on the instance that I'm using for publishing or Docker images. So let me spin that up and show you how I've got it set up.

Juan Barajas: Yeah, pipeline publisher this one. Yeah.

Daniel Oblinger: but it just means you couldn't really use the web interface at all if you did that.

Juan Barajas: Yeah.

Daniel Oblinger: mmm

Juan Barajas: Which I don't know how many people. Are using the web interface?

Daniel Oblinger: No, I just like to have it if I can or I have it if you want it.

Juan Barajas: you may also be able to provide a custom command to web interface, but I'm not sure. how to do that

Juan Barajas: Let me just stop an angel. Here's instance between before I forget.

Juan Barajas: Okay.

Juan Barajas: same

Juan Barajas: my pipeline. Sure.

Daniel Oblinger: Should I use USA East one BC or D

Juan Barajas: I don't think it matters. Let me check.

01:10:00

Juan Barajas: I think I've been using C, and I know that it works, so maybe.

Daniel Oblinger: Yeah.

Daniel Oblinger: I'll stop sharing so you can

Juan Barajas: Okay it lets me.

Juan Barajas: Yeah.

Juan Barajas: I use a command

Juan Barajas: just get the name of the instance

Juan Barajas: if I went publisher.

Juan Barajas: And I have the repo any if I get URL of origin.

Daniel Oblinger: 

Daniel Oblinger: Before just to interrupt you just to show you…

Juan Barajas: it's just yeah.

Daniel Oblinger: where I'm at here. Does it look like?

Daniel Oblinger: So Am I actually on the instance?

Juan Barajas: where

Daniel Oblinger: No, I'm local here. No, I'm local I'm a bit confused about where I'm at, actually, so this I've done I finally went through the Google init.

Juan Barajas: Yeah.

Daniel Oblinger: I'm a little confused. What command prompt am I at?

Juan Barajas: Okay.

Daniel Oblinger: on my local machine

Juan Barajas: I don't know if you do.

Daniel Oblinger: my local machine, but it's weird. my prompt now has oblinger and then IP address in it. I'm not sure what that is.

Juan Barajas: I don't know what happens if you open. another terminal tab

Daniel Oblinger: I bet it's not there. Yeah. I mean they're oblinger at MacBook Pro.

Daniel Oblinger: I don't know what it's done, which we are. in any case it did go through the

Juan Barajas: Yeah.

Daniel Oblinger: I finally did the Cloud in yet. So now if I wanted to log into this instance what I go up here and actually

Daniel Oblinger: can I start the instance from here? I've got my Baseline Runner here. Can I actually?

Juan Barajas: I'm looking at a group Syrian nodes, I think.

Daniel Oblinger: but you're looking at wait a minute here.

Daniel Oblinger: With that's weird. Why are you not seen? I think it's frozen. it's just

Daniel Oblinger: I don't know why it's frozen. Let me just reload me share it.

Juan Barajas: Okay.

Daniel Oblinger: Yeah, so here I should be able to where's that command at that I can load it from the console?

Juan Barajas: Click on the dropdown Arrow next to the SSH button.

Daniel Oblinger: Early got it. Uh-huh.

Juan Barajas: And then view gqlo command.

Daniel Oblinger: You've got it copy this.

Juan Barajas: You just copy that.

Daniel Oblinger: No, Do I need passphrase here?

Juan Barajas: I think so. I don't remember because it doesn't usually ask me that so maybe that's the first time.

01:15:00

Daniel Oblinger: What pass rate should I be using here? Just one that I'm going to use in the future?

Juan Barajas: maybe because it looks like it's the best phrase for the SSH key that it generated for connecting to the instance.

Daniel Oblinger: He does not exist. You do not have an SSH key for G Cloud. This is a key. General will be XD generate key.

Juan Barajas: I think the very first time that you connect to your SSH to an instance, I think it may ask you for that.

Daniel Oblinger: Got it.

Daniel Oblinger: Okay, do you keep this RSA key this image here?

Juan Barajas: No, I don't know if I showed but I don't.

Daniel Oblinger: Right, I don't either I didn't mean to interrupt you. I just thought it would be good to keep this moving along because I think I need this working before I can follow the directions that you're doing right now.

Juan Barajas: Yeah, that's right. Yeah.

Juan Barajas: I was gonna show you that if you ask for the remote on URL, it's just generic. So I think anyone that connects and has their Sage agent forward their keys and their keys have access. I think they should be able to just get full I'm gonna

Daniel Oblinger: wait and see this.

Juan Barajas: Yeah.

Daniel Oblinger: Okay, so let me see what you just did get remote.

Juan Barajas: so if I

Daniel Oblinger: How did you do that though without giving it permissions?

Juan Barajas: if you put a generic link as origin, that is the one that shows Oakland you just clone the repo it automatically picks up your user and everything. Just based on your SSH.

Daniel Oblinger: I see but in order for that to work. I need to do that from the command line here. Got it.

Juan Barajas: Yeah.

Daniel Oblinger: Okay it's being slow over here. So I don't want to hit control C because it's probably doing something important. So we'll just keep going from your side and later. If I'm not able to follow this through we can look at it tomorrow, but from your side, you let me just see yours again. you were

Juan Barajas: So I just connected to whatever instance you want to this on and then you add this part. SSH flag and then the dash a Yeah.

Daniel Oblinger: That I got Where's the rest of that command? It can't start with term. Does it?

Juan Barajas: that's an implementation detail of my setup. It's just a problem that. You should probably ignore it.

Daniel Oblinger: You locked?

Juan Barajas: Let me just show this command.

Juan Barajas: That's this one. just changing the instance name at the end

Daniel Oblinger: I lost most of that. I'm gonna write it down here.

Juan Barajas: Okay.

Daniel Oblinger: You were saying it's an implementation detail. But really it's just G Cloud compute But…

Juan Barajas: Yeah.

Daniel Oblinger: what tells you which?

Daniel Oblinger: What tells you which instance you're connecting to because you're running This G Cloud compute is run from your local machine, right?

Juan Barajas: Yeah, it's on the very last argument. The one that says AI pipeline publisher. That's the name of the instance.

Daniel Oblinger: So in our case that's not even what yours is called. and base

Juan Barajas: then Baseline if you're doing the change, if you're just playing around with some things. I think you could use your instance. If you want to implement these things such that everyone is able to see it. I think you could use the other instance one that we use to create the image.

01:20:00

Daniel Oblinger: Right, but I wouldn't really what I'm trying to do is actually build the recipe for that's already been done. Now Briones is really who I'm thinking of owns would follow this Pathway to actually get an instance running.

Juan Barajas: Either he could do that or…

Daniel Oblinger: And so for that the very first thing he would do on his own machine.

Juan Barajas: just manually. I'm guessing the plan that we have is to just have the instance template to use SSH by default.

Daniel Oblinger: He's got to get gcloud setup though. I must admit for breon's man. It would be much better if we can give him a web interface. but for the moment assuming we can't do that.

Juan Barajas: But then maybe brions can just do a couple of extra steps and…

Daniel Oblinger: He's got to somehow get G Cloud running on his local machine,…

Juan Barajas: we say create your instance from this template that is already set up and already has a get repository and…

Daniel Oblinger: which sucks.

Juan Barajas: everything. And the only thing you're going to do is run these commands that is changing the remote URL to A URL that uses HTTP for authentication and…

Daniel Oblinger: But if he did that then he would execute this on, local.

Juan Barajas: he just puts in his email and password every time that he does anything with getpole get push.

Juan Barajas: Yeah, that's what I was thinking. So putting an origin that is generic, but it's generic in for SSH and then we just Celebrones to run a couple of extra commands to change that remote URL for another one that uses.

Daniel Oblinger: because on the instance. We could set up the

Juan Barajas: https instead of SSH

Daniel Oblinger: we could set up a get with an origin already on it in the instance.

Juan Barajas: Yeah.

Daniel Oblinger: I Okay. Got it. Yeah, …

Juan Barajas: Yeah.

Daniel Oblinger: we'll work through that. maybe tomorrow we won't try to do it today, but it would be good to work through that…

Juan Barajas: Great.

Daniel Oblinger: because I think it is gonna be nice to just not have to say. on his machine do this, but for the generic console do this, by the way, if you look over at my screen looks like I'm in. so here

Juan Barajas: Yep.

Daniel Oblinger: it looks like it's showing me that I'm on oaklandger. Dan pipeline

Daniel Oblinger: and I'm on Dan based Pipeline and now I would do pseudo Sue. Ubuntu

Juan Barajas: Yep.

Daniel Oblinger: So tomorrow we'll go through because I don't want to use this instance either actually probably want to go one more step further and we'll actually get that instance so that the number of steps we need to do in order to run.

Juan Barajas: Yeah.

Daniel Oblinger: It is really minimal and I actually think that's gonna be valuable for everybody because for whatever reason people are going to want to rebuild, from our latest image anyway, and so if we minimize the number of steps It's Gonna Save time for the team. We'll pick it up tomorrow. this was man, excited stuff. I'm excited too about the fact that minutes seem like they're freaking working. Thank God for something and yeah, I'm just gonna try to keep Everybody above us from shifting Focus get those things out.

Juan Barajas: No, I agree. and…

Daniel Oblinger: I know you're looking nervously at me like Dan,…

Juan Barajas: I think as soon as we get the results from The Unseen data,…

Daniel Oblinger: you're not going to start charts are you…

Juan Barajas: the minutes of Alton from James…

Daniel Oblinger: and…

Juan Barajas: if those results look good…

Daniel Oblinger: believe me. I'm really trying to hold myself off,…

Juan Barajas: then I'm gonna be more like, yeah, let's do something else.

Daniel Oblinger: but I do think the thing that's gonna be challenging for the team is Keeping all three of you guys and…

Juan Barajas: Let's see what else we can do. But until that happens, I think I'm a little bit nervous.

Daniel Oblinger: and me productively doing something on minutes without missing a beat. So it will be nice to have a background thing to do at some point.

01:25:00

Juan Barajas: Yeah.

Daniel Oblinger: And the other thing is once we get the results from The Unseen on minutes and…

Juan Barajas: Okay, great. See you tomorrow. Bye.

Daniel Oblinger: they look good is really getting into production yesterday. And so we really may even before we switch over to shot charts if there's things that we can delegate away from In getting things fully running on production we should do that first. and then we do it. Yeah. Awesome I will talk to you tomorrow.

Meeting ended after 01:25:19 👋

  
**
### [[2024-06-13 - Juan EC2 setup]] 
### m2022-08-16  Nick - RZ share price

- Assume drop in price removes tax free status.

Pay taxeszoo



### m2022-08-11  - Jobot Recruiter

- Scale:  Funding What is their scale?  What are they are looking for?  
- Size:  EngTeam, Report structure.
- Resume -  

### m2021-11-04 - NEATO CTO feedback

- not the perfect chemistry
- work in a different way; not a perfect fit
- very unstructured guy and neato is a very structured place
- Dan spoke about comp too much

_



