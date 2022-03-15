# 999PersonalWeb --

     <<>>   
     <<>>   
     
     
    OBJECTIVE: 
      A tool for rapid, flexible organizing of information: 
      - Instant Authoring 
      - Flexible Structure (partial, embedding, sub-component reference) 
      - Universal Reference 
      - Ubiquitous Access 
     
      - Facilitates multiply reorganized info 
        - Roll back on structure 
        - Live embedding (same paragraph exists in two docs) 
     
     
     
    KEY FEATURES: 
     
    - FLEXIBLE ORGANIZATION.   (Hypertext and Page Layout.  E.G. HTML) 
      - Use hypertext (nodes w. internal & external links) 
      - Use generic page layout representation  (E.G. WYSIWUG editor like Word Pro) 
      - Specialized structures built ontop of generic  (eg calendars, address book, etc.) 
      - Seperate Logical/physical naming.  Data stored & manipulated as  
        simple files in file system. 
     
    - INSTANT CREATION/EDITING. 
      - Ablity to instantly switch from from viewing to creating/editing. 
      - Instant creation requires partial specification 
        No Title, No Linking, No content 
      - Instant (independant) editing of:  Content; Link Structure 
      - PROTOTYPING.  User can create template structure, content, and actions 
        this is used as a template for repeated knowledge (e.g. a phone call entry) 
     
    - MULTIPLE VIEWS. 
      - EMBEDDING.  Ability to create multiple views based on the same content 
      - FINE-GRAIN sharing between views:  e.g. embedding whole node, single  
        sub-section, or single line, in new view. 
      - MERGED LISTS.  Items from one list are embedded as items in another list. 
      - AUTOMATIC GENERATION.  Computed views 
        - View all docs by authorship time, by author name, by revision time, 
          search by content, etc. 
     
    - UNIVERSAL REFERENCE.   
      - All objects (and sub-parts of objects) in the electronic 
        universe are referenceable within system. 
        E.g. Notes document, URL, e-mail address, Machine name, File name, 
             notes server, printer name, Internet Bookmark files, etc. 
     
    --------- 
     
     
    SCENERIO SNIPITS 
     
    - You e-mail me a URL that I want to remember, but I don't want to think 
      about where to put it right now. 
     
    - I am going to a meeting with Jacob, and I want to put together a quick 
      agenda docuement that links together a few relevant sub-sections of 
      a much larger corpus of text.  Edits on this new view should affect the  
      source text. 
     
    - My project is getting too big to have a single 'ToDo' list,  
      so I want to dynamically create a sub-project node with its own  
      'ToDo' items, but I to keep the global ToDo list view, which automatically 
      stays in sync with the sub-todo list. 
     
    - I just returned from a conference where I haphazardly entered many disparate 
      and unconnected fact.  Now I want to view all recently created  
      (but unlinked) nodes, so that I put them in places where they can be later 
      found. 
     
    - After entering reference on Machine Learning for 5 years I want to  
      "clean up" my knowledge tree, but I don't want to have to worry about  
      deleting things that unexpectedly turn out to be relevant, so I want 
      to create a new overlaying structure without destroying the old structure 
      and sometimes stealing huge chunks from the old structure. 
     
    - Bill and I both both keep informal summaries of important papers in  
      Particle Physics, and we would like to share & simultaneously update 
      only this info.  Thus *without* changing the logical location 
      of this info in our hypertext linked info-space, we shift the  
      physical file storage to a file that we both have access to on a  
      common server machine.  In bill's case he is using SAMBA to access the file, 
      and in my case I am using Microsoft Networking to access the common file. 
     
    - When organizing a new sub-project I like to see a snapshot of all 
      relevant info starting at a single root page.  This root page has 
      many lists of info relvant to that project.  A list of ASAP items, a 
      list of todo items, a list of phone call back items, a log of related 
      misc data, a log of related meeting agendas, etc.  All of these lists 
      are merged hierarchically into super project pages all the way up to a 
      global ToDo, and ASAP list. 
     
      I use the generic sub-project template as a starting point for a new  
      page regarding the optional "client-side GUI" on my latest project.   
      In one step this template creates a page with the five lists mentioned  
      above, already linked hierarchly to the lists in the parent node. 
     
     
       
     
     
    CONCRETLY WHAT DO I WANT TO BUILD 
     
    * Generic HTML WYSIWYG editor. 
    * Specialized server of dynamically created HTML 
      - splices content from multiple sources 
      - dynamically generate content of merged-lists 
      - maps content between physical and logical location 
        (hierarchical naming, context sensitive name resolution) 
    * Hypertext manipulation operations 
      - Create/Delete/Move/Merge/Split nodes 
      - Node Template Instantiation 
    * Auto view generators 
      - By keyword search, Creation Date, Revision Date, Author, etc. 
     
     
