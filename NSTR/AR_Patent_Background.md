# AR\_Patent\_Background --

    Hi Louis,

    I've done some investigation into prior art for this patent disclosure.  In our last discussion, we decided that we'd like to make the broad primary claim of synchronizing document display with the actions a user performs.  The secondary claim was that we could automatically align user actions with existing documentation.  Based on my survey, I believe we can move forward with this patent by narrowing the broad claim a bit.

    One category of related work is in context-sensitive help, e.g., Microsoft's paperclip.  (See, for example, patents US6260035 and US6452607.)  These inventions describe the display of help documentation in response to (one or more) user actions and various system events.  For example, if the paperclip observes a user mousing around in a series of menus, it may conclude that the user is looking for some feature, and display help text appropriate to that task.  This can be distinguished from our claim in that prior systems synchronize to an entire task description (e.g., a help document telling you how to print your spreadsheet), rather than to specific steps within the task.

    The second category of prior art is in computer-based training.  (See for example patent US20040086834A1.)  I've observed tutorial systems like this in computer games, in which the system presents a series of exercises to the user as a mechanism for teaching the user how to operate the application.  Each exercise is designed to teach the user how to perform some task in the interface.  The user operates the application as instructed by the exercise, and at each step when the system detects that the step has been completed, it moves on to the next step in the exercise and displays another section of text describing the next part of the exercise.  This can be distinguished from our claim by noting that training systems have users perform a task in a context specified by the training system, compared to our claim where our system can support users performing their own tasks.  For example, a computer-based training system that teaches you how to print a document would open up a test document chosen by the system, and show you how to print it.  In contrast, our system could walk you through the process of printing your own document.

    The third category of related work, as mentioned in the evaluation meeting, is that of automated wizards, including the "Cheat Sheets" available in IBM's WebSphere product line.  Although these wizards show a user how to perform a task in the user's context (rather than an artificial training context), they do not pay attention to what the user is doing.  The user must specifically tell the wizard that they've finished a step and are moving on to the next step.

    In light of this related work, I believe we can claim documentation that advances in synchrony with a user performing a task described in the documentation, given that the task being performed is within the context of the user doing his own work (rather than a training exercise).

    What do you think?  Can we move forward on this patent application?  We'd like to demo this work publically at the end of October, so we need to move quickly.

    Thanks,
    --Tessa
    =====
    Tessa Lau, PhD         IBM TJ Watson Research          914-784-6095
