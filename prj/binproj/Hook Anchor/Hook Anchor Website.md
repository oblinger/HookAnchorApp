
prefix menu should disappear once all items and files have been filtered out
files should be filtered along with commands in prefix menu

# PAGE INTRO

Context:  I have written, used, and suffered with at least five personal knowledge systems over the decades.  In the last 5 years, I have landed on three principles that I think are essential for best-in-class knowledge management.  I describe them here, and they are the center of a Mac App I have written that helps implement these principles.  Still, the principles are more general, as I had been following them for years w/o the aid of this app.


Hook Anchor Stream is an approach for knowledge management that works with your existing tools, adding a layer that radically improves three perennial issues plaguing them: how to ***find*** your stuff, how to ***place*** new stuff, and how to keep everything ***up to date***.  Here are the key ideas:


**HOOKS -- 80/20 NAMING**
95% of the time, you are accessing the same 5% of your info over and over, but that active 5% is gradually shifting over time.  
The trick is to maintain a set of "hook" words and continuously adjust them as you are using them.
Why should YOU need to remember and search through tens or hundreds of thousands of files, websites, etc.?
Your knowledge manager should be remembering YOUR active names.
Accessing 95% of your stuff becomes muscle memory since the first name or acronym that comes to YOUR mind is always the right one.  
The result is deliciously instant, thought-free access 95% of the time.

So what about the other 5% of the time?  Hook Anchor upgrades how you group stuff, AND how you find those groups.


**ANCHORS -- LOGICAL GROUPINGS**
You already organize using project and topic folders. Hook Anchor upgrades this habit in two ways:
- First, it makes those groups *logical* so they can span multiple applications/accounts, both online/offline, into a single logical anchor.
- Second, it extends URLs on your machine so you can build unified anchor pages that directly link to external pages in Google, Notion, Confluence, etc.  (using Obsidian Markdown or any locally edited content).

The key for a good anchor is simple: at least for a bounded period of time it is immediately obvious whether an item should or should not be grouped under the anchor.  This makes both storing and retrieving immediate and reliable.  (Over extended periods of time, often new anchors are be needed as a topic subtly shifts)


**STREAMS -- UNIVERSAL AXES OF ORGANIZATION**
Personal knowledge is too fluid for any fixed structure to stand the test of time.  Worse yet, as this knowledge grows, reorganization costs become overwhelming.
The solution is to avoid playing the game entirely: Instead, organize knowledge *locally* by anchor, then organize anchors according to three *universal dimensions*: Time, Place, and Type.  Having search capability across these three axes provides enough structure to find past knowledge directly w/o backtracking.

*TIME* is obvious; all personal knowledge is known and captured as a specific moment in time.  Often, we will remember this time period even when we remember little else specific about the sought knowledge.  Thus, organizing all information by time is critical.

*PLACE* is tricker; with Hook Anchor we give up on the idea of globally consistent taxonomies or tags.  Over time these will change, and we cannot go back and recategorize according to the evolving structures.  Still, there is value a taxonomy and using it to organize our Anchors, we just have avoid using the finest details when searching over regions that have shifted during the time in question.  For example, it is still valuable to separate work from personal, or AI-papers from Economics-papers papers, even if the detailed sub-categorization around these items has shifted.  And building comprehensive taxonomies allows one to indicate many relational assertions by simply placing an anchor into a single place within that structure, so organizing by place can be instant and easy.

*TYPE* is trickiest of all; Some types, like Canva pages or Excel spreadsheets, are obviously different and distinct.  While a markdown page about a person and a markdown page about a paper are also distinct, but the space these logical types require a bit of thought.  Still, this can pay big dividends if we are consistent in the way we treat all papers and all people, then we access and organize along this dimension over extended periods of time without every requiring to refactor.

The key to using these typed ***streams*** is to identify repeatedly captured categories of information, and then to define a templated, standardized way to capture that information, such that it can be  consistently accessed and operated on in the future.  (In the practical application section we provide examples of good stream definitions.)

