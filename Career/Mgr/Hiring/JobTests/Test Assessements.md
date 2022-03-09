
-- saying you will do too much, is just bad as too little or wrong thing
-- de-risk your proposal as much as you can
-- invent the details you would have found.
   ==> if a DEV cannot execute a days work, with the level of detail you provide, you have failed.
       saynig you dont know enough it is never an acceptable answer.  
       imagine who you would ask, iamgine an answer they gave 

## Marcin

9:50  test2


## Alex Dulub

16.05.19  8:15a - 


---DAN--

Out of the box Davide's full contribution is perhaps the most polished I have seen on this task.
It feels completely done.  Alex's by contrast is quite a bit less strong.

Alex also performed a complete rewrite of his code (the flood fill part)
His first attempt at the code was pretty complex.  It kind of reminded me of a Rails application.
In his head he was following some kind of a template, and this lead to quite a bit of boiler plate.
(I noticed that as he asked, and as a consequence he just redid his work from scratch (without asking me).
So I dont have a perfect way to match his work to others.)


Summary:
Given the many other reasons to find value in Davide (he has founded his own robotics startup)
and has super deep knowlege across the robotic stack, I think this choice is a no-brainer.
I think his code *is* better too, but when considering the full package, there is no question.

My sense of Alex is that we should re-consider him later when the $5M comes in, but he 
is the opposite of Davide, in that he has no specific robotic knowlege.  So for now, this 
plus his medium coding performance, does not clear the bar.




---GREG---
Alex
Pluses:
+he use namespaces
+he use templates in meaningful way
+he use qualifiers like const, override
+he have headers for a lot of functionalites 

Minuses:
+He did not implemented everything so at some moments it is bit harder to follow
+I do like to wrap the implementation in the namespace, but it is a preference thing I think 

Summary:
+I like that way he write the code
+I don't have the feeling whether he did a lot or not in the given time (those 10h is really not a lot), probably Theo will have better judgement here

Davide:
pluses:
+The first impression made by the readme.md is positive!
Davide:

- Coding style not consistent with himself, random spacing etc...  Could come from not caring about readability or from Internet copy/paste.  In both cases that's a red flag for me.
- Doesn't know that struct typedef are useless in c++
- The whole addToStackIfEdge loop that looks for the 8 neighbors is horribly hard-coded instead of being in a loop
- Same for addToStackIfSimilar loop
- recursiveFloodFill isn't recursive
- Tests aren't automatic (need for a human to look at an image)
- Very good README
- Regular commits, even though they aren't very verbose

Except for these, the code looks functional and thought of.  I haven't tested the code though.

Alex:

