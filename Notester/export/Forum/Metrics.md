# Forum.Metrics --

    DEFINITIONS
    ===========


    FORUM TREE
         The forum tree represents the the emergent consnesus from a sequence
         of notes.  Formally it is a forest of nodes.

    NODE
         A node is a named entry in the forum tree.
         It contains all notes with the same name
         It may have a parent node.
         It may have a scope. (a namespace in which the node name is unique)

    ALTERNATIVES
         The alternatives for a node is the set of notes with the same name
         and scope as the node.

    PARENT ???
         The parent of a node (and all notes in the node) is the node 
         associated with the note linked by the nodes head note.

    RELATED
         A note is 'related' to node, n, if it is an alternate of that node, or a
         child of a related node.

    TRAFFIC
         The traffic on a node is the set notes related to the node.

    INHERITED TRAFFIC ????
         The traffic on a node unioned with the inherited traffic of all nodes
         with this node as their parent.

    ACTIVE TRAFFIC
         The active traffic on a node is the set traffic that has occurred with 
         in a particular time frame

    VOLUME
         Traffic per unit time over a given interval of time.

    PARTICIPANT
         An entity that submits a note.

    POPULATION
         The set of participants issuing a note related to a node.

    VOTING POPULATION
         The set of participants issuing a support or oppostion note related to
         a node.

    SUPPORT SET
         The set of participants issuing a supporting note for a note.

    LINK
         Each issued note is 'linked' to a previously issued note.

    HEAD NOTE
         A head note is a note that is <<>> an alternative or revision of 
         the note it links.
    OPPOSITION
         The set of participants issuing a opposing note for a note.
         (I don't think we are using opposition nodes)

    ==================
    ===  Notation  ===
    ==================

    NOTE FUNCTIONS
         
      U               A set of notes.  The universe of notes under consideration.
      P               The set of people who post messages
      name(n)         The name string for a note or node n.
      type(n)         The type of a note. 
                      One of: 'topic', 'position', 'support', or 'discussion'
      parent(n)       The parent note for a note n
      children(n)     The children of a note n
                      = <<>>
      positions(n)    Position notes on a topic node
                      = <<>>

      alternates(n)   The notes that are alternatives for a tree node.
                      = <<>>

      voters(n)       The set of voters that support alternatives of n
                      = <<>>
      
      RelSup(n)       Relative support.  Voters on issue supporting note.
                      = |support(n)| <<>> |voters(n)|

      GroupSup(n,G)   Relative support within a group
                      = |support(n) ^ G| <<>> |G|

      MostSupported   The most supported alternative
                      MSA(n) = ArgMax_<<>> sup(n)|

      Consensus(n)    The node with the greatest clipped support.
                      (Integrate support over voters clipping at RelSup)
                      = SUM_<<>>
                          MIN<<>>
                                 <<>>

      Coverage(n)     The average Max RelSup over the voters
                      = AVG_<<>>
                           MAX_<<>> RelSup(p)
                      Note: coverage(n) >= relsup(n)
      

    Consensus
    Suppose:
      P1 has 90% support, and 100% support of n, and
      P2 has 10% support, and 0%   support of n.
      RelSup(n)=90%  Consensus(n) = 81%

    Suppose:
      P1 has 80% support, and 90% support of n, and
      P2 has 20% support, and 10% support of n
      RelSup(n)= 74%  Consensus(n) = 61% = 59.2 + 2


    Let m be the support of the clipped majority class

    Let c by the fraction of clipped classes
     



    10+4*9 37<<>>5                  

      Consensus(


      Bias(n)         Average variation in support across different positions
                      = AVG_<<>>
                           AVG_<<>>
                              | RelSup(n)-GroupSup(n,sup(p)) | *
                              |support(p)| <<>> |voters(n)|

      Bias2(n)        Same but square the diff in support to exagerate impact
                      of large differences in support.

      LeastBiased(n)  The note with the most even support across all positions.
                      LBA(n) = ArgMin_<<>> Bias(n)




      


    WRONG
      Bias(n)         Average variation in support across different positions
                      = AVG_<<>> | RelSup(n)-GroupSup(n,sup(p)) | *
                                 |support(p)| <<>> |voters(n)|

      Bias2(n)        = AVG_<<>> (RelSup(n)-GroupSup(n,sup(p)))^2 *
                                 |support(p)| <<>> |voters(n)|






    SUPPORT
         The size of the support set for a note.

    MOST SUPPORTED ALTERNATE
         The note in a node with a support set of greatest size.
         

    BIAS
         
         (The sum of the difference between support with in each position
          and the average support weighted by the size of each position)
         bias(n) = average{ | average(n) - 


    BIAS
         

    POLARITY
         The polarity of a note is 4*S*O/(V^2)








    === new stuff ===

    Contraversy -- product of support and opposition summed over all alternatives.

    Support -- votes for minus votes against


    Most supported -- Topic or position with the greatest support among alternates



    A nodes title is always 



    =======

    > SUPPORT
    > OPPOSITION
    === All following defined on a node
    > POPULATION              Total number of people issuing a direct note
    > VOTING POPULATION       Total number of votes
    > VOLUME                  Notes per day


    Direct Support -- the number of participants issuing supporting note.

    Direct Opposition -- the number of participants issuing an opposing note.

    Local Traffic -- the significance of a note is the number of direct
        children it has.

    Volume -- significance per time period

    Voting Population -- set if participants that either support or oppose
    a position directly under an issue.




    DESIGNATIONS





    > Major position
      All position with at least half the support of the smallest 
      position in the top 75% of all support.
    > Minor position
      All postions with a minimal amout of support.
    > Proposed position
      A submitted position.

    SUPPORT SET

    OPPOSITION SET

    OPINION SET

    SUPPORT RATIO
    This is the size of the support set divided by the size of the
    opinion set.


    ==========================
    === METRICS FOR ISSUES ===
    ==========================
    An issue represents a point of contention--it represents the center
    of a particular topic for debate.  Thus we focus on metrics that
    are defined in terms of the issue note.


    COMMITTED PARTICIPANTS -- This is the set of participants that have
    offered a supporting or opposing opinion on some position on this
    issue (or have submitted a position).


    SUPPORTING PERCENTAGE is the fraction of committed participants that 
    support a particular note minus the fraction that oppose it.


    COVERAGE ON AN ISSUE -- The percentage of committed participants that 
    support a note or set of notes

    BIAS ON AN ISSUE -- The average deviation of support and opposition
    across the supporters from each of the positions on an issue.  For
    example, a summary note on how hand guns affect the safety of the
    wearer might have 30% overall support, 25% support from the pro-gun
    position, and a 45% support from the anti-gun position.  This yeilds
    deviations of 5% and 15% for an average deviation or bias of 10%.
    Another summary with 20%, 21%, and 19% supports would have a 1% bias.

    This might be computed by assuming that support and opposition are binomial
    distributions, and computing the greatest deviation from participants
    in each position justified at the 95% cofidence level.



    THE BIAS RATE FOR A SUMMARY -- Represents the degree to which its
    support is polarized among the major positions.  Use statistics to
    compute the average change in the summaries percent support across all
    major and minor positions on the issue.


    THE CONSENSUS SUMMARY -- is the summary note 
    (1) that is supported by at least 25% of the commited participants
    (2) has a bias rating no greater than 10%
    (3) and has the greated supporting percentage



    Identifier -- id translation


    Stale information
    - Support or opposition is intended to represent a participants
    *current* support or opposition for some note.  Thus if a note remains
    an active center of debate for a period of years then it may become
    necessary to poll participants to verify that they still stand 
    behind opinions previously submitted.