No matter what organizational system and tools you use, access and upkeep can be dramatically improved using Hooks, Anchors, and Streams.




# Practical Application

Here are some practical ways to dip your toes into the Hook Anchor system.  These notes are geared towards users of my HookAnchor Mac application, but they are also broadly relevant for implementing the method without the application.

## Phase One

At first, you can keep things very simple.  Just start adding words and acronyms that you frequently access as hooks.  These trigger words will open applications, folders, and documents across accounts, both locally and online.  Giving yourself direct access to these frequently used items provides an immediate boost and helps you get used to using the hook anchor as your first method to access content.

I love using the Karabiner macro so HookAnchor is triggered by pressing the Caps Lock key.  Using this macro, the key can still serve as Caps Lock as well as your option/command key if pressed in combination, so no functionality is lost.  And putting the trigger for HookAnchor helps with muscle memory for commonly accessed items.


## Phase Two

Often, you will be in the midst of a couple of activities, each with 5-20 frequently accessed items.  It becomes challenging to maintain unique names for all of these items.
The trick is to create an Anchor phrase for each of these groups and to set the "Anchor" check box on this command.  This will allow it to be used as the prefix for trigger phrases for each item within the group, and these phrases will all be grouped as a "prefix" menu within the app.

For quick access create a 2- to 3-letter acronym for it so that most menu items can be accessed in 4 keystrokes

Generally, we want to use a 2- to 3-word phrase for most anchors.  This allows us to use each word's first letters to create a quick access alias so most menu items can be accessed in under 4 keystrokes, and so we can still have a memorable name later when we "retire" the anchor from active usage.

This method allows us to have "muscle memory" access to 100+ anchors, each with 10+ items.  Instantly, thought-free access to your 1000 active items across all systems is a delicious feeling.  You will be hooked!


## Phase Three

The third phase focuses on global, long-term organization.  We know we cannot depend upon any deep, unchanging taxonomy, so what can we rely upon?

Unchanging CATEGORIES of knowledge.  We are free to define these categories (called streams) any way we like, but for a stream to be a good stream:
1. It should be instantly clear into which stream a given piece of information belongs, and
2. The way information is recorded into each stream type should follow the templated pattern for that stream.

This can be tricky to achieve since most information can be organized in multiple ways.  Still, with thought, we can often cleave off specific categories of information and decide categorically how we will handle that kind of information.  Each time we do this, we simplify both the task of adding knowledge as well as accessing it, but ONLY if we can create streams that really are distinct, sharply defined, and rarely ever change over time.



# Facets Primer

Hook Anchor provides a single, logical anchor for tying information across systems.  Often, this will result in repeatedly having the same kinds of items across many anchors.  For these cases, we adopt a naming convention called "Facets," these simplify hook name choice, keystrokes typed, and simplicity of recall.

A Facet is a hook name suffix that is used for a particular kind of information.  It's not guaranteed that every Anchor will have every type of facet, but when they do, they will always use the same facet suffix.  For all common facets, we choose suffix words that begin with a unique first character. This way, we can often select each facet by typing the anchor's acronym and one additional character to select the sought item uniquely.  Facets simplify the choice of hook name, the typing of the name and remembering its name.

For example, not all of my projects have a local folder, but if they do, the "xxx Folder" suffix is how I name it, "xxx Gdrive" indicates the root Gdrive folder associated with an anchor if it exists, and "xxx Notion" for a root Notion page.  This does not preclude creating a hook to name a specific Google Doc or Notion sub-page as well, still, these root pages are so common that it is nice to have standardized naming for them.

[[Facet]] 
Here are the facets that I personally use:


# Stream Primer

Not all of your information will fit into the streams categories that you create.  Still, it behooves you to think hard about a pragmatic set of streams you might use to manage most of your info in ways that will be trivially accessed later.  Streamed information is radically easier to find since you already know where and how to look for it, even years or decades later.  There is a lot to say about the design of good streams. For now, I will just list a number of my personal streams in order to give a flavor for how they organize one's information.

I use a year number prefixes on all project anchor names, along with a direct name/acroynm alias for quick access while proj is active.

