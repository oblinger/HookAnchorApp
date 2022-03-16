# AR.3c18\_Tivoli --

    Attendees:
    Patrick Guido/Raleigh/IBM@IBMUS, 
    Robert Uthe/Raleigh/IBM@IBMUS, 
    Tushar Chandra/Watson/IBM@IBMUS, 
    Keia Parker/Raleigh/IBM@IBMUS, 
    Robin Fredrickson/San Jose/IBM@IBMUS, 
    Tiffanie Lee/Durham/IBM@IBMUS, 
    Wynn Price/San Jose/IBM@IBMUS


    Dan's notes:
    - Products ship docs
    - Versioning 
    Pat Guido (second line)
    - schedule 
    Research Challenge
    The issue of upgrading procedures


    Tushar Chandra - lead architect & research relationship mgr for Tivoli
    Robert Uthe - lead UI person in Tivoli
    Pat Guido - 2nd line dev mgr under Peter Spung, responsible for interaction design
    ... and a few other people whose names I didn't catch

    [Dan gave the spiel]

    Robert: could I run this headless?
    Robert: this could be used to detect errors and figure out what to do if one pops up
    Robert: is it Windows only?

    Robert: how does it compare to automated testing?
    Dan's answer: conditionals
    Larry's answer: variability

    Dan: way we fit into Tivoli is that we evolve procedure over time

    Robert: what about the web?  SVG, flash, applets?
    Vit: if it's not MSAA, no go
    Robert: we have lots of UIs deployed on the web
    Dan: we should find the best point at which to apply instrumentation
    Robert: we'd like to see how your stuff does with a simple test case, as a proof of concept
    Dan: we have another project on web browser instrumentation
    Robert: heavyweight web applications are difficult to deploy; for example Java Web Start doesn't always work
    Robert: we have a reusable widget library (mostly JavaScript) that we use for all our UIs and is used by many other UIs within IBM
    Robert: we're moving to the Integrated Solutions Console
    Tessa: we could insert instrumentation into your reusable widget library

    Robert: have we considered WBI, which models business processes and user activities (which are essentially traces of users completing those business processes)
    Dan: yes but our initial platform decision was to do Windows, and WBI was on web so it's not applicable

    Tushar: we should talk to Dan Sturman, relationship manager for WebSphere, and show our current demo
    Tushar: we should talk to Jeff Clark, in PCD & ThinkVantage; our stuff could be used to automate common PC tasks.  Actually the person we should talk to is Steve Welch @ Almaden, and say that Tushar sent us

    Robert: I could see this being used as a teaching tool that ships with each product.  But for IT style procedures, versioning is a big issue.  If our customers develop an enormous corpus of procedures that work on version 2.x of our software, we don't want them to have to throw them away when version 3.x is released.
