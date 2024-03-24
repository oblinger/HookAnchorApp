- [[Beta prod push]] 
! Upgrading our monitoring and testing


- [Prod Push](https://docs.google.com/document/d/1hXXBXI0PhSXIwLEsYcTKQPHBeIiNsvYW8zNuONveR40/edit#)  [testers sheet](https://drive.google.com/drive/u/4/folders/1Hapg-3w3fG8S-ny3hIAct-zPlGxQywL5)  
- 
- [Regular Reporting](https://docs.google.com/document/d/1N8AbXgFkXNoqDmYx77tSH_Lxp6JQf4palKKXdUkoTFQ/edit) 
- [Pillars](https://docs.google.com/document/d/1bhmtxJnbB7gPOGnJqV4ibzzQJaUNT5UimnAs9PTli-w/edit#) 
- 


### Scott Management

Brian,



### 1
! Build consensus proposal doc this week with one on one discussions with many stake holder; wider meeting next week


- [ ] [[2023 Notes]]: Contact info(pages, email)  info page for all folks.

- [ ] Call Trees who gets
      what to do with system is down





KEY PLANS AND ACTIONS

REVERSION - Plan for 

DEV ON CALL - 

CALL TREE - 

CONTACTABLE HOURS - 

BACKUPS - Separate thread to be addressed subsequently


### 2 



SUMMARY
- TRACKING
  - Joachim will track QA ticket tests and outcomes.
  - Lichty Lines - Simple weekly template that summarizes user outcomes and testing
- BETA TESTING
	- Visual Indicator 






SMALLER IDEAS
- VERSION CONFUSION - With test flight it is easy to be mislead about which version is on your device so we introduce a configurable colored border on all test flight versions to distinguish which app is currently active.
- VERSION CHECKING - Backend publishes a version number that is checked for compatibility to it is not possible to mismatch front and backends.


- Joachim tracking: ticket tests




APPROACH
- ALPHA TESTING - Joachim
- STAGE TESTING - Driven by Mark



- Copy versions of production down onto QA and STAGE environments.

Version Management






Joachim, 
I would like to start tracking how we are doing in catching and processing bugs.  my thinking is pretty basic, and I am hoping that 



There are lots of good ideas about automated testing and such, but right now it seems we are just not really good old fashion pounding on the 


I think we need (and can) get to a place where egregious regression errors on production are an extremely rare event.Flix

My take is that we should be able to (and NEED to be able to) get the number of egregious regression bugs on production down very near zero.  As we all know, this is not rocket science, we just need to be systematic about our testing 


The fact that we are still regularly finding REGRESSION bugs in production is clear indicator that we need to beef up our testing pipeline.  As a first step in this direction we propose t



TOTALS Stats are broken out by week    (or maybe by sprint)

- Number of QA tests performed.
- Number of QA tests pushed back.

Staging features pushed to production
- Staging bugs identified
- Release candidate bugs found
- Alpha testing bugs found
- Push to production bugs found
- Bugs found on production
- REGRESSION bugs found on production
- OTHER bugs found