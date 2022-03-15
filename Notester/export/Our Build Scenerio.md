# Our Build Scenerio --


     <<>>

     <<>>  scrolls off bottom
    space above edit window still stretches

    ### BUILDING ABBOT  (3)
    Menu Window->ShowView->Navigator
    Expand "abbot [cvs.sourceforge.net]"
    Expand "eclipse"
    Select ".project 1.1 (ASCII -kkv)"
    Menu Edit->Copy Ctrl+C
    Select "abbot [cvs.sourceforge.net]"
    Menu Edit:Paste 
    Click "yes"
    Select "build.xml   (IN THE ABBOT DIR; NOT ECLIPSE)
    Popup "build.xml
    Select "Run->2 Ant build ..."     (NOTE THE "...")
    Check "eclipse" box   (show if here?)
    Click "run" button

    ### BUIDING ABBOT 3.3 - 3.4
    Select "abbot [cvs.sourceforge
    Menu "File->Refresh 
    Expand "abbot [cvs.sourceforge    (Optional step)
    Expand "build"
    Expand "eclipse"
    Multi-Select "doc" "lib" ".project" "abbot.jar" "build.properties" "example.jar" "plugin.xml" "README.html"
    Menu "Edit -> Copy
    Select "abbot [cvs.sourceforge
    Menu "Edit -> Paste"
    Click "Yes to all"

    ### BUILDING ABBOT 3.5 - 3.6
    Select "plugin.xml              (NOTE: UNDER THE ROOT)
    Popup  "plugin.xml
    Select "PDE Tools -> Update Classpath..."
    Click  "finish"
    Menu   "Project -> Properties"
    Select "Java Build Path"
    Select Tab "Source"
    Click  "Add Folder"
    Check "src"
    Check "test"
    Click "ok"
    Select tab "Order and Export"
      (all selected except last two)
    Click "ok"

    ### BUILD ABBOT 3.8
    Select "abbot [cvs.sourcefourge
    Menu   "Project -> Build Project

    ### BUILD ABBOT 4
    Select "abbot.swt [cvs.sourceforge
    Menu   "Project -> Properties
    Select "Java Build Path"
    Select Tab "Source"
    Click  "Add folder..."
    Check  "src"
    Check  "test"
    Check  "example"
    Click  "OK"
    Click  "Yes"
    Click  "OK"
    Menu   "Project -> Build Project"



    (3.2) Build abbot using Ant: Select its build.xml, and choose (from the menubar) Run>External Tools>External Tools>abbot build.xml. Choose the Targets tab, unset target=default and set target=eclipse. You may get some spurious redness in the console (e.g. deprecation), but you should also see "BUILD SUCCESSFUL". (Of course, you can also run ant outside of eclipse if desired.) 

    (3.3) Refresh: Select abbot's folder and F5 or right-click>Refresh 

    (3.4) Copy everything in & below abbot/build/eclipse to abbot/ (aka abbot's root), overwriting everything. Again, use Navigator or other dotfile-aware tool (e.g., a good shell :-) We should probably force the ant target to do this ... there was some reason ... 

    (3.5) Update the classpath: Select abbot/plugin.xml, right-click>PDE Tools>Update Classpath. Update only abbot's classpath (this should be the default selection) for now. 

    (3.6) Check the source folders: Select the project folder, right-click> Properties>Java Build Path. Under the Source tab, add src and test as source folders, if they are not already selected. (This step was necessitated by a known bug in Eclipse.) 

    (3.6) Check the libraries: from Properties>Java Build Path, choose the Order and Export tab, and check to see that the following are exported 

    · the source folders 

    · abbot's jars: <<>>.jar 

    · abbot's dependencies: <<>>.jar and MRJToolkitStubs.zip 


        If any jars are missing, add them. (Gotta update target=eclipse.)

    (3.8) Build abbot in eclipse: C-b, or abbot>Build Project, or use the menubar. This is only necessary so that eclipse can build dependent projects such as abbot.swt, and to avoid annoying red Xs. No errors should result: if they do, correct them or lemme know. 

    4 Build abbot.swt. This should be easier, since the CVS module is an eclipse plugin/project "by nature." 

    4.1 Select abbot.swt's project folder and right-click>PDE Tools>Update Classpath. (Or abbot.swt/plugin.xml if PDE is recalcitrant.) 

    4.2 Check the source folders: select abbot.swt's project folder, right-click>Properties>Java Build Path. Under the Source tab, add src, test, and example as source folders, if they are not already selected. 

    4.3 Build abbot.swt with eclipse, e.g. C-b. No errors should result: if they do, correct them or lemme know. 

    At this point, we would normally instruct you to run CelsiusConverter/Test ... except that we need to get it working again (i.e. on 3.0). It's top of our list, so hopefully will be done soon. 

    Meanwhile, you should be good to go on 3.0--lemme know if not. 

    HTH, Tom Roche, IBM Rational Developer Model2 Tooling, abbot admin
