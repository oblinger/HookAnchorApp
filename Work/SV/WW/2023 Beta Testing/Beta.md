

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