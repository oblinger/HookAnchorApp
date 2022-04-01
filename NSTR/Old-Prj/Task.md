# Old-Prj.Task -- &gt;head

              +complete, x notGoingTodo 
              s SonyTodo, S SonyKnows 
     
     
     
     
     
     
     
     
     
    > MAIN TASKS 
    o Collect Training Data 
    o Data Preprocessing 
     
     
    > DATA PREPROCESSING 
    - User Identification 
    - Xlation: between: cNum, serial number, notes address, internet addr 
    o Word Context Determination 
     
     
    > USER IDENTIFICATION 
      Generate: unique id, full name, and short name for an arbitrary 
      email address, name, or ldap entry. 
      * Applied to from/to fields 
      * Applied to capitalized words in email? 
      * Applied to web profile names 
     
    > WORD CONTEXT DETERMINATION 
     <<>>   
    s from/to/cc 
     
     
    > COLLECT TRAINING DATA (Using remote mail access and web profiles) 
    - Gain Access 
      ! Get button working 
      - Determine "safe" machine for use a server 
      s Get application running on a server machine 
      ? Herb Derby:  common auth for mail 
    - Web profiles 
      = Get set of profile questions in .XML format 
      s Get Rao's editor running on a server machine 
      = Figure out how users will ID themselves (cnum=serial num) 
      s Get serial number, lookup  
    - Write and send button press email 
    - Write/Run collector on user hits 
     
     
       
     
     
     
     
     
     
     
     
     
    [MAY DELIVERABLES]  
    - Demo 
      - Parse tel, names?, urls 
      - Read mail Generate files 
      - Compute lifted terms 
    - Compute lift sets 
      - read other DBs 
    - Skill Taxonomy For Research (Rob) 
    - Simple Profile Interface 
    - Initial Test Of Data Snarfing 
    - Initial Test Of Classification 
     
     
    MAY DELIVERABLES 
    - Specificatation of how skills are defined 
      Yes.  Me/All 
    - Research skill taxonomy 
      Yes.  Rob 
    - Get initial email files 
      Yes.  Me.  (privacy) 
    - Use HR's PSU update tool & push questionaire to PB. 
      ???.   
    - Simple profile viewing interface 
      Yes.  Me. 
    - Initial (deployable) feature extraction 
      Yes.  Me. 
     
    [[ SKILL SCANNER ]] 
     
    [DECISION/KNOWLEDGE BG] 
    - Skill specification language (level of expertise, years experience, pub/private) 
    - Initial Taxonomy 
     
    [INFRASTRUCTURE] 
    - Backend API 
      <<>> May (need to talk w. Allen about extending to capture their data) 
    - Classifier API 
      <<>> VERSION DONE 
    - Interface API 
      <<>> May 
    - Skills Data DB Store & Interface 
    - Client End Installer 
      <<>> 
     
    [EXTRACTION] 
    - Notes Interface 
      <<>> 
    - Stemmer 
      <<>> 
    - CONTEXT (Location within Msg) feature computation 
      <<>> 
    - CONTEXT (Msg Type) feature computation 
      <<>> 
    - STRUCTURAL feature computation 
      <<>> 
    - Feature Vector Accumulation   
      <<>> VERSION DONE 
      - TIME-Based attenuation 
      <<>> 
     
    [EXTRACTION OPTIONAL/SUBSEQUENT COMPONENTS] 
    - Multi-word Term Extraction 
      <<>> IN PLAN (will play w. if I have time) 
    - Attachment Reader 
      <<>> (I hope to get this from raven) 
    - Server Based Data Extraction 
      <<>> (ACTIVE: I am exploring this now w. D. Wood) 
      - Incremental Feature Vector Accumulation 
      <<>> 
     
    [BACKEND] 
    - Classifier 
      <<>> (ACTIVE -- trying to get code from Thilo or Eric Brown, but 
              I may have to port these to Java myself.) 
    - DataSet Manipulations 
      <<>> VERSION BY MAY (ACTIVE) 
    - Rule Application Engine 
      <<>> 
    - Static skill associations metrics 
      <<>> 
    [BACKEND OPTIONAL/SUBSEQUENT] 
    - Unlabelled learning system 
      (This is a fancy technology for leraning w. very little training data. 
       this would be great rocket science for Dan Pelleg!!) 
     
     
    [INTERFACE] 
    - Single Profile View 
      <<>>  (Contractor support) 
    - Profile Edit 
      <<>>  (Contractor support) 
    - Administrators Interface 
    - Skill Search 
    - Dummy Data Backend -- This will allow a demo w. no profiles DB 
      <<>>  (Contractor support) 
     
