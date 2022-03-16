# 95-Assistant.Communities-alg -- Algorithms for finding mail communities

    1) Transform mail into a directed graph G whose node are the users  
       and whose edges are weighted by the number of emails between users. 
     
    2) Let O be the set of communities; initialize it to the singletons of G. 
     
    3) While O!=null or tired-of-waiting 
     
    3.1)   Choose c in O where Cohesion(c) >= Cohesion(c2) for all c2 in O 
     
    3.2)   For all u in G add  c + <<>> to O 
     
    3.3)   Add c to C; O = O \ C 
     
    4) Return subset of O+C that are useful communities 
     
     
     
     