- Not verbose commits
- imagescan processor parameters aren't used
- FloodFillRecursive is recursive ;)
- Image loader isn't functional?
- Most of the code looks empty
- Apparently he has several branches:  master, new_version, task_2...  Which one is the good one?
- Branch master requires Visual Studio to build... F**k it
- Branch new_version doesn't build:
g++ `pkg-config --cflags opencv` -I "=opencv" src/*.h src/*.cpp `pkg-config --libs opencv` -std=c++11 -o ImageProc.out
clang: warning: treating 'c-header' input as 'c++-header' when in C++ mode, this behavior is deprecated
clang: warning: treating 'c-header' input as 'c++-header' when in C++ mode, this behavior is deprecated
clang: error: cannot specify -o when generating multiple output files
make: *** [all] Error 1
- Branch task_2 has no README, no Makefile...

I'm really not impressed with Alex



## Sevrin

SEAN --
It seems I'm late here...but this is my feedback.

Build & Run --- I was able to build and run his code without any
problems. The program seemed to produce the correct output for the
"find region", "find perimeter", "display image", and "dump pixels"
tasks. I ran a comparison smoothing on and off and this also appeared
to be working correctly.

Program structure --- Everything sits inside one class file, which is
not great. The program appears a bit monolithic and I would have
preferred to have seen individual classes for specific
operations. Also having a class called ImageProcessor where most of
the logic sits, and a top level "main" program called ImageProcessing
doesn't help readability.

Documentation --- I think I would have liked to have seen more
documentation/criteria on his choice of of algorithms for
smoothing. Not sure if he sent you an e-mail with his thinking or
not. He does make mention of using Gaussian weighting to improve the
weighting / comparison of pixels, but there isn't a lot of detail
here.

Overall --- Code does what it says on the tin. So that's good. I do
believe he'll need some guidance with regards to code structure and
good coding practices.


THEO -- 
- I would be interested in why he chose to flood fill recursively rather then with a simple loop. 
  He had the experience to check the stack space for this approach, but he could have just kept a point set or a mask image using a 
  loop and thus not risk the stack.
- His color distance function looks odd. He repeatedly avoids using abs() in favor of if:else statements. He could also save substantial 
  cpu time by not square rooting the color distance and instead comparing the distance squared to a threshold squared.
- He states in his read me that he would improve this function by not using a simple vector distance of [R,G,B]. 
  I think we should ask him what he would change it to. I would expect him to discuss converting RGB -> YUV (or some 
  similar format) so as to compare only the chroma characteristic and factor out luminescence/brightness, 
  but regardless of what he has in mind, how he talks about that upgrade will tell us a lot about his thought processes.
- The csv file choice is curious. I don't have a problem with it but it would give some insight into his thought processes to 
  know why he chose it.
- Lines 499 through 502 of 'ImageProcessor.cpp' accomplish nothing. That code is likely vestigial.
- I like his smoothing approach as it functions to remove noise for both the perimeter the flood region. 
  However, that kind of windowed approach annihilates regions of sizes below the half window size. If you want to 
  smooth all possible flooded regions, that could be a nasty edge case.
- I would follow up with some questions about his approach here: Ask why he chose this method, what other methods he 
  considered and what pros/cons he weighed. I would be interested to know what if any questions he asked prior to starting 
  this smoother code regarding the prompt.
- Your prompt stated " remain generally close to the original boundary for this operation. One could argue this type of smoothing 
  fails that requirement.
- In your prompt, you stated "potentially be expressed via lower degree parametric representation". I think we both read this to 
  mean that you want a vectorization of the perimeter into arcs/lines and implicitly the perimeter segments to be an ordered list 
  for each perimeter. In retrospec, we should have been more clear on this point. I would hold that against him much, unless he 
  failed to ask any follow-up questions on this part of the prompt.
- His '-example' argument does not exhibit much in the way of test code. I was really hoping he would show more self-verification
  than that. Perhaps he does not have much prior developer experience (he is a recent grad right?)
- His csv file does not order the perimeter points. That is unfortunate since I wanted to judge his handling of edge cases 
  surrounding that.

Overall: Lukewarm on him



BRANDON -- I didn’t get into an assessment of the algorithms. But- I
found the code to be quite readable, and basically clean, and the
organization is suggestive of algorithmic implementation that’s at
least good.  Even though I’m more C than C++ at the moment, after a
once-over of the code it wouldn’t take me long to figure out where
exactly to look for where something was happening.  I haven’t actually
rendered the doxygen docs, but I used doxygen extensively in my
dissertation (and used that documentation to remind myself of where
things were at times), and he’s got the functions described in a
useful manner.



GREG
Cons:
+helpers methods could constitute a separate class/file
+ImageProcessor.cpp:419
dist = sqrt(((int) (d1 * d1 + d2 * d2 + d3 * d3)) * 1.0);
Cast to int and then multiply by 1.0 to transform back to float?
I might don't know sth about c/c++, but this doesn't look like a nice hack.
+getter methods returning value by a reference parameter.
+ImageProcessor.cpp:501:
boolean operator "=="? probably typo?
regSmooth.at<uchar>(i, j) == 255;
+ImageProcessor.hpp:18-19:
using namespace std;
using namespace cv;
Importing namespaces in a header. He don't know how to use namespaces properly.

Pros:
+ImageProcessor.cpp:463
inline void ImageProcessor::smoothPixel(Mat &regSmooth, Mat &reg, int i, int j, int windowSize)
One could say this function is too long to be inline, but taking into account how it is used this IMHO is a good decision. 
Definitely, he knows how to use inline.
+By the way he operates with pointers to the data, he didn't need a sample from me. 
+Every function is nicely commented.
+Variables/functions named clearly.
+I didn't have problem to follow the code.

Overall:
Positive. I think all cons can be eliminated with PRs with a code review.



AXEL
If he's a fresh out of Phd, then from my perspective it's thumbs up.  His coding still is quite clean, even though he isn't always coherent with himself.
I haven't run the actual code, but from afar it looks good, for its style as much as its collaborativeness.
Will he be working from Germany?



DAN
-- I find his code to be quite readable.  Good variable names, methods, etc.  
-- His tests were quite lacking -- though this is a tough task to write tasks for.  
-- He does have academic points.  
   -- For example he got his git wedged by accidentally 
      committing 100+ megs of binaries into it.  He knew he needed to undo the commit, 
      but did not know how.  Professional coders would have stack over flowed that question
      in less time than it took me to write this sentence.
   -- He used recursion on his flood fill.  that is a bit cute, but one generally assumes that heap
      space is much less constrainted than stack space, so better to use a data strcuture rather than recursion to manage this front.
   -- On the plus side I feel his academic bent comes out in good ways here too.  He was creative and "big picture thinking"
      about how he approached the problem.  That feels exactly right for where we are in our project.  super useful.

-- Also I have a several hour brainstorming session with him, and I was *REALLY* impressed.  He was able to keep up,
   as we were discussing all kinds of approaches to constructing 2D embeddings in 3-space.  We dug deep and were moving 
   fast.  He was able to visualize in his mind the consequences of detailed algorithmic choices and talk on the fly about them!
   SWEET!  He also was creative.  Several times we found road blocks, and both he and I were tossing out ways forward
   fast and furious.  He could understand what I was spewing, and that which he was spewing was quite clear and logical.

OVERALL:  
For me, he is a no-brainer choice.  We are bringing him in as a consultant, and in an area were we are currently short-handed.
But even if all that were not true, I would still be inclined to bring him in.  I think he can think creatively, and he can slam code.
On that basis I am sold.



========
Group Summary:

It seems to me that as a group we saw point things about Severin that could be better, but overall the consesnus seems to be 
that he is basically solid.  

 




## Monica-Greg

DAN-O

Monika's code is pretty clean and straightfoward.  Her models are clean.
Her use of the Sequel package is far cleaner than more than half the submissions that rely direcly on SQL.
She was careful with details like lower-casing identifiers, though one might have dried that out rather than 
having many downcase everywhere in the code....  mostly that is personal taste, since it is one method call.

Monica mentioned not being able too certain about writing-skills so perhaps that is why she did not complete
the scaling docuemnt, or perhaps it was time.  Either way, this is a weak point relative to other submissions,
but not a drastic one.

Overall a straightforward a pretty clean submission.  Above the mid-point with respect to our candidates to date.


SENAD
Hi - here are my thoughts:

Monika -

Really clean code and tests - I assume scaling was not part of requirements because she did not write anything there.
I love the usage of newer ruby features. 
What could be better is error handling - I don't like the fact that service responds with http code 500 on nonexisting group
( and it should be eg. 404 if resource is not found). 
In the UI - submit with onclick handler was used to trigger the actions. In the handler ajax is used to send a request to server 
and form is submitted for  a reason not obvious to me (probably to refresh the UI). 
This causes a race condition - and if the server is too slow to respond UI is refreshed before operation finishes and ajax sucess
handler is never run. This bug is present in all actions and it looks to me like UI design flaw. 

Greg - 

Errors are handled better than in Monika's case. Ui is more complex than Monika's but prevents some mistakes like sending empty requests etc. 
Scalability discussion is adequate. I don't like the fact that all code (models, migrations, and http action handlers) is in one file (regardless of simplicity).
While UI does not have a problem in the sense of bugs - it's very confusing and it took me a while to figure out what's what. 
Naming is not good in some cases (eg. I don't think having Obj class is a good name). 

## Sean-Nick  -- PM

-- Sean
- good costings...  a bit lean on required hours
- loved his "roll up the sleeves" emails    (directly from CEO so all hands on deck)
- did not get a good list of backlog entries!!
  (but maybe he did not hear that request so clearly)

-- Nick
SOW:  does not list what we should do in a good way:
	•	Build, host, and maintain my.bpsox.com, api.bpsox.com, and ftp-api.bpsox.com

	•	Define clear measurable milestones, and the timeframe in which they are to be accomplished  
            !?

>>>  in week 1?!
  Implement new features

Nice idea:
The only thing I need is someone who knows how this system works.  Trying to migrate a system without visibility is going to present myriad challenges.  If you could put some resources into hiring an ex employee from Unreliable Corp, even as a consultant for a couple of hours would be invaluable assistance.

>>>  Basically we want a new app out by end of day. 
not realistic.  just get the data.  test that you can correctly connect to a sox
then get an app up within a week or so that connects. w. them

>>> backlog is way too thin to be useful.


--------
SENAD
Sean - 

Since it is not a primarly coding position - I would say the technical knowledge is adequate for the position. Scaling is with less details than Greg's. 

Nicholas -

 Usinq raw SQL is puzzling for me.  All code is again in one place - now with queries too. 
in scaling.txt there  is a very brave assumption about development time required for desired throughput. 
It's hard to judge with limited information but being that certain could be Dunning-Kruger effect at work. 




## Eric-Rafael -- ASSESSEMENTS FOR FULL STACK DEVS 
-- Eric 


== PIOTR ==


I finally looked at the two repos and both look reasonable. From what
I saw liked Eric’s better (better code compartmentalisation, like
not dumping controllers on the top level, and slightly better –
from my point of view – library choices, like data_mapper over
active_record), but neither was unacceptable or problematic.




======== DAN ========
BOTH
-- Both candidates took ~13 hours to complete.  
-- Both seemed competent, articulate in their interactions via phone.  
-- Neither candidate realized that the project really
   only requires two tables, they both just attacked it by creating
   a table for each type of object and then managing the relaionships
-- both cadidates apps were functional, but both had real usabilty issues
   they needed page refresh in order to recompute the data...
   its seemd they were more backend kind of guys, since it is pretty trival
   to have a static js that itself is dynamic


RAFAEL -- 12.5hrs total
-- was messier on delivery.  emotionally I felt, he felt more at the edge of his abilities in taking the test.
-- Rafael's use of ruby and particularly the ORM
   seemed like someone who has done abunch of it.  
   I found his models to be pretty readable.
-- Rafael's scaling document is less than I would want.  but at least he has
   some ideas here.  (in his signature style notice his is modest
   in his way of presenting his lack of depth here.)

   (for me I am looking for someone to get smart about the fact that we have
    database calls in the middle of these low level permission checks
    and if you want to dramatically speed these you need to move to a
    in memory solution.  icing on the cake to to hear ideas about how
    to cache computed permissions egerly to derivations are fast)
-- I like his quick but effective tests script  (effective for the authoring time used)


ERIC -- 13hrs total
-- seemed like this was pretty easy, and he expected it to be easy.
   he was more careful in the details of his delivery.
-- eric is doing things in a pretty low level kind of way.
-- I really liked eric's summary of his routes.  
   very clean and the routes seem reasonable too.
-- Eric's scaling doc amounts to saying.  we will look for things that take a
   long time, and speed them up!    uh, ok, right.....
   he has some specific speedups that he would consider, but I feel they are
   taken from a google search.  given the specfics of this task they 
   are the second order optomizations I would consider.
-- we told eric not to use gems, but he used data mapper at the DB level
   the result is quite nice, but makes it hard to compare.  also 
   if he were not able to use his favorite tool, I wonder if he would 
   have troubles.  (often in projects this choice has alredy been made for us.)






DETAILS OF THE TESTS

09-06 Rafael        10:20-11:00--7:09 = 8hrs  8:30-9:06a--1pm = 4.5h  ~12.5h
  
09-08 Eric Hubard?  8:30-8:45  3:34 == 7hrs    1-1:30 7:35  == 6hrs   ~13hrs
- was told to pull/push to the repo.   but did not do this.
- was told to commit frequently, but was only doing this locally.
- we had real trouble getting started.  lost connectiong via phone. 

>> Careful directions except does not tell me how to launch viewer! 


CONCLUSION

-- Both candidates would be fine in more junior roles within the team.
-- Both seem to have acceptable velocity.  Raphaels code is cleaner.
-- I don't know that either one would be appropriate operating on a solo project.






==SENAD==
Well they have much less experience (measured in years of professional work) than Piotr and me. 
So it's hard to compare to our existing team and I don't know them very well. Both of them could probably fit in nicely. 
I would say Rafael is more of my type in terms of not being specialized while Eric stick with fewer technologies and tries to excel in them - understandable 
since he comes from non CS background. 

They are above average when compared to all the developers I worked with or know personally of their professional age. 
Velocity - well if they took each 15 hours and finished everything they needed to (as far as functionality goes), they both wrote the tests so - their velocity is ok. 

I have no way of quickly judging accuracy and self-directedness as I did not work with them. 

---
Ok here is my opinion about the candidates:

Eric has a better organized code both on server side and on client side and clearly has more experience with scaling and better codility test scores.

Rafael mentions golang in the scaling document - his style is worse - but that could mean he just spent using other languages more (I checked his cv - yes he did a lot of different projects)
However "app engine has automatic scaling" is not a good approach to scaling. 

So if I had to chose - for this position (full stack ruby developer), it would be Eric. 




========= NICK ========

Rafael
Back-end
I liked that he included run_test_scenarios, which matched the exact example from the spec.
It felt like it was written in the correct "style" by someone who is familiar w/ Ruby
(especially compared to Eric's)
It is organized differently than a Rails app. I don't know enough about Sinatra to know if it correctly matches Sinatra convention, or if it is irregular in some way.
There were a couple funny things (eg: why is users#user_id a string? It also looks like the data model could have been crisper.) but nothing that jumped out to me as "wrong"
Ignoring whether or not it is "Ruby-ish", I felt like it was "simple" in a good way. I liked this submission.
Scaling doc
His scaling doc was very weak.
The English was ok, but he obviously has no practical experience scaling an app.
Front-end
His app needs to be refreshed after pushing the buttons. This is super annoying.
It doesn't seem to exactly match the spec.
The JS seems to be a bit busy for what it does.

Eric
Back-end
Eric was clearly unsure of himself writing a Sinatra app (there were links to how-to's in the comments, he defined his own version of #blank?, there seemed to be a lot of language agnostic imperative programming)
I did not like the way this app felt. Either at the macro level, or at the micro level of individual blocks of code. It's hard to put my finger on it. I just don't like it.
Scaling doc
Eric's scaling doc was weak. It was more practical & more thought out than Rafael's, but it was still not what I'd hope to see from someone about to be tasked to scale something in real life.
Front-end
It seems to do what it's supposed to.
It does not seem to exactly match the spec.
It's hard to tell how "good" the front is since it's using a giant tool (angular) to do a very small task.
Conclusion

Focusing only on this specific programming test:
I liked Rafael's back-end code. I feel like I would be ok reading his code every day. That being said, his scaling doc was very weak & his UX app didn't 100% match the spec. He will need probably need coaching on how to deal with bigger high-traffic applications.
I didn't like reading Eric's code. I feel like I would not want to read his code every day. His scaling doc was better than Rafael's, but not so great that he will not need coaching in this area.
Take care
