# 999KeySearch --

     <<>>     
     <<>>     
     
     
    IDEA: Distributed maintaince of an intranet wide namespace for all 
          IBM web resources. 
     
     <<>>   
    PROBLEM: It is difficult to locate known resources on the web. 
      A great majority of seraches for resources on the IBM  
      intranet are done by a searcher that already knows a name of the 
      resource they are looking for, they just don't know where it is. 
      Distributed namespace maps these names to the most appropriate 
      URL for the given user. 
     
    KEY DETAILS 
    * At the core the system is a mapping from unique name X locations to URLs 
    * System "sits behind" conventional search interface 
    * Data obtained from: tags on web-pages&notes DBs, exports from special DBs 
    * System does not place constraints on content, representation, or location  
    * Administrative control over the creation and maintaince of 'unames' should 
      be similar to current control over web pages posted within the intranet. 
      (No additional beuacratic constraints) 
     
    UNAME 
    A uname is a multi-word term  that names a resource within the intranet 
     
    RREFs 
    An Rref identifies a specific resource with the intranet 
    This entity has the following properties 
    - It has a multi-word character string name. 
    - It has an orgaizational scope within IBM (Point in the blue pages callup tree) 
    - It has a temporal scope (time range where it is valid) 
    - It has an author 
    - It has a priority 
    - It may specify how it is combined with other RREFs 
     
     
    BENEFITS 
    * Searcher does not need to know data organized within IBM 
    *** Searcher does not even need to know what database the data is 
        indexed in.  They do not need to understand the information architecture  
        of IBM. 
    * The way info is retrieved will not change over time, even 
      if its location or the strcuture of the index changes 
    -> This means that documentation can reliably refer to other resources 
       in a way that does not break over time 
    * Search only returns locally relevant information. 
     
    ?? Data does not need to expressed redundantly in order ensure the 
      data can be found. 
     
    COSTS 
    * Development time (minimal) 
    * Computational resources (equivelant to serving web pages for IBM) 
    * Data entry time.  Large, but it is a small increment over time 
      currently spent on generating content. 
