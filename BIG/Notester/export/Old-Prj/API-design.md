# Old-Prj.API-design --

    Transport 
    - Handshake is done w. vector of strings 
     
     
    >>>API 
      VARS: 
        SRC       -- Source dataset (set of people) 
        DST       -- Destination dataset (set of people) 
        ID        -- Unique employee identifier 
        NAMESPEC  --  
        LocSpec   -- Country, State, City, Building, Floor, Room, Radius? 
     
     
     
    SelectByFirstName(dst, src, NameSpec) 
    SelectByLastName(dst, src, NameSpec) 
    SelectByField(dst, src, field, value) 
     
     
      bits 1=polarity, 2=threshold, 3=InheritFromSubFields 
    SelectByAffinity(dst, src, id, radius, strength) 
    SelectByLocation(dst, src, LocSpec) 
      Sorted by distance 
    SelectBy 
     
     
    Constraints 
    - Partial first name  (w. astrix) 
    - Partial last name (w. astrix) 
    - Partial name spec. 
    - Org constraint 
      Specify super mgr 
    - Personal affinity (specify person and radius) 
    - Skill constraint 
    -  
     
     
     
    SendThis 
    SendAndGet 
    Get 
    anythingthere 
     
     
     
     
     
     
    LIST        
     
    start app 
     
    p>s  BUILD-LIST  "rob*"  "f*"  "YKT" 
     
    s>p  LIST DISPLAY-REC "Here are the robs" "..."  "rob1" 77 "rob smith" 88 
     
    p>s  GET-REC 77 
     
    s>p  PRINT-USER "sdf" "Sdf" "erter" "sdffs" 
     
     
     
    char**  send(char* buffer, int len, char servletName, char** message) 
     
     
    char*[10] char[50] char [10] 
     
     
    int send(char* buffer, int len, char* name, char* message) 
     
