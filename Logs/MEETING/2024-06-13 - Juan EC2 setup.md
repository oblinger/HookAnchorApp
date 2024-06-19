CV ML daily standup (2024-06-13 16:02 GMT-4) - Transcript
Attendees
Daniel Oblinger, Daniel Oblinger's Presentation, Juan Barajas, Juan Barajas's Presentation
Transcript
This editable transcript was computer generated and might contain errors. People can also change the text after it was created.
Juan Barajas: Yeah there. so let me Launch an instance.
Juan Barajas: I guess I'm going to use the instance that I use for testing stuff, that's a small CPU instance, but I guess that'll do for now.
Daniel Oblinger: So I'm just keeping track here. so you're literally just going to your main console and finding an instance and then just spinning it up. Okay.
Juan Barajas: Yeah, I can launch either the one that I use for testing GPU stuff for the one that is smaller and I use for just CPU stuff.
Daniel Oblinger: And we'll do this and maybe if we get time we'll go back and create a new instance. I would love to see that process as well.
Juan Barajas: Okay.
Juan Barajas: And should we use the smaller one or the GPU one?
Daniel Oblinger: And that way we can actually run something in it. If we want to we're not going to keep it up that long. How much do these spots this is cost per hour?
Juan Barajas: so this
Juan Barajas: I'm not. Sure. I think it's let's see.
Juan Barajas: It's about 1.6 box an hour
Daniel Oblinger: Yeah, got it.
Juan Barajas: and step one starting the instance enter the instance.
Daniel Oblinger: while we're there, what was the price for the small one? that was actually relevant for
Juan Barajas: So let's take a look at this so that small the CPU one I'm using is just
Daniel Oblinger: Yeah, what I'm looking for is one that James wants to just do downloads and uploads but many parallel downloads 10 Barrel downloads and uploads of the video file at the same time on a Windows box. That would probably be a pretty small instance, right?
Juan Barajas: yeah, I think that can be the tiniest one. yep.
Daniel Oblinger: holy s***, that's What he was said per hour.
Juan Barajas: That's per hour. Yeah. It looks like it. Yeah.
Daniel Oblinger: So it's a penny an hour. And yes you have Got it.
Juan Barajas: and in our
Daniel Oblinger: Okay, that's good to know.
Juan Barajas: Yeah, these ones there's no problem. I think. The small ones even get into the free tier. So if you only use up to so many hours, I think they're free.
Daniel Oblinger: But if you just sat there and did downloads, I guess you're gonna pay separate from that. You're gonna pay for your In Ingress costs, so that's how they're gonna get it covered.
Juan Barajas: Yeah,
Daniel Oblinger: And you sat there and did a bunch of downloads on that device. it's actually a lot more resource intensive than that price is suggests, but they're gonna make it up on the I/O costs.
Juan Barajas: so I don't know if this is the case now, but it used to be that if you do some Ingress, out Chris to egress
Daniel Oblinger: egress Yeah, an Ingress is good,…
Juan Barajas: Yeah. If you do both to the same Zone,…
Daniel Oblinger: too. Yeah.
Juan Barajas: I think it's free or it should be really used to be that way. I'm not sure if that's the case now.
Daniel Oblinger: Okay.
Juan Barajas: So yeah, let's get our instance copy the public AP.
Juan Barajas: let's go to my terminal.
Daniel Oblinger: And you just launched it. I didn't quite see there, but you just click the launch button.
Juan Barajas: Yeah, which is right click it and then start instance.
Daniel Oblinger: Got it.
Juan Barajas: Let me just have a igured. S h config that is just the host name the user or just forward x 11 and then set the identify identity file. Whenever you create the instance, it should generate the pen file for you to just point it to that file.
Daniel Oblinger: Got it. So I really won't be able to use your instance because I do not have your pen file.
Juan Barajas: Mmm Yeah, that's unless I share it with you.
Juan Barajas: Yeah.
00:05:00
Daniel Oblinger: Yeah, which is not ideal. Yeah, we'll see…
Juan Barajas: Mmm Yeah
Daniel Oblinger: how hard it is.
Juan Barajas: Yeah. Cool,…
Daniel Oblinger: Maybe the two of us with the end. We can actually create an instance from me and…
Juan Barajas: then once that's done,…
Daniel Oblinger: that way I actually have an instance I can start I've done it.
Juan Barajas: we can just a sage into it. I think was it you scratch?
Daniel Oblinger: It's been a long time but I even have notes on…
Juan Barajas: Yeah, I think so JB scratch.
Daniel Oblinger: how to create this and Pen files kind of in the history because now getting everybody else doesn't let you use those anymore. So I haven't really been doing them very much.
Juan Barajas: Yeah, you could just SSH and…
Daniel Oblinger: But I guess when you're running instances,…
Juan Barajas: then I think and…
Daniel Oblinger: you still are got it.
Juan Barajas: then the pen. Yeah, I am and…
Daniel Oblinger: Okay, I got it cool.
Juan Barajas: then you can just not forward x 11 the user which is Ubuntu and then the IP. And it should work.
Juan Barajas: It's just more convenient to have it on the SSH config.
Daniel Oblinger: you use it…
Daniel Oblinger: but you could have just SSH to that IP address and…
Juan Barajas: but yeah,…
Juan Barajas: This is my machine.
Daniel Oblinger: that would have
Juan Barajas: I think it should have a GPU. So yeah step one. We're here. what's next?
Juan Barajas: Pull …
Daniel Oblinger: Got it.
Juan Barajas: I forgot to forward my SSH agents.
Daniel Oblinger: Got it.
Juan Barajas: Which is not such a great idea, but it's very convenient.
Juan Barajas: I saw the algorithms.
Daniel Oblinger: Okay.
Juan Barajas: too promote URL I have it set to use SSH.
Daniel Oblinger: let's pull down the latest on ALG, too.
Juan Barajas: So it's going to look for my SSH agent to pull the keys and then allow me access to the pool on algorithm still and so I just SSH into the instance with the dash a argument which is going to forward my SSH agent.
Daniel Oblinger: what is it that you're doing here? I'm not following.
Juan Barajas: So it's going to use the keys from all my local machine, but on the remote machine.
Daniel Oblinger: whoa so when you do an SSH Dash It's going to give you a console on the remote machine and…
Juan Barajas: Yeah, that's right. I think it either forwards the keys right…
Daniel Oblinger: somehow magically it has transported your private keys.
Juan Barajas: then just forwards the connection to my SSH agent. So my keys never leave my machine.
Juan Barajas: I'm not exactly sure…
Daniel Oblinger: Over probably on the file system.
Juan Barajas: but I think you can verify with You grab for agents Maybe?
Daniel Oblinger: It's probably just Inside that console somehow magically those keys are available to the SSH client in that console over there.
Juan Barajas: Maybe SSH agent info something and…
Daniel Oblinger: Is that right?
Juan Barajas: then let's do the same thing here. Being grabbed by a chance.
Daniel Oblinger: wow.
Juan Barajas: Yeah, so I think it's just a socket. Such that this thing can connect to my local SSH agent. Yeah.
Juan Barajas: To my machine. Yeah.
Juan Barajas: Yep. I'm not sure how recommended forwarding your SSH agent is. Not sure…
Daniel Oblinger: It's a pipe.
Juan Barajas: what the best way to do or…
Daniel Oblinger: So basically any SSH you do is going to use that socket to open up multiple SSH pipes through so the data traffic actually is flowing through your machine then.
Juan Barajas: Way to do it.
Juan Barajas: yeah.
Daniel Oblinger: Okay. Got it. come
Juan Barajas: Yeah.
Daniel Oblinger: So it seemed that.
Juan Barajas: Yeah. That's a catch.
Daniel Oblinger: It doesn't seem that risky other than I mean you are giving that machine all of your permissions on everything.
Juan Barajas: Then what do we do?
Daniel Oblinger: So if somebody hacked that machine they could reach back through that and…
Juan Barajas: People yeah, I think that's done.
Daniel Oblinger: basically get to anything you can get to. So I see in that way it's not recommended because you're not giving a specific permission.
Juan Barajas: Should I be on a specific Branch?
Juan Barajas: I think I'm on Main right now.
Daniel Oblinger: You're giving all of them,…
Daniel Oblinger: Got it.
00:10:00
Juan Barajas: Okay.
Daniel Oblinger: So let's go ahead and pull out to.
Daniel Oblinger: Okay, and…
Juan Barajas: Just the assets.
Daniel Oblinger: then let' we got to get the data repo in the right place in this.
Juan Barajas: Sure, but because many assets are actually checked into the repo and they're showing up as deleted and…
Daniel Oblinger: Yeah, we should go ahead and get into the feature.
Juan Barajas: so on.
Daniel Oblinger: feature branch
Daniel Oblinger: you actually have code out there.
Juan Barajas: Yeah on this I come back to doing whatever testing I'm doing and…
Daniel Oblinger: I see.
Juan Barajas: get stash pop and that's it.
Daniel Oblinger: I see So you'll stash that you'll never understand that you'll just keep on going and…
Juan Barajas: Yeah.
Daniel Oblinger: it'll just be in that GitHub. On that and…
Juan Barajas: Yeah, if you stop the instance,…
Daniel Oblinger: I get up forever.
Juan Barajas: yeah, if you terminated I think not.
Daniel Oblinger: They'll just be chunchy data hanging out there, right?
Juan Barajas: So we're on feeds a DVC that.
Daniel Oblinger: Right, but you don't even know what you were doing out there because it's like some Rando instance that you just spun up.
Juan Barajas: should I yeah.
Daniel Oblinger: When you freeze that instance, will it remain in the same state with the stash still there?
Daniel Oblinger: right got it kind of cool.
Juan Barajas: I think it's gone from your list of instances.
Daniel Oblinger: and when you launch an instance If it has not been stopped before it's going to create a new instance.
Juan Barajas: I don't know if it's gone or if you have to do an additional step to delete it,…
Daniel Oblinger: Can you launch an instance that basically?
Juan Barajas: but I think it's maybe gone
Daniel Oblinger: Won't start from the last stop one, but it'll start from the root one when you terminate the instance. does it live on in a initial form or when you terminated it's just gone from your list of instances.
Juan Barajas: You can do…
Daniel Oblinger: icing
Juan Barajas: what we're doing in production. And you have a machine at some State and…
Daniel Oblinger: Got it.
Juan Barajas: then we create an Ami and…
Daniel Oblinger: I see it's not a topic.
Juan Barajas: then I think you have the possibility to launch instances from an Ami.
Daniel Oblinger: Is it possible to Freeze dry and instance such that when you launch it every time it launches from the same place.
Juan Barajas: Yeah.
Daniel Oblinger: So you could launch 10 instances of this thing.
Juan Barajas: And I think I'll check out the branch, but I'm not sure if we have the external. So we have some stuff here.
Daniel Oblinger: Got it.
Daniel Oblinger: Got it. So it's basically to create the answer is yes, Dan. You create an Ami. That's the internet. Got it. Okay. Thanks.
Daniel Oblinger: Yes, I'm unclear now how you want to do this what we did before is and this is different than the data.
Juan Barajas: Mmm, did you update?
Daniel Oblinger: Repo which is what we were working on a minute ago. But what we did before is we had a sub module for DVC.
Juan Barajas: What happened here?
Daniel Oblinger: Dad and then BBC underbard he's a Sim link. I don't know if that's what it is. I don't know what it's supposed to be. but what are you going to do this and you want to get the data repo working first?
Juan Barajas: 
Daniel Oblinger: I think I updated and screwed it up. Because I was what I did in order to be able to run those are Sim links in my file system.
Juan Barajas: Okay. Okay. I think we never stopped using submodels for DVC that we stopped using some modules for the data repo.
Daniel Oblinger: Right in order to be able to run it. Locally, I simply to cross…
Juan Barajas: So you get more Euros? Yeah DVD that is still here.
Daniel Oblinger: which I don't think is the right thing to do. But remember we stopped doing sub module for that. So I didn't know what to do. So that's what I did and I didn't worry about the fact that it just got checked in.
Juan Barajas: Yeah. So, let me see. I think the first thing would be to just delete.
Juan Barajas: the existing PVC that
Daniel Oblinger: Okay, then…
Juan Barajas: yeah.
Daniel Oblinger: how should it be properly set up. Let's go ahead and do it here and we'll check it in as a fix.
00:15:00
Juan Barajas: remote machine if it messes up. Yeah.
Juan Barajas: Yeah. Living Pages module of the I think that should Update in it.
Daniel Oblinger: and you do an RF with a star in it do That's it.
Juan Barajas: Say to anything. No.
Daniel Oblinger: And s***. I don't mess with that s*** and…
Juan Barajas: That's good modules.
Daniel Oblinger: you did a force too. And you did a force man,…
Juan Barajas: Pretty sure this is yeah.
Daniel Oblinger: too. Seriously. Okay, living dangerous
Juan Barajas: it gets sold more use ink and then
Juan Barajas: some more you in its. Why is it not so?
Juan Barajas: It's Mario update in it.
Juan Barajas: external TVC that maybe No, Forgot the commands
Juan Barajas: I think she just be thinking update. But I'm not sure why.
Juan Barajas: It's not doing it.
Juan Barajas: Are okay stay?
Juan Barajas: So for some reason the DVC that is not in my config. some
Juan Barajas: but it was in it.
Juan Barajas: listening to help
Juan Barajas: He saw my good minutes.
Juan Barajas: it's
Juan Barajas: yeah, for some reason it only picks up the two existing ones
Juan Barajas: and that's external.
Juan Barajas: You know why this is happening.
Juan Barajas: of us that is That only cease two of them, but I'm not sure…
Daniel Oblinger: no, but I wonder…
Juan Barajas: why this could be.
Daniel Oblinger: if me wiping out because I wiped out DBC that right and then that got checked into the repo. So, I don't know if that would. cause it
Juan Barajas: and it's
Juan Barajas: okay modules.
Juan Barajas: So Mojo external BBC that yeah.
Juan Barajas: have URL
Juan Barajas: Yep, I don't know. what's going on.
Juan Barajas: Yeah.
Juan Barajas: Yeah, right.
Juan Barajas: So the weird thing is that if you check yet and then The salt only two sub modules are set up.
Daniel Oblinger: Can I see that foligan? I just want to make sure that the paths are correct for.
Juan Barajas: So module equation with triple and…
Juan Barajas: to pytorch But I think or…
Daniel Oblinger: GitHub Opening or…
Daniel Oblinger: dvc.get I think so. Yeah.
Juan Barajas: I thought that he did get so module in it.
Juan Barajas: that would
00:20:00
Daniel Oblinger: After get submodule in nits, we're not finding it config lists the submodules even though the sub modules are actually there in the configuration file for sub module. What would cause this to happen?
Daniel Oblinger: Okay modules are listed in a specific section of the configuration file that might not be included in the output of this by default. Here's what you can do to check. One take the configuration file directly. Instead of relying on getting big bash list directly open the dot Gitch config file of your repository. Look for a section named module. This section should list all the initialized sub modules along with their configurations to verify initialization ensure that after running get sub module in it. You also really get populate the sub module content. If the sub modules were initialized correctly you flash. Config three different configurations.
Daniel Oblinger: We can have multiple levels of configuration files specific system wide ensure you're looking at the correct level where the sub modules are configured by directly inspecting the dot dish config file.
Juan Barajas: I think we did to update. Yeah.
Daniel Oblinger: You should be able to confirm whether the sub modules are properly listed at your sub module. If they are present there, it is aware of them. Even if get config bashless doesn't show them in the default output this Behavior ensures that sub-module configurations are stored correctly within the repositories get configuration.
Juan Barajas: Yeah, I think we did. So some module update external I think happened.
Daniel Oblinger: So we didn't do that update we did the init but we didn't do update now to see if that doesn't anything.
Daniel Oblinger: After running get some modules also need to execute update.
Juan Barajas: He yes,…
Daniel Oblinger: I didn't follow all of the pieces it was.
Juan Barajas: I understand idea.
Juan Barajas: Okay,…
Daniel Oblinger: I didn't really follow…
Juan Barajas: what happens if we redo everything so…
Daniel Oblinger: where all these files are. So this is the thing that we've specified in get modules.
Juan Barajas: if you do get so module ad and then the repo for that…
Juan Barajas: which apparently is.
Daniel Oblinger: and then
Daniel Oblinger: this is used to derive get config.
Daniel Oblinger: Yeah.
Juan Barajas: I mean we could but
Juan Barajas: yeah.
Daniel Oblinger: The other thing we could do what if you just clone that whole freaking repo again and see what happens? You might just be in a weird state.
Daniel Oblinger: Actually, FYI what I was thinking about doing it probably be faster and more robust. If you don't mind, actually, I do add my own pem file to your instance that way because I know you're gonna keep your instance current and I never will. But what I can do is actually create another folder. called Dano and that way I never mess on your system that way if you're doing something to it, I'm not messing with it.
Juan Barajas: okay, let me try two things first and…
Daniel Oblinger: I'm just in my own subfolder on that same instance.
Juan Barajas: then I'll do that.
Daniel Oblinger: It's a problem if I used it very often because then we couldn't both run at the same time.
Juan Barajas: I'm just gonna try to remove it.
Daniel Oblinger: …
Juan Barajas: Yeah, and then get
Daniel Oblinger: I don't think that's a great. Long-term solution, but I just wonder if building my own instance is going to be the better way.
Juan Barajas: yes.
Daniel Oblinger: Either way you could create another clone right now in your folder structure and…
Juan Barajas: Hopefully that's it.
Daniel Oblinger: just see if it fixes it.
Juan Barajas: Are you? Okay, hold on.
Daniel Oblinger: What are you doing here?
Juan Barajas: I think it's thought.
Daniel Oblinger: You're gonna
Juan Barajas: Yeah modules. Just you see that I'm trying to run it again. There we go. Okay.
Daniel Oblinger: We had those things in there, We can't have that stuff in there, That's why I come on,…
Juan Barajas: an external PVC
Daniel Oblinger: right? Yeah.
00:25:00
Daniel Oblinger: Yeah, right. I bet that's it. Yeah.
Juan Barajas: then we had a Sim link, but I'm not gonna create the Sim link. I'm gonna do something else to see if it's possible to specify a
Juan Barajas: editable install from the environment.amo it's six Or…
Daniel Oblinger: Yeah.
Juan Barajas: what it's been. it's Don't have Konda mamba microconda what?
Juan Barajas: you don't have just install equipment.
Juan Barajas: I think it's just a script. but I use micromumba, so
Juan Barajas: I think it's just the curl for next my voice. Okay, I think this one.
Juan Barajas: Yep.
Daniel Oblinger: How do you install condom?
Juan Barajas: It's like a condo installation,…
Juan Barajas: but it's score is written in C++ instead of python. So it's considerably faster.
Daniel Oblinger: kind of curl
Juan Barajas: Okay.
Juan Barajas: Remember, there we go.
Juan Barajas: something like All let me actually specify it.
Daniel Oblinger: What is micro mamba?
Juan Barajas: the Environmental yamo So I wanted to know is if we could.
Daniel Oblinger: 
Juan Barajas: in specify here
Juan Barajas: to install
Juan Barajas: an editable local installation
Juan Barajas: segment anything, So it's ml that should be.
Juan Barajas: did you see that? Sorry, that's right.
Juan Barajas: 
Juan Barajas: what happens if we do?
Juan Barajas: external TVC that
Juan Barajas: he does that work.
Daniel Oblinger: It's dvcat.
Juan Barajas: The external DVC that let's see what happens.
Juan Barajas: Make sure probably put this here.
Juan Barajas: Okay.
Juan Barajas: I forgot the command.
Juan Barajas: and creates my Chromebook and create. Environmental, okay.
Juan Barajas: See if it works.
Juan Barajas: okay.
Juan Barajas: don't see it.
Juan Barajas: Yeah, we should.
Juan Barajas: let's see these ones.
Daniel Oblinger: scroll up and see if you see we see DVC in there.
Juan Barajas: No.
Daniel Oblinger: once again
Juan Barajas: Yeah.
Juan Barajas: It's just gonna take a little bit.
Daniel Oblinger: We had an error though.
Juan Barajas: yeah, in the meantime, I guess I can just open another SSH session and do the other stuff.
Juan Barajas: so what do I do I get clone the
Daniel Oblinger: We should be able to go into a python though and see if it's there, right?
Juan Barajas: generate long
00:30:00
Daniel Oblinger: it's still.
Juan Barajas: Okay.
Juan Barajas: let me find the
Daniel Oblinger: All right. data repo
Juan Barajas: I don't know either. Let's see what happens.
Daniel Oblinger: which should be under on our good bucket And I think we're just cloning that into slash data Inside Out 2.
Juan Barajas: on repost data for this
Daniel Oblinger: I don't know how that's gonna be.
Daniel Oblinger: configured
Daniel Oblinger: To migrate away from assets. What do you think about the idea that we would? Have our system lazy load a thing called, standard assets or something like that, which is a dad. And we just fill it in with I like trying to get rid of the assets folder instead.
Juan Barajas: I think it's at least the one that I think everyone uses or the one that I use and I've seen Greg updated is
Daniel Oblinger: We actually reference the specific asset that we're working with…
Juan Barajas: It's in S3.
Daniel Oblinger: but Going through all of our code and…
Juan Barajas: Let me check.
Daniel Oblinger: changing all that stuff is just going to be a little painful. Do we have a standard assets folder that is like a gold standard for the team are deserably kind of just build their own assets folder.
Juan Barajas: it's just a folder in S3 that contains all the assets and see if I can find it.
Juan Barajas: In annotations I'll put yep here is so it's the same one that is in the readme. So if you check the readme file. the first
Daniel Oblinger: so There's a zip file.
Juan Barajas: yeah, it's gonna this one.
Daniel Oblinger: That's got the assets folder.
Juan Barajas: Let's see how many assets.
Daniel Oblinger: Got it.
Juan Barajas: mentioned in the code quite a few.
Daniel Oblinger: I think yeah.
Daniel Oblinger: Yeah, my thought is that what we do is we just decide to have our system.
Juan Barajas: but How that work.
Daniel Oblinger: If you do not set an asset variable, then we'll set it for you and…
Juan Barajas: I'm not. following
Daniel Oblinger: we'll set it to a lazy loaded asset that becomes kind of our standard asset and then people can. not use their own assets folder
Daniel Oblinger: My idea is we would say. Okay the new standard. For the asset has moved from this S3 bucket that they did there into adapt bucket, And so the new standard asset you would use a DAT load to get that asset folder. But then in addition to that we would because remember nobody's automatically getting that S3 bucket. we delete that S3 bucket so that people don't do that. And we instead have a debt folder with Assets in it.
Juan Barajas: mmm
Daniel Oblinger: We copy that into a DAT and that becomes the standard asset object that the Greg can keep updating and then in the code, we can actually just say if the user didn't set an asset variable. I am going to Lazy load assets and I'm going to set the asset variable to be that path into that lazy-loaded thing that way in your local code people just stop using asset but if they wanted to they could create an asset folder and they could copy stuff in there and some people may want to still manually, manage their own assets, but
Daniel Oblinger: In production, we would just not do anything like that in production. It would just do a lazy load of adapt that we give you the assets and then over time what we do is we say quit using the asset folder in your code when you refect code, let's put the assets into debts separate debts that we maintain right if it's in it if your code is gonna reference an asset and…
Juan Barajas: Yeah.
Daniel Oblinger: use that to do it, but that can happen over time and at some point we might scrub, assets out of the system, but that would be in the future That way, we wouldn't. for historical reasons we might even just leave An asset out there…
00:35:00
Juan Barajas: Okay.
Daniel Oblinger: because they're probably gonna be deprecated code that will still need an asset folder.
Juan Barajas: And let's see if that works.
Daniel Oblinger: And if you just rip it out, it will no longer run…
Juan Barajas: You see us,…
Daniel Oblinger: which would be unfortunate.
Juan Barajas: later.
Daniel Oblinger: But if you said it so that basically it auto lazy loads the asset thing…
Juan Barajas: 's I thought important. I know that nope.
Daniel Oblinger: then if you ever call the duplicated code it'll lazy load the asset and it'll set the asset variable and it'll work right.
Juan Barajas: Let's see if it's even possible to that. Iron Man, yeah.
Daniel Oblinger: cool
Juan Barajas: You should do that combine vehicles.
Juan Barajas: 
Daniel Oblinger: In the environment.yamophile, is it possible to do a editable hip install?
Daniel Oblinger: The animal file you can specify an editable pip install. This is done by using the PIP section and specifying the package name followed by my C. This tells pip to install the package and editable mode from the current directors. Can you show me an example inside an environment.yamo file so I know exactly the format I should use.
Juan Barajas: What's it say?
Daniel Oblinger: certainly It was an example of how you can specify and editable pip install inside an l file. You can see the code in our conversation history in this example.
Juan Barajas: Yeah, let's see.
Daniel Oblinger: My package name is the name of the package you want to install and editable mode. e
Daniel Oblinger: After it is a in square brackets.
Juan Barajas: Yeah, if it gets this right, I think it's pretty impressive
Daniel Oblinger: It's a dash e period it's weird. just Let me send this to…
Juan Barajas: package square brackets
Daniel Oblinger: You would never got this, right? Yeah.
Juan Barajas: It's that correct.
Daniel Oblinger: It is surprising how well you can ask very pointed questions.
Juan Barajas: let's say yeah. dependency
Daniel Oblinger: Exactly. Look at that thing, right?
Daniel Oblinger: And I don't know it's showing here. I mean, it say it's got here name and…
Juan Barajas: I mean we can and we can try it. Let's see.
Daniel Oblinger: then Channel and…
Daniel Oblinger: then dependencies and then pip is underneath that is that the way you have it is your structure what's above pip here? Is it dependencies?
Juan Barajas: Dash E Period
Juan Barajas: this doesn't sound right, but let's see.
Daniel Oblinger: Yeah, okay. I don't know it's a weird-ass format. But that's my package name Dash e and then it's a period almost like it's Yeah,…
Juan Barajas: Yeah. Let's give up the fire path.
Daniel Oblinger: I don't know.
Juan Barajas: I think it's a local package.
Daniel Oblinger: Okay.
Daniel Oblinger: and also you're giving a file path here and it was giving the name of the package. Let me ask it about that.
Juan Barajas: in the official Documentation for Konda they have an example of an editable local package and…
Daniel Oblinger: Can you show me that example, but do it with a local package with a relative path name?
Juan Barajas: I think it's exactly the same as we had it before. It's like this one.
00:40:00
Juan Barajas: so it's like editable Dot
Daniel Oblinger: When you're using a local package with a relative path name in your project, you typically refer to it using dot location, right?
Juan Barajas: external
Juan Barajas: / Let's see if I get it, right.
Juan Barajas: external TVC that
Juan Barajas: okay.
Daniel Oblinger: 
Juan Barajas: You see that? Okay.
Daniel Oblinger: Please provide an example an environment.yamal that has a local. File that's installed in an environment. Here's how you would specify it. You can see the code in our conversation history in this example replace path to your local package with the relative path to your local package.
Daniel Oblinger: Really?
Daniel Oblinger: Please give me an environment.yamal example that has a local package that is installed with an editable fashion rather than a normal install. Give me the exact format for that in a example yaml file.
Juan Barajas: 
Daniel Oblinger: Who's an example of an environmental file that includes a local package?
Juan Barajas: How should I change this?
Daniel Oblinger: They've got Pip gash Dash e and then the path to the local file.
Juan Barajas: Also, let me try to install.
Daniel Oblinger: so there's just like we did it,…
Daniel Oblinger: but it's a dash and then a dash e
Juan Barajas: A mini conda,…
Juan Barajas: maybe micro mamatos and support it.
Daniel Oblinger: H let me just show you what you put a dash in front, but it's unclear to me. let me just send this to you see exactly what
Juan Barajas: Where is said - it's a pip? Does she but I think that Dash is just a list of the element
Juan Barajas: Yeah.
Juan Barajas: I think we have it, Put it. Maybe just that micro. Mamma doesn't support that thing. Let's see.
Daniel Oblinger: you can see the format that they did there.
Juan Barajas: Me come.
Daniel Oblinger: I see.
Juan Barajas: Anyone last three?
Juan Barajas: and it Ash what?
Daniel Oblinger: that doing
Daniel Oblinger: I see.
Daniel Oblinger: My goal is to I mean, I'm okay doing that. but really what I want to do is get to the point where we can tell everybody do this and it'll work for you. So if we can get it to work now, it's probably better. But I wonder why.
00:45:00
Juan Barajas: instructions
Daniel Oblinger: Many time does not installing that seem like that should be. How would you install many conda into Ubuntu?
Daniel Oblinger: When Stone any kind of unable to follow these steps one download the mini content installer. You can see the code in our conversation history thought too verify the installer optional but recommended you can see the code of our conversation history check the output against the hash provided on the many kind of website to ensure the Integrity of the download three. Run the installer. You can see the code in our conversation history got four follow the props to complete the installation except the license.
Daniel Oblinger: Yeah, so there are W getting from Anaconda. Probably what shooting as well.
Juan Barajas: Yeah.
Daniel Oblinger: And there's sourcing activate.
Juan Barajas: Come back. There we go. No, it is those work I think it's just that the command is conda directly even…
Daniel Oblinger: What was wrong?
Juan Barajas: if you install micro or miniconda. Yeah.
Daniel Oblinger: I see, right.
Juan Barajas: Okay, let's try it again. So it looks like this is in the right format. Let's try and do Honda exactly this.
Juan Barajas: thing so kondai and we create f barometer yamo
Daniel Oblinger: I wonder if that square bracket thing is somehow an alternative format seems very weird that would invent something like that.
Juan Barajas: Yeah. that sounds to me more specifying a specific version of the same thing we did with DVC is three or…
Daniel Oblinger: right right
Juan Barajas: eight WS
Juan Barajas: yeah. Yeah.
Daniel Oblinger: right, right. Yeah, I agree with you.
Juan Barajas: Just give me a second. I'll be right back.
Daniel Oblinger: sir
Daniel Oblinger: Alexa set alarm for 6 minutes
Juan Barajas: back
00:50:00
Daniel Oblinger: I'm still here by the look. don't know…
Juan Barajas: Okay.
Daniel Oblinger: if you were I can't see my screen. I am preparing some food.
Juan Barajas: okay.
Daniel Oblinger: I get these. prepared meals. I think I told you that.
Juan Barajas: yeah, yeah. so it's just like we reheating it or
Daniel Oblinger: Yeah, you put in a skillet for about to put it in for three five minutes. I put it in for seven minutes, but I put it on a very low heat.
Juan Barajas: right
Daniel Oblinger: I don't have to attend to it.
Daniel Oblinger: did condoload
Juan Barajas: It's working on it. It's a little bit slow.
Daniel Oblinger: much Yeah, and at the same time we can do the data folder.
Juan Barajas: Yep.
Juan Barajas: So I've already cloned data.
Juan Barajas: Okay. Yep.
Juan Barajas: Okay, it finished. Let's see.
Juan Barajas: Okay conductivity and
Juan Barajas: it worked. Okay. Nice.
Daniel Oblinger: 
Juan Barajas: I'm still external DVC that now let's test that it is indeed and editable install. exit and I'm gonna modify some file.
Juan Barajas: external and now that's
Juan Barajas: of PVC that I always forget movies he got and then
Juan Barajas: the world
Juan Barajas: Yep, it works. Nice.
Daniel Oblinger: so now everybody else can just do get clone and Boom, it's done. as long as they've got they're using mini condo or condo and not using some sketchy C++ variance and…
Juan Barajas: Yeah.
Daniel Oblinger: Alexa stop so you've got your local data repo
Daniel Oblinger: I guess we can Just do that. if it's in your current environment, you ought to just be able to take that list.
Juan Barajas: 
Juan Barajas: It's very good.
Juan Barajas: that list Yep. or base names matching anything
Juan Barajas: yeah. I think that worked.
Daniel Oblinger: What's the thing?
Juan Barajas: It's saying base names matching star and then add list pull that Tools in DT.
Daniel Oblinger: Yeah, so it interesting even found. It worked. I didn't think that would work.
00:55:00
Juan Barajas: Okay, why not?
Daniel Oblinger: Because I didn't think this external so it is using the fact that DVC is but you did install it as a sub module. So it really will be there.
Juan Barajas: Yep.
Daniel Oblinger: So it's utilizing. into the DVC that module in two ways, right? It's skip installed the DVC underscore right? But it's also using the fact that the whole repo is at a known location inside of external.
Juan Barajas: Yep.
Daniel Oblinger: So, let me see if I can. make it work properly for me now because I have
Daniel Oblinger: Let me just share my screen real quick and…
Juan Barajas: No.
Daniel Oblinger: make sure share now entire screen.
Daniel Oblinger: And it should just work if I just let me bring. This up.
Daniel Oblinger: So I'm in that's DVC debt. I'll go over here to data alcohol external
Daniel Oblinger: and then RM Okay. Are that Evie see that two and DVC underscore debt
Daniel Oblinger: and so then I should just get submodule.
Daniel Oblinger: in it
Juan Barajas: I think it's gonna be easier if I just push the sub module changes.
Daniel Oblinger: I see. Okay. Got it. Yeah.
Juan Barajas: Yeah, let me do that first.
Juan Barajas: Let's turn all these that's
Juan Barajas: pitched
Juan Barajas: You know what attractant?
Juan Barajas: no, I think that's fine. So good. Come here. it's to fix
Juan Barajas: and some module
Juan Barajas: If you do push now, it should work.
Daniel Oblinger: pull you mean
Juan Barajas: Or pull sorry. Yeah.
Daniel Oblinger: And then I should do SUB module in it.
Juan Barajas: Yep, or you can just do update dash in it.
Daniel Oblinger: what is in it do?
Juan Barajas: It's the same as if you were to do SUB module in it and then sub module update. It's just a single command.
Daniel Oblinger: so let me do that.
Juan Barajas: Okay.
Daniel Oblinger: to do anything
Juan Barajas: So it's external. I think in it by itself doesn't do much.
Daniel Oblinger: No.
Juan Barajas: If you cut your dog.
Daniel Oblinger: and then I do it now. I need to do some module update.
Juan Barajas: Yeah.
Juan Barajas: I think so at least do you see that checked out?
Daniel Oblinger: Yeah. and…
Juan Barajas: Yeah.
Daniel Oblinger: We don't have a subfolder anymore that. Points to it. So now we just have DVC dot there, but
01:00:00
Daniel Oblinger: How do I get into the Honda environment that yaml I need to create that environment, right?
Juan Barajas: Just run this command.
Juan Barajas: this one let me just remove the temp just environmental demo. Okay.
Daniel Oblinger: I don't think you're showing your screen anymore.
Juan Barajas: I just sent the command by the meetings chat. Yeah.
Daniel Oblinger: I got a
Daniel Oblinger: condo create What's it what?
Juan Barajas: So I think it's on the root spot the project fruit. So now algorithms, too. that you should yeah.
Daniel Oblinger: I see, okay. Okay.
Daniel Oblinger: What's It Gonna create though
Juan Barajas: It's gonna create a combine environment with all the dependencies for the project. It's gonna be SV underscore Analytics.
Daniel Oblinger: What's the name of it gonna be?
Daniel Oblinger: Okay, it looks like it didn't work. I don't have high tour.
Juan Barajas: because I haven't pushed.
Daniel Oblinger: I can't do all that. No, I can't actually build this thing.
Juan Barajas: I haven't pushed those.
Juan Barajas: All Or not available. Okay. Yeah.
Juan Barajas: then I guess you're gonna have to install it by yourself and just to pip install that she and then the path.
Daniel Oblinger: You right and I really done that. already
Daniel Oblinger: if you recall. wait
Juan Barajas: Check the weight.
Daniel Oblinger: on the so it's already in there now and…
Juan Barajas: Yeah.
Daniel Oblinger: 
Daniel Oblinger: that's not good.
Juan Barajas: Think it's gonna be better if we just uninstall it and then reinstall it.
Daniel Oblinger: I see it looks like you got Json on there twice. I don't know why. Okay, ling reinstalled. So then I would do pip uninstall or does that work?
Juan Barajas: yeah, they're uninstall and then TVC underscore Yeah.
Daniel Oblinger: Okay.
Juan Barajas: Then you would do if Dash e then
Daniel Oblinger: And now I'm in the root so I would say external. slash EVC Yeah like that. right
Juan Barajas: Either that or yeah, okay.
Juan Barajas: cheating file
Daniel Oblinger: So you did it, right? so,…
Juan Barajas: yeah. Okay. Yeah.
Daniel Oblinger: let me see now and It's weird that it's somehow seems like it's loading notice.
Juan Barajas: Okay.
Daniel Oblinger: It didn't debt. Config Jason.
Juan Barajas: Yeah.
Daniel Oblinger: if you type down on your system. does it
Daniel Oblinger: does it work? It does right.
Juan Barajas: Just that. Let's see. Yeah. It does work.
Daniel Oblinger: Yeah. Yeah, that - dad space dash info. Do you get a nice info?
Juan Barajas: That just info. Yep.
Daniel Oblinger: Yeah, I have no idea. Maybe I've made changes.
Daniel Oblinger: I don't have anything modified it that can think Json I bet I bored my dad can say Jason.
01:05:00
Daniel Oblinger: Yeah. I don't know why there's an extra character there.
Juan Barajas: Yeah.
Daniel Oblinger: That'll do it.
Daniel Oblinger: There we are though. I'm gonna fix that though because that's a weird error message clearly something's wonky with my error…
Juan Barajas: Yeah.
Daniel Oblinger: So I'll make an error again so I can fix that. So now for other folks. They would be able to. clone the day data repo and then clone the data repo and then
Daniel Oblinger: really just do conduct buildaconda environment and they're Off to the Races. They should just be able to do that info to see that it's working. It should just work. So my thought and we don't do this now, although we can do it now is I want to create an environment for Briones. And maybe what I'll do is I'll just use his environment. So in other words, we'll just create an environment. Maybe we'll be sloppy with this environment. Let's think about how we want to do this. I want to make it so that Briones and potentially other people can actually get to this environment. Once we've got more of a production stance. I'm gonna have to tighten this down because the problem is Once you can get to this environment, you can actually push.
Daniel Oblinger: Stuff into our data repo right which is a messy right? You actually can probably push stuff into our ALG to repo now Brion is not going to do that. I don't worry about that. he's a trusted Insider as much as our team is but I think I let me just verify Dan here.
Daniel Oblinger: what company do you work for?
Daniel Oblinger: Got it's terrible. I normally have my phone set to just go to voicemail but I have a doctor call for my doggy and so terrible. I have no idea I don't know if I was answering my phone. Maybe these guys would give up after a while since I'm not answering. They just are all there but they turn this on I'm getting deal used by one.
Juan Barajas: Yeah.
Daniel Oblinger: It's ridiculous. Anyway,…
Juan Barajas: it's dog. Okay.
Daniel Oblinger: so how will we share?
Daniel Oblinger: How do we share the access to this thing with Briones?
Juan Barajas: to this instance
Daniel Oblinger: I turned off my headset. Hang on.
Daniel Oblinger: check Yeah,…
Juan Barajas: check. Yep.
Daniel Oblinger: yeah. Yeah, I mean what I give him a pen file or the private one, how would we concretely give him access to this?
Juan Barajas: I wouldn't like to give him access to this one because this is this mine.
Daniel Oblinger: No, not just one. No, I meant no, we're gonna build one. But how would give them access to it?
Juan Barajas: Yeah. Just we could just share the pen file, I believe.
Daniel Oblinger: Mm-hmm
Juan Barajas: And he should be able to just SSH into it.
Daniel Oblinger: What's a good way? Would we put the In a folder on the Google Drive, I mean kind of gross, right?
Juan Barajas: it is but I think that's what sortie did pretty much just uploaded a bunch of credential files to drive. Yeah.
Daniel Oblinger: is there any better we can do or
Juan Barajas: I think there's some major US magic that you can do such that you set up your user and then you can only have access to the instance by the permissions of your user or something.
Daniel Oblinger: mmm, but
Daniel Oblinger: can we use SSH with just user pass and know hem file? is that
Juan Barajas: I think you can yeah.
01:10:00
Daniel Oblinger: I don't.
Juan Barajas: 
Daniel Oblinger: I don't know if that's any better.
Juan Barajas: Yeah, I don't know if that's better. Yeah, I think we can but as you say I don't know if that's any better.
Daniel Oblinger: For now, we don't have a better solution than Putting the permissions in there. Maybe we can lock that folder down a little bit. on so that on
Daniel Oblinger: So that we at least have explicit control over who in Google Drive. Can Remove people in other words if this is enough that this is in a folder or others have permissions to it in Google Drive. Can we actually remove permissions?
Juan Barajas: mmm
Daniel Oblinger: I'm not sure that you can
Juan Barajas: I think you can. but only yeah,…
Daniel Oblinger: let me try.
Juan Barajas: yeah we can try but I think you can
Daniel Oblinger: Okay. Ask me.
Daniel Oblinger: data
Daniel Oblinger: I guess I'll just put it inside of the SD data. new folder
Daniel Oblinger: credentials
Daniel Oblinger: And then let's see here share.
Daniel Oblinger: Okay.
Daniel Oblinger: We'll just see if I can remove somebody I'll remove.
Juan Barajas: You could maybe remove me and then I'm gonna try to enter.
Daniel Oblinger: I'll just remove the shawl remove from item
Daniel Oblinger: a general access
Daniel Oblinger: I don't remove.
Daniel Oblinger: Maybe I just put Sports Physio. Wait, what?
Daniel Oblinger: When I just do like I remove button.
Daniel Oblinger: I don't want.
Juan Barajas: Just Happening changes
Daniel Oblinger: I don't want to.
Daniel Oblinger: I don't want to hit the save button. Discard, okay, so let's do it again.
Daniel Oblinger: share share
Daniel Oblinger: the show
Daniel Oblinger: I'll remove access there we move from item. Okay, and now Sports Physio. Why can't I remove access?
Juan Barajas: anyone
Daniel Oblinger: Maybe I can say restricted.
Juan Barajas: maybe restricted out.
Daniel Oblinger: remove item access
Daniel Oblinger: Only people with access, And then I can add.
Juan Barajas: If you can sure.
Daniel Oblinger: With Realms, go ahead.
Juan Barajas: you could share the link with me and see if I can enter and see the credentials given that I'm not in the explicitly granted access list, see if it works.
Daniel Oblinger: Yeah, that's right. You're not there. Okay done.
Daniel Oblinger: share link
Daniel Oblinger: We are.
Daniel Oblinger: I'm gonna get rid of this freaking Hotmail.
Daniel Oblinger: can I edit here share with these people?
01:15:00
Juan Barajas: Which one?
Daniel Oblinger: don't care.
Daniel Oblinger: So I'm not adding any so if I click don't give access I'm not adding any access. but
Juan Barajas: I guess.
Daniel Oblinger: unclear
Juan Barajas: exists denied
Daniel Oblinger: Okay let me change share and now I'm going to say one.
Daniel Oblinger: He's a good guy. We'll share with him.
Daniel Oblinger: James Brown's also a good guy. Okay.
Juan Barajas: And credentials. Yep, it works.
Daniel Oblinger: Great. So, how about we create an instance that's got
Daniel Oblinger: It's got GPU on it. Let's just go ahead we'll probably do another instance later.
Juan Barajas: Yep.
Daniel Oblinger: That doesn't but just for now. I want them to be able to do whatever and also for me to do whatever.
Juan Barajas: Let's do that.
Daniel Oblinger: I want to unshare.
Juan Barajas: You share my screen. Let me shut down the instance.
Juan Barajas: Makes this one.
Juan Barajas: Yep.
Juan Barajas: that winstons
Juan Barajas: and let's create another one.
Daniel Oblinger: I see you did a midday lunch today. So was that what the girlfriend?
Juan Barajas: That was a breakfast quote unquote Bunch.
Daniel Oblinger: 
Juan Barajas: Yeah. by the way yesterday when I said dinner, I meant more of lunch because I kind of kept thinking about why would I it's dinner like Two o'clock or one o'clock. Yeah. It's like
Daniel Oblinger: Yeah, I was surprising you were calling that dinner like what you can call whatever you want.
Juan Barajas: yeah. Yeah.
Daniel Oblinger: I guess, your mouth, but I want to call one o'clock thing and dinner, Was that a translation bug or what was that? Why were…
Juan Barajas: That was a translation ball. Yeah.
Daniel Oblinger: You were so there's a word and it's named differently in Spanish.
Juan Barajas: Why it's kind of the same thing. I just get confused over, breakfast lunch and dinner. We have this and Sena basically the same thing. It's just I know that Americans I think typically dinner a little bit earlier than us.
Daniel Oblinger: 
Juan Barajas: So your dinner is in court quite our dinner, but kind of same thing equivalent. Yeah.
Daniel Oblinger: but not that I see so It wasn't a word thing. You just were confused about what to call it for Americans. I think I'm actually paying attention.
Juan Barajas: Yeah a little bit of both, I think.
Juan Barajas: Yeah. Let's see large we're gonna launch an instance.
Juan Barajas: just gonna put this
Daniel Oblinger: but Are you creating a new? I'm confused.
Juan Barajas: So when you launch an instance, I think it creates one by default. And what you can do is just select an existing one and launch it but you would have to click it and then launch instance.
Daniel Oblinger: I see…
Juan Barajas: Or start instance,…
Daniel Oblinger: but I see…
Juan Barajas: I think.
Daniel Oblinger: but instead of doing that won't over there if you click launch instance, it's gonna launch. a Posix instance. What's it launching?
Juan Barajas: You click it and then this opens up so for you to configure your instance,…
Daniel Oblinger: …
Juan Barajas: so that's kind of from scratch thing.
Daniel Oblinger: I see them.
Juan Barajas: And then you could also just click this arrow and then you can launch from templates and then I think you can launch from an Ami.
Daniel Oblinger: Got it, okay. but you're gonna launch from how she Corp something.
Juan Barajas: a fresh instance I mean, I don't know template my Amazon template whatever and I think you can choose the template so First name? How should we call this?
Daniel Oblinger: This is gonna be it's not for pre-owned specifically. It's a computer vision.
Daniel Oblinger: Admin, but it's not admin. it's Ops. how about CV Ops?
01:20:00
Juan Barajas: now that I remember I think James has his own instance this pre-owned pipeline execution. So I think this is from working with Greg and whatnot. He already has this. Yeah.
Daniel Oblinger: maybe it's already good enough to use you want to just try starting it and see if we can got it. Of course.
Juan Barajas: We could it's just I don't know if this one has specific set of credentials. Let's see.
Juan Barajas: security
Juan Barajas: Yeah, generally AWS key. I think this is let's see.
Juan Barajas: Generally just key.
Juan Barajas: so it has a keeper called General AWS key, but I'm not sure if that's one that I already have or it's a new one. Let's see.
Juan Barajas: Let's start We own Spider-Man execution.
Juan Barajas: It's running.
Juan Barajas: game, let's say
Daniel Oblinger: There's an error over there on the right if you notice.
Juan Barajas: The others and error, it's just the credentials are probably not right. Projects. What's this yellow?
Juan Barajas: So I have a couple of them we can try different.
Juan Barajas: Make sure my terminal.
Juan Barajas: so I'm gonna try this one a master prod. I'm going to I think this is the correct a beam.
Juan Barajas: That's not it.
Juan Barajas: That's true property. I'm guessing And that's not it either. No. I guess James has that keeper so maybe we should ask him to provide us with that keeper. And we should use the general one.
Daniel Oblinger: 
Daniel Oblinger: It's weird though. Why would he have a key pair called AWS General?
Juan Barajas: I don't know I think. Probably either Martin or Greg gave him though that keeper.
Daniel Oblinger: But it sounds like they keep pairs used. more generally for us
Juan Barajas: It I'm not sure where it's used or why I don't have it, but I don't have it. It looks like
Daniel Oblinger: okay, and Big picture longer term. Do you think the better thing here is that we have a separate instance.
Daniel Oblinger: for cdops or should we really just use this James one and probably less maintenance for us to just have that one that we're sort of maintaining.
Juan Barajas: yeah, I think we can reuse James instance.
Daniel Oblinger: The only thing that would go wrong is if I were using it because I don't think I can't run our system. Except to run it on ec2, which I've not done yet. And if I were running it, there's no way for me to run it if James is already running on it. So probably the thing that we should do and…
Juan Barajas: Yeah.
Daniel Oblinger: we don't necessarily need to do this today, but we probably should do it soon as we probably should have an Ami instance. that we have access to in fact, we might depressate Briones in favor of an Ami instance that Briones could spin up. because the thing is I really want to get to the place where we can run in parallel a bunch of compute jobs. And so
01:25:00
Daniel Oblinger: in order to do that right now. we don't have what we really want is an Ami instance that we can basically instantiate and then it's already ready to go. It's got DVC Dot and all that stuff in there. And so then you would use that a little bit like our production pipe. Maybe it could even be our production pipe and that we actually have a special mode on our production pipe where we basically say, Interactive. and then it's gonna run Interactive
Juan Barajas: Yeah, we could just reuse the Ami that is being used in production. Because in theory that should have everything like us updated as production uses. So
Daniel Oblinger: right right, so
Daniel Oblinger: But how could we use the one for production?
Juan Barajas: same not sure what software is doing at the moment to launch AI workers, but Let's see if we can do one.
Daniel Oblinger: But even if we launch it. I guess we can just SSH to it doesn't matter.
Juan Barajas: Yep.
Juan Barajas: same launch from template
Juan Barajas: No templates found a bunch of principle.
Daniel Oblinger: where
Juan Barajas: God maybe this is not it. Maybe they might Think the latest one is stock price.
Juan Barajas: this one for example I'm using a Dev suffix for an instance that doesn't have an entry point. So you can just SSH into it and it's not going to look for a job to execute.
Juan Barajas: Launch instance from one.
Daniel Oblinger: So Dev doesn't mean Dev branch.
Juan Barajas: Yeah, I know. It's just being so no entry point, basically.
Daniel Oblinger: Probably use a different word for that. What I'm thinking we're going to be doing is we're going to be keeping track of Amis for production for stage and…
Juan Barajas: Yeah.
Daniel Oblinger: for Dev.
Juan Barajas: we are already doing that with the parameter store. Because all the environments can point to a single image and we don't have to create three images for that.
Daniel Oblinger: right, but if we want to have another words three images But we might not put the suffix on them. We might just have three images and they are referenced.
Daniel Oblinger: Yeah, we may not have the word stage on them. It may just be that you keep track of this images now in stage. It is the definition of stage.
Juan Barajas: Yeah.
Daniel Oblinger: I got a
Juan Barajas: But yeah, I think we could probably just change this. For Dev meaning yeah.
Daniel Oblinger: there's something else. Yeah.
Juan Barajas: It's using the latest changes or something and then we've removed the depth and it's kind of the official version the 1.11. But yeah, I think we should. Be able to just do launch instance from Ami.
Juan Barajas: then maybe instance change the instance type
Juan Barajas: although we should probably ask software how they do it and then We do the same thing or reuse that same mechanism or something.
Juan Barajas: But yeah.
Juan Barajas: Chief five
Daniel Oblinger: will
Juan Barajas: Yeah, maybe launch instance and that's gonna work. I'm sure.
01:30:00
Juan Barajas: Yeah, I think that's how we would do it.
Daniel Oblinger: what can we do it?
Juan Barajas: He can try.
Juan Barajas: Okay, it asks you for a set of keys. Let's do.
Daniel Oblinger: Should we create a new pair and we'll put that in the credential file? Are the credential folder?
Juan Barajas: we could Create Bearer. CV
Daniel Oblinger: Let's do that.
Daniel Oblinger: Yeah, we call it cvops or what do you think?
Juan Barajas: CV UPS General
Daniel Oblinger: our CV Ops 1
Juan Barajas: Are we gonna have? CV Ops 2
Daniel Oblinger: I don't know. I'm okay going at CVS General. Yep.
Juan Barajas: Okay. I say them.
Juan Barajas: and launch instance again
Juan Barajas: Great.
Juan Barajas: Let me upload. the kief the credentials with data
Juan Barajas: credentials this one
Juan Barajas: successfully launched Yep, the instance. That's the AP.
Juan Barajas: It's already uploaded and let me share my terminal.
Daniel Oblinger: And go ahead and copy it into that folder. I'll try from my side to see if I can at the same time SSH into it.
Juan Barajas: It should already be there.
Daniel Oblinger: is it?
Juan Barajas: it should if it's not just let me know.
Daniel Oblinger: Got it.
Juan Barajas: Listen, it's yeah hops.
Juan Barajas: sit code CVS General
Daniel Oblinger: And I should put that dot pen file into my DOT SSH folder.
Juan Barajas: You can put it anywhere. I think. It's just as long As you provide the path with - then you can have it anywhere.
Daniel Oblinger: I see but the normal place to put it would be in your dot SSH wouldn't it?
Juan Barajas: I have it on this path. I'm not sure if the standard ways to put it on SSH because I don't think it's gonna automatically pick it up. You still have to provide the path hear that we're just configure your SSH config to point towards that path. But I'm not sure what the standard way to put the credentials is.
Daniel Oblinger: I see. you have a DOT creds, but then that doesn't get checked into your repo you get ignore that.
Juan Barajas: That's outside a new repos kind of. I have Yeah,…
Daniel Oblinger: Sports, Physio is not a Got it. Okay, I'll do the same thing.
Juan Barajas: that's not Rico. so it just have Yep.
Daniel Oblinger: You just did why not?
Juan Barajas: And we should be able to Sage into it.
Juan Barajas: yep. There we go. It's configured with GPU. It has the algorithms. Repo and everything needed to run the pipeline, I think.
Daniel Oblinger: see if you can get pull and get the latest that and see if it actually works.
Juan Barajas: 
Daniel Oblinger: Right, or is that not?
Juan Barajas: I could do anything.
Juan Barajas: Just gonna have to forward my SSH and everything.
Juan Barajas: all around okay pull
01:35:00
Daniel Oblinger: can you? Pause for a second. I move that pen file. show me how to log in using that So I think I've got in the right spot. So I would just did I did - a right.
Juan Barajas: - A if you want to forward your agent and then Dash I for the identity file.
Daniel Oblinger: And then there I'm in the kred folder. So I would basically just say What's it called there? That's what I do and then I'm gonna say.
Daniel Oblinger: What address am I going to? wait do I need to have you you've actually added stuff on your system into a folder that specifies these different targets?
Juan Barajas: it's
Juan Barajas: yeah, but that's not exactly needed. Plus you need to The address every time because I think it's going to change. So the best thing is just to check it.
Daniel Oblinger: Right, so just get the address. yep, just
Daniel Oblinger: and in our case, what's the address?
Juan Barajas: In our case, let me share that. Checks this one.
Daniel Oblinger: Got it. And so it's just going to be what user do I say at that?
Juan Barajas: It's Ubuntu at and then that one.
Daniel Oblinger: like that
Daniel Oblinger: 
Daniel Oblinger: the authenticity of the host
Juan Barajas: yeah, because On a new connection, I think.
Daniel Oblinger: I've seen this before. I just can't I remember what I had to do to fix it.
Juan Barajas: You can just do yes, and then that's it. Yeah.
Daniel Oblinger: I see. Okay. Got it.
Daniel Oblinger: Okay, so I'm in I wait.
Juan Barajas: yeah, I think you still need to modify the permissions for
Daniel Oblinger: a traumatic Yeah.
Daniel Oblinger: it's supposed to be for five. zero zero
Juan Barajas: You can do 400.
Juan Barajas: Also, I think 600 should work. But 400 is. I think better that's read only.
Daniel Oblinger: Yeah, And so then let's do it again. It should say okay this time. We're cool And so…
Juan Barajas: Yep.
Daniel Oblinger: then is right there, so.
Daniel Oblinger: What's now?
Juan Barajas: just package manager for Ubuntu
Daniel Oblinger: could not read from remote Repository. wonder why
Juan Barajas: 
Daniel Oblinger: I forwarded my permissions.
Juan Barajas: Let's get Euro origin. you should have access I think.
Juan Barajas: Yeah.
Juan Barajas: Can you run this command?
Juan Barajas: this one see if it shows up anything.
Daniel Oblinger: But the question is do I have permissions?
Juan Barajas: Yeah.
Daniel Oblinger: Let me just verify on my own machine I should.
Juan Barajas: Yeah your mission.
Daniel Oblinger: I should be able to here let me go. on control he
01:40:00
Daniel Oblinger: CD objects Sports Physio out
Daniel Oblinger: 
Daniel Oblinger: Get cool. Yep, I can do here. So I don't know why I wouldn't be able to do it.
Juan Barajas: I think sometimes you need to. eval
Juan Barajas: your SSA agent. Let's see.
Juan Barajas: 
Juan Barajas: So what happens before connecting to the instance you run this?
Daniel Oblinger: honest copy it.
Daniel Oblinger: That's weird. What's Dash ass?
Juan Barajas: I'm not sure. Let's see.
Daniel Oblinger: Okay.
Daniel Oblinger: Okay, and then I do the same Community before.
Juan Barajas: Yeah, let's See that works.
Juan Barajas: What happens if you do get pull? Dash yeah.
Daniel Oblinger: Notice what he's trying to do here it's trying to do. a bit bucket that org public key
Daniel Oblinger: I don't know what we're gonna say. Trying to do. Yeah.
Juan Barajas: what happens if you Get pulled and then Dash vvv or something like that. Let's see if it works like SSH.
Daniel Oblinger: - vvv
Juan Barajas: Vvp or just V?
Juan Barajas: That doesn't do anything. No.
Juan Barajas: that has happened to me a few times but I Remember how I sold it?
Juan Barajas: verbose
Juan Barajas: mmm
Juan Barajas: I think you can also try to connect to.
Juan Barajas: It bucket or something. I think you can do Let me look it up. Okay. just
Daniel Oblinger: Might actually just add permissions to that. instance
Juan Barajas: but if you see
Daniel Oblinger: we could just add the permissions directly to the instance as well. Right. So the instance itself has the permissions to do the pull.
Juan Barajas: I think you could add the keys to the instance and then Add that key set to bitbucket. To allow it to have permissions just like we do with circle CA. And I think it should work,…
Daniel Oblinger: .
Juan Barajas: but I'm not.
Juan Barajas: it's the best thing to do.
Daniel Oblinger: to have credentials on an instance
Juan Barajas: Yeah, an instance. Yeah.
01:45:00
Juan Barajas: Or we could just use an http.
Juan Barajas: link and then you would have to just provide your username and password anytime you want to do anything you get.
Juan Barajas: which is in exactly a deal but
Daniel Oblinger: not killer. But how would this work? We would just add a remote on its out too right now.
Juan Barajas: yeah, we can just change the link such that origin points to the https address instead of the SSH one
Daniel Oblinger: I see got it. Okay. so that would be
Daniel Oblinger: get config remote. Is that right? Is it?
Juan Barajas: thing you can do get Remote set Dash URL. and then
Juan Barajas: yes. all together set Dash URL a single word
Daniel Oblinger: Got it,…
Juan Barajas: and then origin
Daniel Oblinger: and then we got to go to.
Juan Barajas: and then the URL
Daniel Oblinger: And the URL we would get from the bucket, right?
Juan Barajas: Okay, yeah.
Juan Barajas: Sure this one.
Daniel Oblinger: two
Juan Barajas: And I think the https has your username. I don't know…
Daniel Oblinger: so let me just …
Juan Barajas: if you can do it. Yeah.
Daniel Oblinger: I do a clue. I should really do a clone here, that's what I'm doing. I would get this.
Juan Barajas: yeah, but
Daniel Oblinger: I would give you the address right here.
Juan Barajas: Yeah, but if you notice the address as your username, so Dan Dash public Nur.
Daniel Oblinger: Right, so right if we did this this Ops General would become. Dan's at least what we could do on this one is if we're going to do that, let's do this.
Daniel Oblinger: about that and so then
Daniel Oblinger: on that thing there's a Dano folder and in that Daniel folder is gonna be a repo that's
Juan Barajas: but this instance is going to die as soon as
Daniel Oblinger: can we stop it and then it's state lives out there this thing lives on.
Juan Barajas: we could also stop it.
Daniel Oblinger: That just gives me something I can run I don't know…
Juan Barajas: Okay.
Daniel Oblinger: I don't know if it's gonna be long term, but I'd love to have something like just wait a minute password for bit bucket.
Daniel Oblinger: I mean to bucket. okay. I know what it is.
Daniel Oblinger: Okay, that didn't work. Let's try again.
Daniel Oblinger: Okay.
Daniel Oblinger: wait a minute. Nope, that's a different password. Okay, try this again.
Daniel Oblinger: Make sure I'm really getting this copied. This is Dan at Sports Physio. Why am I?
Daniel Oblinger: Dan oblinger here at Sports Physio. I don't understand.
Juan Barajas: It's a big bucket. I think is your bitbucket username.
Daniel Oblinger: I'm sorry, I mean I'm looking at my bit bucket. Password that I have in one password and I have one for Dan at Sports Physio, but I noticed that here. I'm trying to clone. Deanna oblinger
01:50:00
Juan Barajas: No.
Daniel Oblinger: but this is my bit bucket. No, Look at that. the account is Dan and of course physio. So why is it cloning?
Juan Barajas: So I think the email. And the username may be different. So if you go to your settings. You can click on your yeah.
Daniel Oblinger: Yeah.
Daniel Oblinger: Good here.
Juan Barajas: Yeah, and then I think it's personal beatbox settings
Daniel Oblinger: Right here.
Daniel Oblinger: Where do gear?
Juan Barajas: personally, yeah. Here there you Dan Dash over no.
Daniel Oblinger: no username is I see.
Daniel Oblinger: okay. we don't want to change it. That's what it is.
Daniel Oblinger: I see but it should still be the password that I used to log into bitbucket, right?
Juan Barajas: I think so.
Daniel Oblinger: Yeah, it says no here is my get access password. Okay. Let's try that. I don't know.
Daniel Oblinger: Password forget so I have a really long as password Here. I don't know where I got this from.
Daniel Oblinger: But That's weird. So I have a different password get than I do for login.
Juan Barajas: okay. You could also if you wanted to keep it more secure. I think you can create an app password.
Daniel Oblinger: Yeah, all right, and then that's gonna only act then that password is gonna have permissions. that you assign specifically to that password.
Juan Barajas: Yeah.
Daniel Oblinger: right
Daniel Oblinger: So, we'll see. Once I do this I need to also go in and do a clone of data into the same thing.
Juan Barajas: Okay.
Daniel Oblinger: And then change Branch pull. I probably need a condo out there don't I?
Juan Barajas: I think it's already installed because it was derived from the Ami.
Daniel Oblinger: Okay. comment
Juan Barajas: But let's see.
Daniel Oblinger: maybe I will.
Juan Barajas: So you also need condom because it looks like it's not installed.
Daniel Oblinger: so I'm going to
Daniel Oblinger: To do the same SSH command here. It doesn't seem like I've got it Dash a. that's not really working - I
Daniel Oblinger: went to X
Juan Barajas: - I and then the name of the word path to the pen file.
Daniel Oblinger: this
Daniel Oblinger: right. I'm forgetting that. Okay, and the path to the pen file is gonna be? I'm in there right now. So it's gonna be whatever that is. Yeah. like that
Daniel Oblinger: Let me make a note on that because I'll forget.
Daniel Oblinger: you see that I'll just put it there.
Daniel Oblinger: Okay.
Juan Barajas: although you could included in your SSH config and just to SSH and then Workstation.
Daniel Oblinger: I see, okay, so let me just Where would I put that in my DOT SSH config?
Juan Barajas: Yeah, it's in dot Config. Yep.
Daniel Oblinger: Tilda slash dot SSH c o n f i g that file
Juan Barajas: That is what is in dot SSH slash config. so the file inside of the dot SSH folder
01:55:00
Daniel Oblinger: like that Okay.
Juan Barajas: Yeah.
Juan Barajas: And then the template would look like this.
Daniel Oblinger: No such file so I should.
Daniel Oblinger: wait a minute. No, I'm on
Daniel Oblinger: there's no file edit. an idiot
Daniel Oblinger: Okay.
Juan Barajas: and then the template looks like it's
Daniel Oblinger: Reduce text it to me or you just send it on the chat.
Juan Barajas: Yeah, I'm going to send it in chat.
Juan Barajas: Yep.
Daniel Oblinger: I'll just call it.
Daniel Oblinger: my athlete there we go. And then host name is gonna be that. IP address, can I put a comment in this file?
Juan Barajas: Let me see. I think you can but let's see.
Daniel Oblinger: and do I do it this way? I put the hosting and I put that user Ubuntu forward. Yes.
Daniel Oblinger: Sports Physio Threads and then it's the pen file, whatever that file.
Daniel Oblinger: s
Daniel Oblinger: I'm gonna call helps G for Ops General
Daniel Oblinger: for good measure.
Daniel Oblinger: And so then I should just say.
Daniel Oblinger: as is H a I think
Daniel Oblinger: boom That's pretty cool. Okay, so now I'm in there I can CD to Daniel.
Juan Barajas: Yep.
Daniel Oblinger: Alco 2 and then I can get clone again. back over here
Daniel Oblinger: this bucket data
Daniel Oblinger: clone copying
Daniel Oblinger: data Password, is that big? ugly thing
Juan Barajas: Do you want me to install corner?
Daniel Oblinger: Yeah.
Daniel Oblinger: Show me your screen. I want to see you. It's all kinda. yeah.
Juan Barajas: Yeah, I'm sharing right?
Daniel Oblinger: Yeah, how do I tell it to pin your screen large instead of mine? Is that possible? I'm presenting to not show my presentation screen.
Juan Barajas: Yeah, I think you can just click the pin on whatever screen you want to.
Daniel Oblinger: It pins it a little bigger, but it doesn't Is it possible to hide mine? I have to stop sharing Maybe. Yeah.
Juan Barajas: I'm not sure.
Daniel Oblinger: So you're installing condo for this user for everybody, right?
Juan Barajas: Okay, not just
Juan Barajas: I think so. Yeah. Just copy paste the snippet that they have on their website.
Juan Barajas: And that should work and you should just reload. Your shell and I think it should work. so exit try again.
Juan Barajas: either it works or first you need to execute this command. You couldn't have been calling it Bash. exit enter
02:00:00
Juan Barajas: Condom, there we go.
Juan Barajas: you should have gone.
Daniel Oblinger: Okay, so now let's do reload.
Daniel Oblinger: No. What's reload didn't work. Rehash is that? It's not reload, right? it's
Juan Barajas: to login again
Daniel Oblinger: you want me to log in again? Okay, I can do that.
Juan Barajas: Yeah.
Daniel Oblinger: No.
Daniel Oblinger: Okay, I'm back in and then Honda. activate SV
Juan Barajas: So there's conda we still have to create the environments.
Daniel Oblinger: Okay, so, let's see if I got condo. I got kinda good.
Juan Barajas: then
Juan Barajas: So now you can enter algorithms, too. and…
Daniel Oblinger: meaning and
Juan Barajas: run this command
Daniel Oblinger: and now I get pulled at this point.
Juan Barajas: I think so. Yeah.
Daniel Oblinger: So, let me just do it You get the password up again?
Daniel Oblinger: Okay.
Daniel Oblinger: Okay, and then check out. It should be feet. your ad DBC
Daniel Oblinger: That is that right.
Daniel Oblinger: Looks like it is.
Daniel Oblinger: Up late last night.
Juan Barajas: Not quite I don't know why I feel a little bit sleepy.
Daniel Oblinger: If I work for a while, I'll use to take a nap and I'll go wake me up. if I hack this long. I tend to like a little 10 minute nap will definitely help me. Okay, so I switched.
Juan Barajas: Yeah, I cannot.
Daniel Oblinger: What's that?
Juan Barajas: I could not sleep for just 10 minutes. I think that would be a two hour nap probably.
Daniel Oblinger: really? You can't just close your eyes and just take a nap.
Juan Barajas: Yeah, I know. I can't do that. Yeah, they also be like a longer one. Yeah.
Daniel Oblinger: It resets for me, in fact, I will just wake up. I won't be ten minutes. It'll probably be 20 minutes between 20 30 minutes. If I just close my eyes right now, I'll go to sleep throughout 20 or 30 minutes. it is…
Juan Barajas: Not cool.
Daniel Oblinger: because it resets me. The only thing is my body is now it's like it demands it.
Juan Barajas: Yeah.
Daniel Oblinger: If I'm working as much I really will find a place and so So I did the check out of this now I want to do What am I doing with condo now conductivate Or no I have to build the condo,…
Juan Barajas: It's created.
Daniel Oblinger: right? yeah,…
Juan Barajas: Yeah. Using the command that I sent on Jimmy. Yeah.
Daniel Oblinger: so it's
Daniel Oblinger: I see it right there. Yeah.
Juan Barajas: Where you can check it on the readme same thing.
Daniel Oblinger: Yeah, sorry.
Daniel Oblinger: Not that I really need to keep those notes, but get hurt.
Juan Barajas: So we're still recording this session. So it's gonna be a long video Yeah.
Daniel Oblinger: Long ass session. That's right. Yeah. It might be nice though. There might be like if I wanted to go to a certain spot. I'm not sure I'll ever look at it, but it can't hurt.
Juan Barajas: Yeah.
Daniel Oblinger: Yeah. Yeah, it will be long.
Daniel Oblinger: What are these numbers that are showing up there six eight nine?
Juan Barajas: Which ones?
Daniel Oblinger: On the status bars there. the pounds are digits at the very end.
Juan Barajas: I'm not seeing your screen Maybe.
Daniel Oblinger: do I stop show? I stop sharing my screen. No.
Daniel Oblinger: Right there.
02:05:00
Juan Barajas: The numbers. I think that's Yeah,…
Daniel Oblinger: then.
Juan Barajas: I think that's just the progress bar thing. so it's like
Daniel Oblinger: Inches like it counts up. It's tenths of a pound sign basically. I think okay.
Juan Barajas: yeah tons of accounts or anything.
Daniel Oblinger: it looks impressive
Juan Barajas: Yeah.
Daniel Oblinger: and to try to run the debugger there you would just Google around on pie charm see how to do that. Have you run a debugger remote on ec2?
Juan Barajas: I have not. But if I would I think I would just
Juan Barajas: I don't know. because I think I have set up a debugger for a Docker container and it's basically like a remote so it treats it like a remote you need to go to this address and then Mount this local directory on this remote directory and it works. So I'm guessing it's going to be something similar with the remote.
Daniel Oblinger: it treats it as a directory.
Juan Barajas: I haven't done it.
Juan Barajas: He treats it as a remote location. So the docker containers like You set up your debugging such that the debugger attaches to a process in that process In a local process ID is more of a you need to go to this address and…
Daniel Oblinger: So it's not a yeah,…
Juan Barajas: then connect to some process in there.
Daniel Oblinger: it's an IP address location with a probably port number or something like that on it. Got it.
Juan Barajas: Yeah, yeah.
Daniel Oblinger: I see and then inside the docker. I need to run a process there that basically is serving it up something like that.
Juan Barajas: I use debug pile, so if you use debug pie, and then I think run or execute or something and Dash and then the name of your script
Daniel Oblinger: I see. Okay So when it's writing,'s got it. You do run debugger.
Juan Barajas: Yeah.
Daniel Oblinger: You just run it locally. Is that right,…
Juan Barajas: Yeah, I'm mostly do a local development.
Daniel Oblinger: ?
Daniel Oblinger: So this is not done yet. This is
Juan Barajas: Nope. Yeah, that's why I use micro Mamba instead of combat. Yeah.
Daniel Oblinger: I see got it.
Daniel Oblinger: How do we have everything else? I guess? today I got data over there as well.
Daniel Oblinger: .
Juan Barajas: data, the thing that I think is missing is the changes to the environment.yaml file that include the editable install of DVC that So maybe that's not gonna get done.
Daniel Oblinger: but we check that in didn't we?
Juan Barajas: I don't think so. I think that's on my local copy. That's
Daniel Oblinger: let's check it in. but I thought I didn't know I think you did because I ran on my system.
Juan Barajas: I did. no good you had…
Daniel Oblinger: Don't you?
Juan Barajas: because you tried to run The environment creation and…
Daniel Oblinger: It failed,…
Juan Barajas: then it just failed.
Daniel Oblinger: still I had to have the editable yamo in order to try that.
Juan Barajas: governed But I think you did it manually. So pip install Dash e and…
Daniel Oblinger: Eventually I did…
Juan Barajas: then the path.
Daniel Oblinger: but before I did that we tried it with environment. and we're I can't actually run an environment. but remember we did it. We Just check to see if you're clean.
Juan Barajas: Yeah, we can check it.
Daniel Oblinger: I think you're clean.
Juan Barajas: that was on the previous machine. So let me put that one up. Yeah.
Daniel Oblinger: I can just also go on my local copy here. I'm right here. So if I do get pull I should be up to date and then I can just look in the environmental file, right? cats,…
Juan Barajas: Yep.
Daniel Oblinger: that's environment yamo and then Somewhere. I think its at the end maybe it's not the product. That's for sure it was.
Juan Barajas: Yeah. I think
Daniel Oblinger: I don't understand though because didn't we try running the environment? Yeah, I'm only crashed. Maybe we tried running it but it didn't have the latest version anyway.
Juan Barajas: yeah, I think that's what happened.
Daniel Oblinger: NYC
02:10:00
Juan Barajas: so I mean, it's a simple change. I think I could just do it on my local computer.
Daniel Oblinger: Okay.
Daniel Oblinger: you did on Briones. You see two instance.
Juan Barajas: Think I did it on my local school or my own machine, like my scratch test.
Daniel Oblinger: Your JB scratch one,…
Juan Barajas: Yeah, Jerry scratch.
Daniel Oblinger: I got it, right. Yeah.
Juan Barajas: Community now, please see that.
Juan Barajas: Okay.
Juan Barajas: Fireman, it's yeah, no.
Daniel Oblinger: is this one called Dan pipeline?
Juan Barajas: Yep.
Daniel Oblinger: Okay.
Daniel Oblinger: Fashion just take notes on that.
Juan Barajas: Okay, I just pushed the commit.
Daniel Oblinger: Okay.
Daniel Oblinger: And really Dan pipeline. I mean I can keep this around. I can just do it get pull anytime. I wanted to use it and update it, right.
Juan Barajas: mmm
Daniel Oblinger: I can take damn pipeline the one we created just today and it stopped forever. And then whenever I want to run I would just start it up again. See the end of the Daniel folder and…
Juan Barajas: Yeah.
Daniel Oblinger: yeah, okay. cool, so
Juan Barajas: Thank you.
Daniel Oblinger: I'm here. What am I doing now all kind of activate. kinda Go ahead.
Juan Barajas: Yeah, You could activate and then manually Ml dots as editable or you can just pull the changes and then the create environments command again, and then just wait for everything to finish and see if it automatically installed. Yeah.
Daniel Oblinger: crap, I don't but it would have to build from scratch to do that, screw that. Okay, so, right.
Juan Barajas: Yeah.
Daniel Oblinger: And anyways, Okay, I'm activated and so now I would just do pip. first I have to. let's see if I go into external is it there don't even have an external XT. Yeah, okay DVC that is there so I would just to install Dash e d
Juan Barajas: Hold on. did you see that in …
Daniel Oblinger: Yeah.
Juan Barajas: if it has something in it?
Daniel Oblinger: Nope. Yeah.
Juan Barajas: Yeah, I think we're get sub module in it and get some module update.
Juan Barajas: Yep.
Juan Barajas: Now it should work.
Daniel Oblinger: and now
Daniel Oblinger: it's out there quit it. tipped installed - e DVC Wait a minute. Let me go up a directory.
02:15:00
Juan Barajas: Yeah, or you can just do pip install the editable DOT.
Daniel Oblinger: I know. hip install Dash e and it's gonna be DVC dance that is that right?
Juan Barajas: Yeah, but I think you have to prefix it with a DOT slash. But let's see what happens if you just do that.
Daniel Oblinger: But how does it know that the underscore is the one that's gonna be installed? Of them.
Juan Barajas: because it's like yeah.
Daniel Oblinger: It's in the Tamil file thing. That's right. Okay, I got it.
Daniel Oblinger: Take a note on that.
Daniel Oblinger: But yeah, it's really.
Daniel Oblinger: done by the
Daniel Oblinger: got it. Okay, I should just be able to say that he fell. That is super cool. Okay. so
Daniel Oblinger: this environment is Up and running I can do stuff with it.
Juan Barajas: Set up.
Daniel Oblinger: Okay, so from a practical point of view. I could just keep this thing around and just pull to get the latest stuff off of how to mean, I've never even run our system yet. but the time I'd like to run our system is once we have that multiprotter.
Juan Barajas: Yeah.
Daniel Oblinger: Did you have any thoughts about the?
Juan Barajas: yeah.
Daniel Oblinger: Configuration of the multi-runner versus the configuration of that I had do we have an example of configuring them that we can look at?
Juan Barajas: what's the last thing do you have an examples of what?
Daniel Oblinger: Do we have an example of his multi Runner that we can look at?
Juan Barajas: Yeah of the matter Runner.
Daniel Oblinger: Yeah, that's right.
Juan Barajas: yeah, yeah, but
Daniel Oblinger: he CD dots
Juan Barajas: yes. Sure my screen.
Juan Barajas: It's this one meta basketball. For example this one this is a full run of the pipeline
Daniel Oblinger: got it. okay, can you cut just I know it's so trivial, but can you cut and paste that into the
Daniel Oblinger: Into the chat here and I'm gonna put it into the same file.
Juan Barajas: Yep. Okay.
Daniel Oblinger: the
Juan Barajas: Just to give you some context. Let me show you those. Pace basket ball Chase on and show you one stage for example stage one. Chase on yep. There you go.
Juan Barajas: So that would be the matter Runner config. The one in the middle is the base config. So the equivalent of common and then the one in the right is the specific config. for example the first stage
Daniel Oblinger: I see. So he does them all as separate.
Daniel Oblinger: He does them all as separate files.
Juan Barajas: Yeah.
Daniel Oblinger: Got it. let me copy one of those in.
Juan Barajas: so that's Yeah, yeah.
Daniel Oblinger: I can't copy from there. I'm gonna copy off your screen.
Juan Barajas: Of course.
Daniel Oblinger: Can you please give me the small one one the stage specific one? Yeah.
Juan Barajas: This one. Yeah.
Juan Barajas: I think it has a limit. Let me just send you by Slack.
Daniel Oblinger: Or use paste it twice.
Juan Barajas: But it would need to find the cutoff point. Yeah.
02:20:00
Daniel Oblinger: Yeah.
Daniel Oblinger: 
Daniel Oblinger: I mean it's easy enough to have. Specter, Json our expect.yamo, look like his config file. and in many ways it's
Daniel Oblinger: even a little bit cleaner because then it's gonna be a tiny. Config file.
Daniel Oblinger: It's just
Daniel Oblinger: If you did it that way you don't have a single object anymore that you can Whittle. So the only disadvantage of doing it the way that he's doing it presently is when you keep it as one object that you're working with then for example at the command prompt with that you can say something like this you can say, let's say this is I don't know. what data set was that using? One baseball that's baseball some so maybe it would be that, Rp5 is like the standard run of.
Juan Barajas: skip this one's the basketball one, I think. All that I sent.
Daniel Oblinger: I'm dyslexic. I'm just like so maybe it would be like,…
Juan Barajas: Okay.
Daniel Oblinger: the way that I'm imagining it Is he still doesn't have the input file specified here, right? And so
Juan Barajas: Yeah, you specify that and at one time I think.
Daniel Oblinger: right, and so I'm imagining that maybe we just Bebe one is just like the standard pipe for basketball right that we use right now. So let's just say that's what you did. So That's the standard multi runner for baseball our basketball, or maybe we say, BB run
Daniel Oblinger: The BB run I don't know.
Daniel Oblinger: Multi-run, I don't know. And maybe it's called So it's just bb1 but then what you could do is you'd say set.
Daniel Oblinger: and that input
Daniel Oblinger: equals, LT And hold out. And if you did that what you're doing is you're going into that config file and you're saying Hey, I want to change this parameter to that. If you didn't do that and you just ran bb1 it would run maybe lt10 was the default that it would run against So everything would be defaulted and you would just including the input and everything. It's just like This is what it would run. Right, and so you could change,
Daniel Oblinger: But I guess I might have a section if I were doing it that way I might have a Multi Runner, metrics.
Daniel Oblinger: Plus equals and then I would add
Daniel Oblinger: BB PCC, my test and so now if you do that, you're going to run bb1 with the standard of inputs, but instead you're gonna put that my test in there and so
Daniel Oblinger: If you're config is one. unified thing you then can work with it. You can also create. Sprint 23 is going to be
Daniel Oblinger: if you define Sprint 23 dot yaml, you could subclass bb1 and say yeah for this Sprint I'm running bb1 but I want to disable this one module the whole Sprint. I just want to operate with that module disabled, right? And so I'm gonna do that to it right again because it is
02:25:00
Daniel Oblinger: One config file you can operate on the whole recursive structure in those ways. If it's not one config file, what you would do is not that bad in Sprint 23, you would have another file, Sprint 23, past two would Define the parameters that you wanted for past two in this Sprint, and then you'd have to create a route. That was calling that structure with …
Juan Barajas: So you're arguing for all of this info to be in a single file?
Daniel Oblinger: Sprint 23 past two instead of the normal one, and then you would get your updated parameters but notice because it's not one structure. You can't operate on it anymore as one structure. So that's the disadvantage if you allow your config file to be Federated in that way, of course the advantages it's smaller right and you can
Daniel Oblinger: Almost and I'm not arguing. I shouldn't that strong but at least I'm proposing let's say that I'm just noticing that there's a difference here between the way that he's doing it and I'm doing it. I don't imagine the way that I did it. I still had a base. So this first one here this file that you have in the middle. That is like your Global parameters. I do not propose having that one. I propose that there is a defined Global parameters that you reference. And then if you look at my code over here, base is STD M proc arcs. So this
Daniel Oblinger: Thing here is like we all agreed. This is the name of where we keep our standard arcs for MC proc and…
Juan Barajas: But if we're allowing.
Daniel Oblinger: then we default to that,…
Daniel Oblinger: in the comment set here.
Juan Barajas: any standardization of…
Daniel Oblinger: but I am Taking the pain that you have on the right and…
Juan Barajas: where we store things and we all agree that that is the Defined set of default arguments that is going to live in our file system and…
Daniel Oblinger: directly pasting it in underneath processing. So there I just under preprocessing.
Juan Barajas: it's going to be persisted. Why not apply the same thing to the preprocessing main processing and…
Daniel Oblinger: I'm just putting all the parameters there. So Yeah,…
Juan Barajas: So have them let's say that the first one I saw.
Daniel Oblinger: that's what's gonna happen.
Juan Barajas: Because I think it's pretty common for several pipelines to use the main processing and then a slightly different end processing. And if we were doing it this way, we would have to basically copy paste the vast majority of the file and then just change the end processing part.
Daniel Oblinger: no, I know What I would do for that. is I understand what you're saying it and again, I'm not strongly arguing for anything here, but you would do this.
Daniel Oblinger: all right, and so if you had Standard preprocessing args that you wanted to like that you didn't want them to be in this file. You wanted everybody to use the same standard ones. Yeah, you would base off of this and undoubtedly the definition of this would recursively base off of standard. proc arcs Right until it would just inherit right up through there. if that's what you want. If you wanted to basically maintain a global name for that. That's what you would do. What he's doing here is a bit different than that.
02:30:00
Juan Barajas: So it's basically…
Daniel Oblinger: What he's doing here is in his local run.
Juan Barajas: what instead of configs in Json we would have convex in that…
Daniel Oblinger: He's referring because I'm assuming this full pipeline slash stage 1 process.
Juan Barajas: but that's basically the same thing because here it's on it's not specifically in the Run itself.
Daniel Oblinger: Json config is inside the run for baseball Arts.
Juan Barajas: It's just on a common config. I think it's in source.
Daniel Oblinger: It's inside full pipeline.
Juan Barajas: Sourcing configs and…
Daniel Oblinger: So he's doing that.
Juan Barajas: then here are all the basics or…
Daniel Oblinger: He's actually
Juan Barajas: the common convex that you can refer to. So it's basically the same idea. It's just instead of Json having a debt.
Juan Barajas: because
Daniel Oblinger: got it.
Daniel Oblinger: Got it. Okay, let me see those folders really quick the ones that are in the standard configs.
Juan Barajas: Yeah. I don't know. I think it makes more sense to be dot cfg.json,…
Daniel Oblinger: Full pipeline AWS.
Juan Barajas: but I don't know why they did it.
Daniel Oblinger: Let's go into one of those pool pipeline AWS.
Juan Barajas: Yeah.
Daniel Oblinger: right
Daniel Oblinger: Dot json.cfg That's weird that you have CFG as a suffix. Why would you do that?
Daniel Oblinger: Right, right. Does that's wrong actually, but okay.
Daniel Oblinger: Yeah, if my system would handle it out of the box if it actually had the right extension on it. You're actually going to screw me because I won't know what to do with a DOT CFG. I can hack my system for the moment and just if you say dot CFG you meant Json our DOT cfg.json.
Juan Barajas: I think I would maybe replicate all of these ones…
Daniel Oblinger: because the reason what I could do is I could actually
Juan Barajas: but then adapt them to be in that format and…
Daniel Oblinger: Allowed do in other words. Those could be due loadable.
Juan Barajas: then just leave this alone…
Daniel Oblinger: Right you using your method…
Juan Barajas: because I think
Daniel Oblinger: where in the debt config you just reference this in writing the source you just say, this is the definition for pipeline, right?
Daniel Oblinger: okay, but let's be precise here in our wording when folders that are that loadable when I do format when I say do format I mean things that are due loadable. So do you mean that do I think these should do loadable. Because they're source code.
Daniel Oblinger: to me the way I've been thinking about it, you can make them that loadable, but I think you want them to do loadable. remember that the way that do operates.
Daniel Oblinger: It's is an 00 language Where a Json object is an instance in the new language and you can and you can inherit from it and you can do all the inheritance stuff all of those. functions are due functions that operate on do objects, right and so just to be clear when you say here notice the dot here when you say Bass. broad dot this that is a do load of that object. Not a DAT load.
Juan Barajas: but I would imagine that something is gonna get executed. So is
Daniel Oblinger: But not if you do do load. but yes, if you just do do what's gonna get running. But if you say do load it's literally the same as in Python if you were to say de To be this then you were to reference X versus referencing x with parentheses. If you reference X without parentheses,…
Juan Barajas: Yeah.
Daniel Oblinger: you're loading it. Basically you're giving me reference to it. And then…
Juan Barajas: yeah, but then what's the difference between do load in that load?
Daniel Oblinger: if you
Daniel Oblinger: the object space that's being loaded from when you do it, so a debt is a folder, but that's a folder period But a Json file is not a folder.
Juan Barajas: Yeah, I get the part. Yeah.
Daniel Oblinger: So you could that load a folder that had a Json file in it. that's fine. You can do that and we could put all these jsons in a gap older. And then what you would do with it then is you would download the folder. And then you have a path and then you could load the python file or the Json file if you wanted but notice that whenever you use operating at the file system level whenever you use the word do your operating at the python object level?
02:35:00
Juan Barajas: So the way that I was seeing is Earth thinking of it is if you want something to persist as a file somewhere and then share it or reuse it then it's gonna be at that. and to
Daniel Oblinger: No a little bit. No not quite. let me edit your thinking and…
Juan Barajas: Yeah.
Daniel Oblinger: I started out thinking that way and I changed it and I think my new Thinking about it is better, but the source code of our repository persists I want to persist my source code right?
Juan Barajas: where
Daniel Oblinger: So there's two kinds of persisting that happens in Sports Physio sport one kind of persistence is stuff that's get checked in. And there's another kind of persistence which is checked in. That is F3. is get So now the question is where do you want to check these files in and if you want to check them in to get then is the natural way to talk to it. You could use either for either but you're really messing it up if you do the other and so the question mark here is if you want these Fig files to learn You could do it. Either way you really could using the current debt architecture. If you wanted these config files to live in S3, then you would put them in dats and that's how you would deal with it.
Juan Barajas: so the way that I was thinking of it is source code is gonna be persisted but it's gonna be persisted as a unit. So everything has to work with the current state and everything only work with the current state where you can only assure or you can only make sure that the thing works with the state that it's currently in and nothing else should be from the past or from the future. Otherwise, it may not work.
Juan Barajas: And then artifacts are things that one version May produce and then future versions may use and you can interchange that with any other thing at any point. So I will
Daniel Oblinger: Unaccept backward compatibility of data formats but …
Juan Barajas: Yeah, ioring. microcompatibility
Daniel Oblinger: that's a big caveat. but okay assuming you didn't change data formats. That's right.
Juan Barajas: Yeah, so…
Daniel Oblinger: Yeah, right.
Juan Barajas: what happens if I'm making some changes and then I Implement a new configuration thing or something in the source code that I think it's going to improve everything and I want to rerun the same config that Martin used three weeks ago. How would I do that? In my mind, you would get those configs from somewhere and…
Daniel Oblinger: right
Juan Barajas: that would be the past and…
Daniel Oblinger: I agree you if that's right.
Juan Barajas: then run on the current system. Yeah.
Daniel Oblinger: And this is the way and I say, I'm proposing not arguing here because I don't have strong feelings about this. But what I had in my mind was
Daniel Oblinger: that there isn't a one size fits all to Json config, is it always in that or is it always in due and it's in both? So what I imagine is
Daniel Oblinger: that Ops of the hierarchies as these things inherit from each other. I'm thinking of the tops of these hierarchies as being more like source code and they change with the source code. So I'm imagining that the tops of my hierarchy for example, all of my parameters that are in the very highest one that you showed there that everything inherits from that one. I was imagining goes with the source code because essentially if I were to check out a different version of The Source Code, probably the parameters changed with it, and so my default file better change with it. I can't let those break apart from each other now.
02:40:00
Daniel Oblinger: If I'm running a single run of my data and that changes with my data and it would be in the debt system. So what I was imagining happening is
Daniel Oblinger: In the debt system, you would have a speck.json file. And in that specific Jason value, you have all the parameters that you set for that moment. So if I went three weeks back in time, I would just go back like you said I could just without changing the data.
Daniel Oblinger: pointer or changing the get pointer I could just because everything is all the runs are actually, timestamped I could just go to an earlier timestamp if Martin saved it anyway and there's all the parameters he used and I could just run it and it would just run right and…
Juan Barajas: Yeah.
Daniel Oblinger: in that way it's doing exactly what you proposed which is it's in that file system and the spec that Json is there but that's like that g son references a bunch of variables which are these standard things that I'm thinking of that I'm managing as source code in the sense that I need to validate them and do my QA testing on them and this set of parameters works, it's good check that in those sets of parameters that are the common parameter sets those kind of go with my source code and I'm managing them as if they were source code and so in that sense. I'm managing as do's and quite literally I would use the do mechanism when I want to manage something as source code. and so the breakover to win it's being managed as
Daniel Oblinger: source code and not is really the break over between that and do so what would happen is in the local? File tree. You have a spec that Json which is in the DAT, right and that can refer to here this
Daniel Oblinger: if we wanted to do it that way, what we could do is we could do just like he did here. if you put config and then you reference it here. It wouldn't be full pipeline. it would be Something like this if you wanted to have it local,
Daniel Oblinger: I would either want it to be in this directory with this Dad in which case it's local and you're doing whatever you're doing with it, or I would want it to be in do hierarchy. I don't think I would want it to depend on another dad. if I'm depending on another dance for my configuration. you could but I just feel like if you're doing that it seems like you're treating that other dat as a source code as I and this thing I'm managing which is the correct version of Something. Although I don't know it's hard to say because
Daniel Oblinger: I am referencing a model the most recent version of the OCR model, that would actually be referenced by that name, right, which is the latest OCR model and…
Juan Barajas: And then if I wanted to create a new config just as let's say that I am testing out a new intermediate stage.
Daniel Oblinger: so I'm not using to do hierarchy for that. So you could have a DOT object that represented a set of parameters, right and…
Juan Barajas: between processing and…
Daniel Oblinger: you could reference them, but I don't.
Juan Barajas: Main processing What would I do?
Juan Barajas: How would I do that? But I would look like
Daniel Oblinger: I wasn't imagining to do that with common configuration.
Daniel Oblinger: I was imagining to do for that and I was imagining to leave it checked into the source code it would be in the source code.
Daniel Oblinger: right What would you right?
Daniel Oblinger: So let's suppose that I will edit this to make it kind of do
Juan Barajas: Another question now that you thought. If I see a dotted name, how would I know if it's that or a do?
02:45:00
Daniel Oblinger: that
Juan Barajas: that's just
Daniel Oblinger: Base and then I'm assuming that we have …
Juan Barajas: okay.
Daniel Oblinger: I guess this is a yaml file so full pipeline.
Daniel Oblinger: Dot stage preprocessing let's say it's like that, right? Uh-huh.
Daniel Oblinger: It's a do. that slashes Do's have dots. I changed it because of that we used to not be that way and I changed it. yeah, so that's how yep, so
Daniel Oblinger: But also remember they're just strings and yammals. So you could take a new name and that load it that would be dumb, but you're explicitly in the code. doing it other than base which has a semantics to it. Nothing else really has a semantics They're just But to answer your question you would create This is in Sprint 23 Dot yaml and in that you would have actually let's just do it in Python and then you would say Main. equals, and you would inherit from the standard Runner and then you would just say, stages. Deep Rock right and then…
Daniel Oblinger: 
Daniel Oblinger: 
Daniel Oblinger: I'm bastardizing yaml and…
Daniel Oblinger: Json but who cares and then right in here you would have. preprocessing And then in this one your dad. base would inherit from pre-proc and it would be define with the parameters that you wanted and it's History here. It would hurt from JB. 23 that. base Would probably still point to the standard,…
Daniel Oblinger: 
Daniel Oblinger: 
Juan Barajas: but
Daniel Oblinger: prod was TV free product
Juan Barajas: what the bottom part is yaml and the top part is python.
Daniel Oblinger: I just was getting lazy there what I might do for this is actually put it all in a yaml file. and then
Daniel Oblinger: I think I can do that, but I think Maybe I can extend it. so that if you ran JBS 23 and it found a main it would run the main what's convenient about doing it.
Daniel Oblinger: This way I'm trying to be clever here and allow it to all be in one file. If I didn't do it in one file, I would just have this be my main. and then Yeah, let's just not do it as one file. And then I know I can make it work. So here stages. This is preprocessing. is and it's
Daniel Oblinger: and then this is a different file. So here instead of calling a JD 23. I'm gonna call it JB 23.
Daniel Oblinger: X preprock and then hero in the file system. I'll have JB
Daniel Oblinger: 20 23
Daniel Oblinger: actually
Daniel Oblinger: and now it's just all files. I think I can put them all in one file. It's just the only reason that I had a little bit of trickiness if I put them all in one file is at the command line. I want to be able to just type. that's JBS 23
Daniel Oblinger: and right now if you did that. It would run this whole thing.
Daniel Oblinger: I probably could still do it. Yeah, if I did it that way I would put at A dance section that ran it. I could think I could do that.
02:50:00
Daniel Oblinger: Pride Ms. Pipe run
Daniel Oblinger: all right, and so that's gonna
Daniel Oblinger: but I'm being a little clever if I do it this way because what I can do is, Extra parameters, or just I'll call it XX.
Daniel Oblinger: I can do that.
Juan Barajas: think I'm pretty lost at this point, but
Daniel Oblinger: So what I did here was this yaml if I structured it this way this top level yaml is what's gonna run? The Monty pipe which has a bunch of stages in it. And of course the Preprocessing is one of the stages. and now I'm inheriting. But look what I just did. I said go and find JBS 23. that's this file. Look up XX. yeah, that's this. Find free proc. And then this is what you're inheriting and then in this one, I have a base of standard, preprocessing, as my base and now
Daniel Oblinger: module one
Daniel Oblinger: He won. funky
Juan Barajas: Yeah.
Daniel Oblinger: value Or s23. you said Dan. How would I use the tool to If what you're saying is hey, I'm working on a new branch and I want to Delta from existing standard. How would I do it? And here what I did is I said, okay. Yeah, I just grabbed the standard, although I probably didn't run. Ms. Pipe run I probably had
Daniel Oblinger: a broad
Daniel Oblinger: standard BB run right. there somewhere is a definition of a whole. Multi-pass run that says all the stuff it says. yeah. I got this is how I do my preprocesses now, I do all my stuff and you could standard BB run. if you did we might even have a section called standard. STD, and that is a standard multi-stage basketball run. It's going to use LP 10. all the modules got everything set up. That's way it is and now you can make arbitrary. changes to everything in it. right
Juan Barajas: so I think a follow but I think it's slightly confusing. So for me to understand it it would be If I looked at this file,…
Daniel Oblinger: yeah, it's
Juan Barajas: then I would have to know that keys. Special and what base means and where to look for the thing that the thing is inheriting from?
Daniel Oblinger: absolute You yeah for sure, but those are table Stakes for using that and do the 10 but I don't think it's that hard that part. I would push back that that's a problem. In other words. as a team. We are gonna have to basically curate that namespace and a new namespace and we're gonna have to know not just what that does generally but you're gonna have to know
Daniel Oblinger: which names in the DAT namespace we are guaranteed to be what they have specific meanings and the same for the due namespace. there's certain Keys which are out there which are document like we have regression tests and all that stuff against them because it's the same as knowing what pipeline means or full pipeline like you can't really understand what gregorish's script is doing without understanding what full pipeline supposed to mean and what it does and you need to know that Or at least that represents the standard of this right the thing that I'm doing here, which is clever in a good and bad way. Is I'm letting you wrap. everything up in one structure including
02:55:00
Daniel Oblinger: your overrides anywhere in the other structure. So if I'm giving you one whole structure which represents a recursive tree of dance, right and then I'm letting you have another structure. Which defines Deltas against the first structure? Right and you could have another one which defines Deltas against that by the way all of this is going to get resolved out into a spec that Json that you can look at. So when you run it in the file system the resolution of all these things is actually I think that people will regularly look to see what parameters do I have and then if the parameters are not what I expected to be I need to go back and look at my original spec. without arguing against what I'm proposing here versus what jiggish has. I think the advantage of what jigich has is everything is kind of more spread out. He's kind of look at each one, but the disadvantage of jiggish's is
Daniel Oblinger: If I wanted to override. In there, so I wanted to take his standard one. And then while I'm working in this Sprint override it. You're tempted to just copy his folder and I'm edit it right because to try to specify it as a Delta against the other one. Gets a bit messy, And with mine you could cut and paste it and just edit it but notice it is possible with this to actually specify I want to run with the standard BB run and I want to zip in here and change this parameter and zip over here and changes parameter and it's very parsimonious. But a little abstract. Way of declaring that what you just did.
Daniel Oblinger: I'm not arguing against it, but that's the trade-off there is if you do it in other words the way that Jesus is doing it now. It's perfectly with the debt structure. Both of them. Do there's no actual problem question mark for me is really If you keep it as a unified config file. If you mash all of his stuff into one config file. It then allows you. You wouldn't even have to do it. But it allows you to do stuff like this. You didn't want to do something like this. You could just not be his dad put in another place and edit it in place go in your editor and just added the pieces you wanted and now you have a new one.
Daniel Oblinger: So you still could do this same kind of copy edit with even if you unified it, but the thing that you can do with the unified one that you really can't do. without it being unified is to specify a Delta against it as a unified thing. Right, if you specify a Delta the other way it becomes it's not that bad. If you did it his way. What you would do is I started doing it somewhere you would create.
Daniel Oblinger: A sub file and in it you would then reference your own version of one of these and then you have to have another file there and you preference that right. So you'd build a new top level and you'd build a new. Sublevel, and you'd override it seem to end up with a couple files that referenced each other.
Daniel Oblinger: Might be simpler.
Juan Barajas: I think I'm gonna have to wait and A concrete example, yeah.
Daniel Oblinger: I'm pretty example of both of them. Yeah. Yeah, if I get a minute or two, I might just take This specific example and try to kind of write it out in both forms and…
Juan Barajas: Yeah.
Daniel Oblinger: and let's look at it. So specifically what I'm going to if I have time to do this it will basically be
Daniel Oblinger: Imagine that there is a multi-runner and then someone has written basically. Standard basketball run which is gonna take an lt10 and then run it and put some metrics against it and do that then during a Sprint. They wanted to subclass it and change some stuff on it and in both cases, I'll show what it would be like if you subclassed it and did that.
03:00:00
Daniel Oblinger: I guess what you're saying is. Hey Dan for the one that's closer to what jiggish did you kind of want me to do it as dance. So in other words maybe not the root ones, but most of the config files would actually be in the debt would be an adapt folder and then Would you want me to have one dat reference the other one in programmatically like paste the values or would you want me to just cut paste one to be the other one and then edit it in place just show that.
Juan Barajas: One I'm not sure what the debt because I do agree with you that the most recent at least defaults. Configuration of something should be tracked us source code so that I think that's fair.
Daniel Oblinger: just a comment on that a little finish your thought.
Juan Barajas: Yeah I think that's fair. But I also would like to have the ability to get you say that at the very end. You're gonna have a single result spec or containing all the parameters with that. Everyone is gonna execure.
Daniel Oblinger: Yes you yes, it will have that right? Yes.
Juan Barajas: And I'm guessing that that is going to be an artifact produced by our run. So it's going to be at that or that is going to contain that part of it.
Daniel Oblinger: Yes. Yes, it will right.
Juan Barajas: As long as you can reuse that one and then also plug it into the runner somehow. I think I'm okay.
Daniel Oblinger: I Got What I'm imagining just as I do these out is we would have in source code.
Daniel Oblinger: Standard default params standard preprocessed params standard, those would be listed, as a group of parameters, and those are due tracked those are definitely in the do hierarchy but then I'm imagining that the rest of it is actually and there probably would be a standard Multi Runner that says yeah, please use this. for the preprocessor and use this it's the default values. but then what I would do is I would have a dad.
Daniel Oblinger: Which now references it bases on those right? And so I would have adapt config that's in the dance space that says okay, but for this Basketball in other words what I wouldn't check into the source code the standard run of our metrics, for example, I might have …
Juan Barajas: Yeah.
Daniel Oblinger: M1 does our standard metrics, for our system. It runs lt10 does and then somebody else can base off of M1. Right? and now
Daniel Oblinger: there's the way I can base off of M1 is I'll probably have source code the generic template of M1 not maybe I would actually just have a dad which this is the dad that runs M1 and it LT 10 and all that kind of stuff in there and then there's two ways of doing it I can either if I want to have on Sprint 23 that I referenced that I can just Override M1. Which remember is already at that I'm just going to reference that other dat and then override it or I can just cut and paste. you literally cut the folder if you wanted to do a different version of that you're starting to figure out what you're gonna do in this Sprint you actually cut paste the folder of them1 and then change some of the parameters around it you're working on it.
Daniel Oblinger: and then maybe later you, check that in as the new version of M1, right if that's what you wanted to do, but it would live on is this thing. That was that Iran, but there's no real connection between M1 and this thing it's just you cut and pasted it and then started editing the parameters and the other thing you could do is is really you're still inheriting from it. And so if m1 is in the do space then you can actually inherit from it override it and so what the way you would run without overriding is It wouldn't exist in the DAT space at all. He would just say, do M1 and it would run and one.
03:05:00
Daniel Oblinger: But then if you wanted to override it, you would say Sprint 23 inherits from M1. And you'd say do Sprint 23 our JB Sprint 23, and then that would inherit the parameters that would do the trick that I was showing here where you're inheriting the parameters.
Juan Barajas: Yeah.
Daniel Oblinger: So I guess I'll show you both. I'll show you the cut paste and I'll show you the inherent parameters one and yeah.
Juan Barajas: Yeah, just to see it. It's actually how it will be.
Daniel Oblinger: I think I want to see it too. I mean, I have a pretty clear idea in my head, but I think when I get it actually written out I'll have a clear idea in my head. Cool,…
Juan Barajas: Yeah.
Daniel Oblinger: we should probably cut it off though. It's fun to think about. Yeah. Okay.
Juan Barajas: Yeah.
Juan Barajas: but
Daniel Oblinger: I'll show you both those when we get a chance.
Juan Barajas: Yeah, for sure. I was gonna say don't forget to stop your instance because there have been times where I just forgot and…
Daniel Oblinger: I would forget that.
Juan Barajas: then two hours later. It just pops into my head. Yeah.
Daniel Oblinger: So let's do that right now. So. I am
Daniel Oblinger: right here. Dan pipeline running and I bet there's an option here to stop it.
Juan Barajas: You can just click it. I think yeah and…
Daniel Oblinger: Right. Click it. pop instance stop
Juan Barajas: stop instance. Yeah.
Daniel Oblinger: and then
Daniel Oblinger: and then all I would have to do if I wanted to run again was literally started and then SSH back into it and get pulled.
Juan Barajas: Just started then I think update the IP just in case he changed and then SSH into it.
Daniel Oblinger: I see. I've now put that in my SSH config. So I would have to go and edit the SSH config. And is it likely to change the IP?
Juan Barajas: Yeah.
Juan Barajas: I think it always changes, but I'm not sure. I think it always changes on this you attach a permanent IP and…
Daniel Oblinger: Got it. That's too good.
Juan Barajas: that costs.
Daniel Oblinger: Would you try to pay for yeah.
Juan Barajas: Yeah. because money
Daniel Oblinger: got it, so I should probably not have it in my SSH config
Juan Barajas: I think it's easier just because you only have to update it once and then you just forget about it. I supposed to if you want to run three commands one SSH and then one another SCP or something. You need to remember to update every time you're on the command, but that's up to you. Yeah.
Daniel Oblinger: I got it. Thank you, sir. I will talk to you tomorrow.
Juan Barajas: No, thank you. Yeah, see you tomorrow.
Daniel Oblinger: What are you gonna do with the day off you were trying to move your day off and you weren't able to.
Juan Barajas: Yeah, I forgot about the Wednesday. What's the meeting?
Daniel Oblinger: right
Juan Barajas: Nothing special. I think it would feel better to have many days off in a row instead of just one day off and then two days working and then another set of days all. Yeah.
Daniel Oblinger: I think I got it. all right. …
Juan Barajas: Yeah, because yeah.
Daniel Oblinger: I mean you could do it. It's just you'd end up being on your day on, two different times during that time. I don't know. Yeah.
Juan Barajas: Yeah, at that point is not even a day off. So now let's just leave it was done.
Daniel Oblinger: Right, right. just you could take the day off do those two meetings and give yourself those hours off on another day. So you would at least get the time off if that was more valuable to you certainly could do it. But yeah,…
Juan Barajas: Yeah. Yeah,…
Daniel Oblinger: I know what you're saying, anyway.
Juan Barajas: I think they may just go full on work mode for a day and then just forget about it.
Daniel Oblinger: Yeah, yeah and in general, I think that sports music should be really pretty loose about this my view of it is just that you and I have a special extra. Responsibility of tempo we are the tempo of the team and so it is important that even when we are taking our day off, we kind of make sure that the tempo is kept up at least…
Juan Barajas: Yeah. Yeah for sure.
Daniel Oblinger: if other people are working. Yeah.
Daniel Oblinger: Awesome, I will talk to you tomorrow.
Juan Barajas: Yeah. See you tomorrow then. All right.
Meeting ended after 03:10:01 👋

