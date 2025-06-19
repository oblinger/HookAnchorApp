
[[KM Anchor Implementation]] 

- I need a better name for this system!  Its core construct could be called a jump page, a group, a bucket, an anchor, a project, a hook.  Not sure of the best metaphor.


# The Hook-Anchor System

**HOOK** — A ***hook*** is a short, unique string a user uses to refer to a digital thing.

**ANCHOR** -- An ***anchor*** is a hook-named container of digital things.







# === OLDER TEXT ===
# The Hook Set System   (I need a better name!)


Personally managed knowledge is usually (1) captured, (2) retrieved, and (3) used within a specific contexts.  
There are large benefits if that knowledge is organized appropriately for the contexts in which is it captured, retrieved and used.
Ensuring these contexts remain appropriately organized over time is the heart and value of the hook system.

OVERVIEW
- The core of this system is the "Hook Set" -- a series of usage contexts (called "hooks") into which information is organized.
- At any given time up to 100 or so hook contexts will be active, gradually they fade to become passive and eventually archived.
- Hooks are organized in a continuously evolving two-level index, one global index over active contexts and a second level index for each hook.  
- The approach has several key benefits:
	- One always knows where to find a given piece of knowledge
	- Access to any given piece of knowledge can be done in at most two steps.
	- And refactoring the index can be done incrementally and rapidly since each individual index is small.
- There are several important "Exception" categories of info that is better handled using specialized patterns:
	- ENTITIES - Knowledge about Entities like people or corporations (anything with a unique proper name) is handled specially.
	- LISTS - 
	- LOGS - 
	- MEETINGS - Any information captured during synchronous communication (phone, chat, zoom, etc.) with entities.

## The Hook Set System


### Partitioning all activity into usage contexts (called "hooks")

According to this system all activity is broken into a distinct set of distinct knowledge usage contexts (called "Hooks"):
- Each context is associated with a distinct topic, area or task that occupies notable effort over weeks to years.  The quintessential context is a project that one is involved in over some weeks months or decades.
- The set set of active usage contexts (Hooks) will evolve over time with older ones being archived but still available.
- Something like 100 hooks should be active at any given time.  This is enough that is breaks all activity into manageably sized chunks while still keeping the number of hooks small enough that one can easily remember the meaning of all of them without trying.
- Each hook has a multi-word naming phrase that is descriptive enough to be remembered decades later, 
  and a short single word or abbreviation that is ideally 2 to 6 letters long.  (this is for quick access and because it is used as a prefix.)

The key value of the hook system is that it PARTITIONS all knowledge uniquely onto a SINGLE hook.  Most of the time this is simply the hook associated with the usage context active at the time the knowledge is captured, but there are exceptions to this generality.  The main requirement is that one can instantly know which hook is relevant, and one can remember years later which hook was used.  There are some tricks to making this work in practice discussed below; in general with a bit of up-front thinking this turns out to be much easier than it sounds.

The key idea is to keep these context hooks broad enough and simple enough that one can always know immediately which context a give piece of knowledge belongs in.  Usually it will simply associated with a single hook based on the relevant activity that the time the knowledge was captured.


### The contents of a single "hook"

Each usage context (hook) can now be organized appropriately for the usage context w/o any consideration for how it might relate to other hooks.  This makes its organization trivial to maintain the internal organization of the hook over time as the usage context evolves.

- Each hook page must have a table of contents document (sometimes called a jump page) that helps to organize info in this hook.
- Sometimes a dense block of links with 5-10 links per row, and 10-20 rows total allow one to see ~100 links as a group without scrolling.


# Exceptions





# ANCHORS

tl;dr.  Maintain a thousand or so anchor terms as sets, streams, rocks, or topics in order to directly jump to any info in your personal knowledge store.


PKM Anchors -- Sets, Streams, Rocks, and Topics

A PKM should be an evolving conversation between your mind and and and associated digital store.  In order for this conversation to be productive it is important that the terms you use to index your mind and your digital store stay in sync.  This ensures you can merely think of a thing and already how it is organized into your PKM.  Over time the topics indexed evolve, thus the terms must also evolve, but in order to be remembered, it is critical that we manage this evolution so it is not too quick, and so that the association of information to terms is remembered indefinitely.


ANCHOR TERM -- An anchor term is a short, memorable alpha numeric phrase with an understood boundary specifying which pieces of information are and are not associated with that anchor.

Many will push back, "oh my thinking it too fluid in order to put into boxes like that!"  Its true, the web of links between our thoughts is quite complex and does not fit into boxes.  Still with care we, create mental rules that provide a simple in/out designation for any piece of info.  Then we need to ensure these rules can be maintained in a consistent way indefinitely, and that we can consistently arbitrate when multiple anchors apply to a single piece of info.  If this all sounds like a very tall order, it is, but it is also quite possible, and once you have done this, managing a sprawling PKM becomes a pieces of cake, even as it grows over decades of continuous aggregation!  Here is the structure that I have evolved over the last 20+ years of usage.  There are four types of anchors:

- SETS -- These anchors manage collections of some common underlying type:  e.g. all people, or memorable quotes.
- STREAMS -- These are sets that are further organized by time into a stream of items
- ROCKS -- These are larger, time-bounded, topics or activities.  E.g. "the zzz proposal"
- TOPICS -- These are timeless (evergreen) notes around which we have built a taxonomy of info for that topic.  e.g. finance.

The arbitration rule is to always put items into SETS before STREAMS before ROCKS before TOPICS.  Important information can subsequently be put into other buckets, but it is critical that it is always associated with the earliest anchor that way it will be there when retrieval is attempted.

SETS are easy.  We know what the type of info is, so it is completely obvious if a piece of info fits into a set based on what it is.  Is it a person, well then it goes into people.  Is it a URL reference then it goes into references.  But often we want to indicate the type of a thing, but then ALSO organize that info according to time or topic as well.  As described below, this system allows us to do both of those things at the same time, since it is such a common thing to do.

STREAMS are easy.  These are just sets whose items are inherently organized by time.  For example meeting notes are indexed by the date when the meeting occurred, and url references can be organized by the date that one collected it.  Adding the time dimension is quite critical for large sets.  Over years a PKM can become a cesspool of complexity without the use of steams since each category grows so big nothing can be found.  But often one can limit ones looking to an approximate month or year, which keeps the found information constant, regardless of how long the PKM has been growing.

ROCKS are easy.  These are streams whose elements each have an anchoring name that is relevant for some period of many months.  This is large enough and slow enough that one can remember the anchor name for the period that it is active, and when scrolling thru a list of all rock anchors with one-line descriptions one can remember the meaning of the rock anchor even a decade or two later.  A rock might be the final essay for a class, a job search you did in 2018, 

TOPICS are easy.  These are just rocks with no expiration date.  They are an area of information are evergreen, they will remain relevant, and meaningfully-named forever.  For example, MED (Medical/Health related info) and FIN (Finance related info) are two such anchors for me.  Since these anchor pages will be viewed many many times, it makes sense that we organize them well.  Think of each these as a landing page or a routing page -- they are organized in order to point the reader in the correct direction for ALL info that falls under the purview of this topic.


## USAGE

So how is all of this used?  Generally 


TOPICS
- A small two-level outline (which fits on one screen!) can be crafted used if there are many sub- or related- topics for this one.
- Streaming info related to this topic is attached directly to this page in a LOG section at the bottom.
- Very cohesive topics or documents can use the folding-text feature to directly embed a significant amount of info into a nested outline of upto three or four levels deep.