- WW stream of work projects.  Each has a Notion anchor page with optional Gdrive folder and local file folder.
- SVtask stream is a stream of delegation projects.  These are organized around a Google doc in a Google folder since some members don't have access to Notion, and a GDrive folder is a natural for delivery of task results.
- PP stream of personal projects.  Each has a local markdown folder file
- binproj stream of smaller coding projects.  Specfically these result if scripts that I run personally 




PAPER STREAMS
Research information is notoriously difficult to categorize and stream.  Still, information associated with specific paper publications can be productively anchored to a page about the paper itself.  So, for me, the "Paper" stream is a sequence of markdown pages where each describes a single publication.  It's name is the title of the paper, it can be located within any topic relevant topic area.  In some cases they are grouped into an anchor associated with a particular investigation, or they might be under a general topic area.  Rarely I might add a tag or two to the paper.  The fact that I can reliably select ONLY paper pages and I can search by date, usually to find a reference paper even years later with nearly no search.


PERSONS
Information associated with a person is almost always associated with a project, a company, or other items.  Still, if it is related to a specific person, I almost always will know or can find the name of the person.  This makes people a fantastic stream.  Any time I have information that naturally associates with a single person, I will link it directly or indirectly to that person's "name" page.  A person's name page will have a hook that has "@" then their first and last name.  So my entry is "@Dan Oblinger".  Even if were to have a single conversation with a person that I may never meet again, if I record anything about that conversation I will create a new person entry.  It feels like overkill, but it is so helpful later, since you never really know what the future brings, and now the info is captured in a way it can be found even a decade later.


MEETING STREAMS
It took time for me to evolve this powerful stream.  At first I started organizing 


ENTITIES
Person entries are so useful, I have extended the person template to cover any entity that I might have a conversation with, so this would 






PROJECT STREAMS



LOG STREAMS




, each well formed stream will dramatically improve those classes of information.

Here are some example streams that I have evolved and I expect may 

Let me illustrate this using three concrete examples:

T


Once you are gaining solid value 
Start identifying and codifying 



# OLDER STUFF





BY STREAM




- Just deciding where to put some new item or project so you have any hope of finding it becomes painful.

The solution is to stop organizing globally by tree or tag and organize instead by time and stream.
- Like a trusty journal, you always know where to put the next bit of information; it goes on the next blank page, of course!

This system works well because it applies universally to all collected information, and often, we have a sense of the time period when each item was collected.

We can further simplify by having a couple of journals, each with its own "stream" of content.  But it is best to start with
We have to be very careful however to strongly limit the number of these "streams" because even small amounts of confusion about which stream a bit of information belongs in





## __ OLDER STUFF


LOCALLY ORGANIZED
Taxonomies or tagging systems often start strong, but as they grow, they become crazy hard to manage.  The solution is to STOP organizing globally and start organizing locally by anchor, by time, and by stream.  Together, these methods facilitate quick finding in the 5% of cases where you don't instantly remember an item's name.


Imagine an ideal personal organizational system.  What would it look like?

MULTI-TOOL, MULTI-LOCATION
First, it would span the full range of tools that you regularly use for work and home.  Its information would live wherever was most convenient; it would not impose structure on you; you would impose structure on it as you worked.


80/20 NAMING
A great assistant would keep track of the few hundred files and locations you are currently accessing and would be able to find them based on YOUR naming of them without requiring YOU to remember where they filed it.  The ideal organizational system should


LOCALITY 


*MULTI*  — It give you access to your info wherever you wanted it to be using which ever tool you wanted to use.

NAMING -- It would learn and maintain names the names and acryonyms you have update to match names in YOUR head, rather than YOU needing to remember 

ANCHOR -- 


### OLD "LAYER" Introduction

Hook Anchor adds a layer over your existing personal organization to address three key issues that plague these systems:
(1) Where to ***put*** your stuff without much thought
(2) How to quickly ***find*** your stuff without much thought
(3) How to keep it all ***up to date*** without much effort

