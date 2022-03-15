# PBC.0A-PBC00 --

     <<>>     
     
    >>>   [[[WIN]]] 
     
    >>> COMPLETED  SCIENTIFIC\TECHNICAL WORK 
    >>>(LIST SIGNIFICANT COMPLETED TECHNICAL WORK, WITH A SHORT (1-2 SENTENCE) DESCRIPTION OF 
    >>> ACCOMPLISHMENT INCLUDING IMPORTANCE, OR POTENTIAL IMPACT, EITHER ON IBM OR THE SCIENTIFIC/PROFESSIONAL COMMUNITY.) 
     
    Employee Skill Identification 
    - Synthesized ideas of e-mail community finding (Ukelson and Fairweather), and employee skill profiling 
      (Cathy Lasser) into a machine-learning based approach for skill identification. 
    - Deployed simple e-mail feature extraction demonstration. 
    - Implemented many of the infrastructure modules necessary for e-mail based skill identification. 
      - Sparse Vector Dataset manipulation package, Feature parsing package, Notes remote database access 
        (including automated approach for access authorization).  Generic induction algorithm interface package. 
    - Web-based Skill profile editing interface (implemented by Narsimha Rao and Sony Sebastian under my direction). 
      - Extensible Skill-Rule evaluation language (and report generator) 
      - Programmable data-tabulation language 
     
    - Primary impact for this year is set of experiments that determine depth of skill inference possible 
      from individual e-mail features.  (We believe single feature inference will correlate well multi-feature skill inference.) 
      Limitation of analysis this year is likely to be breadth of available testing data.  Experiments 
      can be immediately rerun once sufficient additional data is obtained.  Experiments: 
    + Testing many manually generated skill rules  (By test set ordering rule report) 
      + Most distinguishing mail features report (for distinguishing single e-mail databases).   
        (Experiment repeated using many alternative definitions of "distinguished feature") 
      + Best simultaneous separator report, lists features by their ability to *simultaneously* 
        separate "positive" and "negative" groups of mail databases. 
    - Ultimate impact of automated skill identification will be profound if detailed skill 
      profiles can be obtained with minimal human intervention.  Such technology makes 
      effective human resource management feasible for even the largest organizations. 
     
     
    >>> ONGOING AND NEWLY INITIATED WORK 
    >>>(LIST SIGNIFICANT ONGOING AND NEWLY INITIATED WORK, INCLUDING  A SHORT  DESCRIPTION OF  
    >>>  IMPORTANCE AND, WHERE POSSIBLE, CHECKPOINTS MET.) 
     
    Skill identification problem requires extension of classical learning techniques.  Components and  
    experiments developed to date are designed to test and implement the following three: 
    - Detection of e-mail based social networks communities 
    - Adaptive construction of features from a prohibitively large cross-product space. 
    - Combination of supervised and unsupervised modeling of unstructured textual data. 
     
    Approximate Clustering of Very Large Datasets. 
    - Applying existing clustering techniques to very large datasets 
      is not possible because these algorithms often require data 
      to be in memory, and are super-linear in dataset size. 
    - Over the course of 20 months Nina Mishra (at Hewlett Packard), Lenny Pitt (at U. of Illinois) 
      and I proved probabilistic (PAC) bounds on performance degradation of sub-sample clustering 
      that are *independent* of the initial dataset size.   
    - Understanding this sampling approach is crucial since it is the only feasible approach for 
      clustering very large datasets (e.g. the web, financial transaction logs, phone records, click stream data, etc.). 
    - Results submitted to next years SODA Conference (Symposium on Discrete Algorithms) 
     
    >>> GROWTH AND SERVICE 
    >>> (INCLUDING PERSONAL DEVELOPMENT, TASK FORCE PARTICIPATION, CUSTOMER INTERACTIONS, MENTORING, COMMUNITY   
    >>> IMPACT.) 
     
    Learned to make meaningful, simultaneous contributions to many distributed groups in time-efficient manner.  
    Skill Mining: Farrell, Rao, Sony, myself.  AI-PIC: Morgenstern, many others, myself.  Patents: Keller, Podlaseck, Biebesheimer, myself.  Decision Tree 
    Experiments: Vilalta, myself.  Clustering Proofs: Mishra, Pitt, myself.  WMR: Adams, myself. 
    Encyclopedia Britannica: Fairweather, Prager, myself.  Workshop: Vilalta, myself.  Session Co-chair: Hellerstein, Rish,  
    Vilalta, myself.   
     
    >>> EMPLOYEE DEVELOPMENT (PRIMARILY FOR MANAGERS) 
    >>>(LIST SPECIFIC ACTIONS YOU HAVE TAKEN TO HELP INDIVIDUALS OR GROUPS DEVELOP NEW SKILLS, MAKE MORE EFFECTIVE  
    >>>DECISIONS, ETC.  LIST HERE THE NEW EMPLOYEES WHO YOU BROUGHT IN TO RESEARCH FROM WHOM YOU EXPECT A 
    >>>PARTICULARLY  HIGH LEVEL OF CONTRIBUTION .  ALSO LIST  ACTIONS THAT HAVE HELPED RETAIN OUR  CRITICAL SKILLS/KEY 
    >>>EMPLOYEES.) 
     
     
     
     
     
    >>>   [[[TEAM]]] 
     
     
    >>> DEVELOPING OUR COMMUNITY 
    >>> (INCLUDE ITEMS SUCH AS DEVELOPING AND HOSTING SEMINAR SERIES, MEMBERSHIP IN LOCAL COUNCILS SUCH AS THE 
    >>>   DIVERSITY COUNCIL, NOMINATING MEMBERS OF THE STAFF FOR EXTERNAL HONORS) 
     
    Instrumental in the formation of the AI-PIC, and an associated tutorials track. 
    - PIC is active and growing. 
    - The first half-day tutorial track was on a Bayesian Reasoning Tutorial. 
      It was very well received with a strong attendance of over 30 researchers  
      attending it in its entirety.  We have speakers for two future tutorials. 
      (Specific contributions to the AI-PIC this year are listed in CROSS ORGANAZATIONAL section below) 
     
    Hosted Arthur Caplan, speaker as part of Paul Horn's Visions Seminar Series. 
     (Built collection of web pages, organized lunch/dinner guests, escorted speaker, etc.) 
     
    Coordinator for summer students with in the SAS area. 
     
     
    >>> TEAMWORK AND CROSS-ORGANIZATIONAL CONTRIBUTION 
    >>> (ENTER HERE YOUR COLLABORATIVE INVOLVEMENTS, INCLUDING CONTRIBUTIONS TO TEAMS AND OTHER PROJECTS  
    >>>  BOTH INSIDE AND  OUTSIDE YOUR ORGANIZATION.) 
     
    Significant contributor to the Artificial Intelligence Professional Interest Community  (AI-PIC) 
    - Invited and hosted Pat Langley, the president of the International Conference on Machine Learning. 
    - Coordinated the AI-PIC lunch meetings, and some of the AI-PIC seminars. 
    - Hosted Nina Mishra from Hewlett Packard as part of the AI-PIC seminar series. 
     
     
    >>>UNIVERSITY RELATIONS AND RECRUITING 
    >>>(ENTER HERE A LIST OF PEOPLE TO WHOM OFFERS WERE MADE DURING THE YEAR IN WHICH YOU  PLAYED A SIGNIFICANT ROLE. 
    >>> LIST WAYS IN WHICH YOU HAVE HELPED IMPROVE OUR DIVERSITY INITIATIVE, VISIBILITY ON CAMPUS, INCLUDING SEMINARS,  
    >>> AND FACULTY VISITORS YOU HAVE HOSTED.) 
     
    - Maintaining contact with potential AI recruits outside of IBM:  Dan Pelleg and Karnal Nigam. 
    - Spent significant time and energy supporting the hiring of two new RSMs: Mark Brodie and Carolyn Fiebig 
      (Note: My role in their decision to join IBM was entirely an unofficial one.) 
    - Hosted Pat Langley (listed above).  He is a Stanford professor, and director of ISLE (the Institute for the  
      Study of Learning and Expertise) 
     
     
    >>>EFFECTIVE LEADERSHIP (PRIMARILY FOR MANAGERS) 
    >>>(CITE EXAMPLES OF HOW YOU HAVE LED YOUR TEAM AND INDIVIDUALS ON A DAY TO DAY BASIS. DESCRIBE HOW  YOUR ACTIONS WERE  
    >>> EFFECTIVE PEOPLE MANAGEMENT AND LEADERSHIP QUALITIES AND HOW THERE WERE EFFECTIVE FOR YOUR TEAM OR FOR THE INDIVIDUAL.)  
     
    - Directed Narasimha Rao's development of skills profile editing website. 
    - Directed Sony ###'s development of the backend skills database. 
      + Coordinated database design details with Rob Farrell, and communicated these to Sony, 
      + Provided Java API and documentation for interface for the database backend layer. 
     
     
    >>>ORGANIZATION AND TEAMWORK (PRIMARILY FOR MANAGERS) 
    >>> (ENTER HERE ORGANIZATIONAL CHANGES, CONSOLIDATIONS, ETC. AND CROSS-GROUP TEAMWORK YOU HAVE INITIATED OR  
    >>>  ENCOURAGED THAT HAS SIGNIFICANTLY IMPROVED THE GROUP'S IMPACT.)  
     
     
     
    >>>   [[[EXECUTE]]] 
     
    >>>PRODUCT/SERVICES IMPACT 
    >>>(LIST HERE THE IBM GROUPS/UNITS WHO HAVE PROFITED FROM YOUR WORK AND COLLABORATION DURING THE YEAR AND THE           
    >>> NATURE OF  YOUR CONTRIBUTION.  CONTRIBUTIONS SHOULD INCLUDE TECHNICAL CONTRIBUTIONS, TASK FORCE MEMBERSHIPS,  
    >>> CONSULTING, ETC.  IF YOU ARE A MANAGER, ALSO LIST HERE THE IMPACT WITHIN IBM AS A WHOLE FROM THE GROUP(S) MANAGED) 
     
    Contribution to Planet Blue 
    - Assisted in design and implementation of the personal information database, development of simple email extraction 
    demonstration, and profile editing interface. 
     
    Application of Watch-Me!-Read Diagnostics to Adult Literacy.         
      - Released new version of the WMR Diagnostics, "At a glance".  Release included several relatively  
        minor additions and bug fixes in service of the Adult Literacy Program. 
      - The adult literacy version of the WMR platform is currently deployed on 50 systems across 7 adult literacy 
        centers, and is also being used in an ESL (English as a Second Language) program in Germany. 
     
     
    >>>EXTERNAL PUBLICATIONS AND PRESENTATIONS  
    >>>(LIST IN ORDER OF SIGNIFICANCE, PUBLICATIONS, PUBLICATIONS IN PRESS AND CONTRIBUTED TALKS, ETC. 
    >>> LIST ANY  PROFESSIONAL SOCIETIES OF WHICH YOU ARE A MEMBER, AND  INDICATE ANY OFFICES HELD OR MEETINGS ORGANIZED OR  
    >>> CHAIRED, JOURNAL EDITORSHIPS, ETC.   IN ADDITION, IF YOU ARE A MANAGER, DESCRIBE HERE ANY SIGNIFICANT IMPACT BY  
    >>> MEMBERS OF THE GROUP(S) MANAGED) 
     
    * ICML 2000 conference paper: "A Quantification of Distance-Bias Between Evaluation Metrics In Classification." 
      - Paper provides framework for analysis of the four most popular decision-tree splitting metric, followed 
        by an experimental comparison of these alternatives. 
      - ICML (International Conference on Machine Learning) is the longest running and most prestigious forum  
        for Machine Learning research. 
     
    * Co-chair for a workshop held as part of ICML 2000.  Workshop focuses on domains of applicability for 
      current learning techniques:  "What Works Well Where?" 
      - The editor of the Machine Learning Journal has expressed interest in our proposal to follow this workshop with 
        a Special Issue of the MLJ devoted to this workshop topic.  Final approval of such a special 
        issue will not occur until next year.   
     
    * Associate Editor for the International Conference on Artificial Intelligence, and  
      Co-chair for the Inductive Learning session at this conference. 
     
     
     
    >>>INTERNAL PUBLICATIONS AND PRESENTATIONS 
    >>>(LIST IN ORDER OF SIGNIFICANCE, PUBLICATIONS AND PRESENTATIONS, INCLUDING WHITE PAPERS, INTENDED PRIMARILY FOR AN  
    >>> IBM AUDIENCE.   DESCRIBE IMPACT.) 
     
    E-mail based skill profiling 
        + A FOAK Proposal, "Email based skill identification" 
        + An e-BTI Proposal: "Email based user profiling" 
      - Presented concept to IRIS as a possible component of the Raven architecture. 
      - Series of three conference calls and a presentation to the Architect for the 
        HR skills application, and members of the Learning Integrators subgroup 
        (also part of Human Resources). 
      - Primary impact of these actions to date, has been the solidification of a 
        basic approach that integrates: email communities, skills, and Machine Learning. 
        Work is an ongoing part of the Planet Blue project (listed above).  We believe initial contacts  
        with Human Resources (for IGS) and Lotus/Iris/Raven may yield significant contributions  
        with in one or both of these organization. 
     
     
    >>> PATENT DISCLOSURE ACTIVITY 
    >>>-  FILED;  -  RATED FILE (PENDING); -  RATED PUBLISH; -  RATED MERGE/FILE; -  PATENT ISSUED 
     
    * Submitted seven patent disclosures on adaptive tutoring environments. 
      I have contributed to (am an inventor on) the seven listed here.  I am the original author for   
      patents #3 and #4; I am the sole inventor on Patent #5 and #7.  We expect to follow these disclosures  
      with subsequent filings for each of the patents.  (My first meeting with the attorney, Dan Morris, 
      should late next week.  Perhaps we can file these disclosures before the end of the year.) 
      1.  Customer Self Service System for Learning. 
      2.  Customer Self Service Iconic Interface for Learner Portal Entry and Search Specification 
      3.  Customer Self Service Subsystem for Classifying User Contexts 
      4.  Customer Self Service Subsystem for Adaptive Indexing of Learning Solutions 
      5.  Customer Self Service Subsystem for Response Set Ordering and Annotation 
      6.  Customer Self Service Iconic Interface for Learning Search Results Display and Selection 
      7.  Customer Self Service Subsystem for Context Cluster Discovery and Validation 
       
    >>>IBM AWARDS AND RECOGNITION 
    >>> (INCLUDE RECOGNITION FROM OUTSIDE THE RESEARCH DIVISION.) 
     
     
    >>> PROFESSIONAL AWARDS OR HONORS 
    >>>(ENTER HERE A LIST OF EXTERNAL AWARDS OR HONORS RECEIVED.) 
     
     
    >>>NEW IMPACT FROM PREVIOUS YEARS' WORK 
    >>>(ENTER HERE A LIST OF  ANY WORK FROM PREVIOUS YEARS THAT HAS HAD SIGNIFICANT NEW IMPACT THIS YEAR. 
    >>>  DETAIL THE NATURE OF THE IMPACT.) 
     
