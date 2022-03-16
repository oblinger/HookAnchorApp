# AR.44\_Britten --

    [Executive summary: see the last paragraph]

    Larry and I talked to KB again today.  She put us in touch with Steve Ims, in the adtech group, who joined our call.  Steve is working on automating the process of securing a portal server installation.  His group started with the Redbook descibing the steps involved in this process (<<>>).  They then hand-authored Ant scripts that automate the process, producing a "wizard" that automatically runs through the process.  The procedure may be parameterized, but these parameters are set by editing a property file before running the wizard.

    I had a look at a couple of the Ant scripts.  They seem to be mostly commands to run or files to copy.  Presumably the hard work they did was to come up with command-line equivalents for each of the GUI actions described in the Redbook, and script these command-lines together in the right order.  I see our technology as parallel to theirs, starting with a Redbook and producing a semi-automated interactive procedure rather than a fully-automated one.

    Larry and I decided that our plan of action will be to read over the Redbook, identify a concrete subprocedure that we could start working on, and then either go back to Steve or Kathryn to talk about how to instrument it.  I took a quick look at the Redbook.  The procedure touches all sorts of interfaces, from Windows "add user" control panels, to Swing UIs, to running DOS BAT files, to web-based configuration UIs.  I didn't see any obvious conditionals, but there are lots of variables which have to be entered in different windows at different points during the procedure (usernames, hostnames, installation-specific stuff).

    We also talked a bit about the Integrated Runtime.  This is a packaging of IBMHttpServer, DB2, and WebSphere App Server for the midmarket environment.  Essentially they're in the process of porting the config interfaces for these apps to the ISC. So once we identify the subprocedure we're interested in, we may want to talk to these guys to find out whether it's been ported to the ISC yet.  (Though, in hindsight, perhaps we'd do better to choose a subprocedure which has already been ported!)

    The IR may also need our technology sooner than the ISC will.  KB mentioned that their focus is on controlling multiple servers from a single "network central console".  So they may have a need to record a procedure on one server, then play in back on the other servers.

    KB mentioned that Randy Horman, our suggested contact who could identify specific WebSphere-based tasks, is hard to get hold of these days, and we should put it off for now.  But Steve Ims does work for Randy, so he may be our best contact anyway.

    KB also mentioned that using Ims's scripts, one admin reduced the time it took to perform the procedure from a week to 45 minutes.

    There certainly seems to be value in automating these Redbook procedures.  The first step is identifying a subset of this problem that we could start addressing, and build a quick demo that illustrates the potential of the approach.  This problem of entering the same variables into different places in the procedure seems to be key.  We should be able to learn those variables by example.  But would that be a compelling enough demo?

    --Tessa
    =====
    Tessa Lau, PhD         IBM TJ Watson Research          914-784-6095
