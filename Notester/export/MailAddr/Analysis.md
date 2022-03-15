# MailAddr.Analysis -- Analysis of alternatives

    GOAL: Assist a mail sender is specifying an email's destination address. 
     
    Possible sources of information: 
    - ALL EMAIL: The universe of all mail traffic (with in IBM). 
    - SENDER EMAIL: Subset of universe sent<<>>received by the sender. 
    - ADDRESS SYNTAX: Syntactic analysis of sender's email address. 
    - CURRENT MESSAGE: Message being viewed by the sender. 
     
    APPROACHES 
    ---------- 
     
    FIXED HEURISTIC 
    - Partition the universe of email addresses into five categories: 
      1) Addresses in the senders address book 
      2) Those the sender has sent mail to 
      3) Those the sender has received mail from 
      4) Those at the same location as the sender (addresses have the same suffix) 
      5) Everyone else 
    - Choose the addresses from the lower numbered categories first 
    - Advantage: Uses only local information (no security<<>>synchronization issues) 
     
     
    PROBABLE CONTACT 
    - Compute a link graph based on email traffic probability densities. 
      (One possible definition listed below) 
    - Order all receivers based on their "distance" from the sender 
    - Warning: The full all pairs shortest path algorithm is n^3 which is  
      unacceptable for 100K to 1Meg address space. 
      (I think limited depth versions of this alg will be linear however) 
     
     
    TYPES OF PEOPLE & TYPES OF SENDERS 
    - Assumptions: There exists an unknown flat partitioning of people into "types" 
                   Each sender sends to an unknown <<>> of people types. 
    - Algorithm One: 
      1) Cluster people into a set of types (e.g. K-means or baysian clustering) 
         use the number of common contacts (see below) as your distance metric 
      2) For each person order the types by propensity to send email to that  
         type and use this to order possible addresses. 
     
    - Algorithm Two: (allow people to be of multiple "types") 
      1) Greedily grow set of overlapping clusters of people using the "number 
         of common contacts" distance metric.  (similar to AQ-11 star building) 
      2) Cover each senders contacts with a set of overlapping types 
     
      Optional optimization: 
      3) Keep the senders contact types fixed and regrow the overlapping  
         clusters of people types in order to best explain senders contacts. 
     
      4) Repeat steps 2&3 until convergence 
     
     
     
    ------------------ 
    DETAILS: Just some definitions to make the notion of links between people  
             and email contact precise. 
     
     
    Psend(i,j)     PROBABILITY OF TRANSMISSION (amount of traffic from i to j) 
                   This is the probability that randomly chosen doc from ALL 
                   is a message from person i to person j. 
     
    Pcontact(i,j)  PROBABILITY OF CONTACT 
                   = Psend(i,j) + Psend(j,i) 
     
    Pout(i)        PROBABILITY OF OUTPUT (amount of traffic from person i) 
                   = Sum Psend(i,*)   where * matches all people 
     
    Pin(i)         PROBABILITY OF INPUT  (ammount of traffic in to person i) 
                   = Sum Psend(*,i)   where * matches all people 
     
    PRsend(i,j)    RELATIVE PROBABILITY OF TRANSMISSION 
                   = Psend(i,j) <<>> Pout(i) 
                   Assuming that doc, D, is send to someone, this is the  
                   probability that it is send to j. 
     
    PRcontact(i,j) RELATIVE PROBABILITY OF CONTACT 
                   = PRcontact(i,j) <<>> ( Pout(i)+Pout(j)+Pin(i)+P(j) ) 
                   This is the probability that some email connected to either 
                   i or j was an email that linked the two. 
     
    link(i,j)      DEGREE OF LINKING 
                   = Log( link(i,j) ) 
                   This is a number that represent how often i,j communicate 
                   with each other normalized by how often they communicate with 
                   anyone.  (The log is taken so that adding link scores will 
                   coorespond to combining independant probabilities) 
