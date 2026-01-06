.[[ABio]].  [[@Github]] [ABIO Folder](spot://ABIO~Folder) 
   [[ABIO Sys|SYS]]:	[[ABIO Roadmap|Roadmap]], [[ABIO Files|Files]], [[ABIO Protocols|Protocols]], [[ABIO Topics|Topics]], [[ABIO Notes|Notes]], 
   TOP:	[[ABIO.]]  [[ABIO Log|Log]], [[AlienBiologyWhitepaper (do not distribute)|Whitepaper]], [[ABIO Results|Results]], [[ABIO Safety]], 
   PARTS: [[ABIO infra|infra]], [[ABIO biology|biology]], [[ABIO execution|execution]]
   EXP:	[[ABIO Exp]], 

   EG:		[[ABIO Photosynthesis]],  [[ABIO CellMetabolism5]],
   REF:		[[ABIO References]], 
   WP:		[[ABIO Behavior Templates]], [[ABIO Generator]]
   LISTS:	[[ABIO Prompts]], [[ABIO Queries]], [[ABIO Tasks]],
   [[ABIO PRD|PRD]]:	[[ABIO Notes]],
   [[ABIO Legacy]]:	[Browser](hook://abiosys), 
   TEST:		[[test-entity]], [[test-molecule]], [[test-reaction]], 





xxxx


In components, you list membrane flows, but I think that these flows are gonna be more general than just membrane flows. They really can be flows the change the multiplicity of different items and they can adjust notches across that membrane but it can adjust across the structure. I think we should probably just call them flows.


I think we need to update the documentation that goes with full name to be sure to have it defined even for entities which are not in the componentry, but which are in the dictionary of attributes associated with an entity. My thinking is that we have a nice way of naming entities that are in the component tree, but we really want a global way of naming entitiesin general I think because the dictionary is key value it gives you the equivalent of a local name for each one of the children in the dictionary just like you have local names in the componentry. But what I like about the component tree is it gets its name from these prefixes and we have a way of generating short names that are still unique that we can still get back to. I want to retain this capability. Alternatively, we could continue using a local name for everyone of these entities, including the entities which are down inside of the slots are the attributes of an object, but as I think about it, those are not actually they won't have parents and children. They may not even be entities. I'm not even sure if Adams and molecules are entities. Would you consider this? What are our alternatives here? My goal is that we end up with automatically end up with unique namings for all of these simulated worlds for all the stuff. And that they're not too long at least we can use prefixes to shorten them somewhat. Let's think about alternatives for this.


> I noticed that in the import statements for alien biology, you are indicating that the implementations are also exported invisible. I was thinking that
  those would actually not be visible and that we would be creating a factory that allows us to create these various types of things attached to the
  context, or maybe attached to expression but actually I think it's probably or maybe attached entity but I think that's not right I think there's
  probably a factory maybe a factory that maps heads onto instances. And we have a and then we have entities actually implement the EXPR protocolso that
  they can be instancy from EXPR items or strings and also structures. This would unify the way that we capture information from Yael and Jason Anand
  structures for both expressions and Four entity hierarchies. What's you're thinking about this?

The system should have earth atoms as a list that somehow accessible. Maybe it's a function which you can call that does a lazy computation of this list and then always returns it. I'm not sure but I think in in the beginning when we have a chemistry we're gonna wanna be able to refer to earth Adams as a si starting point for a lot of chemistries only later will we actually create synthetic atomic elements?

