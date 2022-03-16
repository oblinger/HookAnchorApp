# Docs.Debugging\_Mode --


    Debugging mode can be invoked from the 'Advanced' Menu, or by clicking on the '' mode symbol
    in the very low left corner of the notester screen.  Once in debug mode this command will either
    take a single step (a single goto), or it will exit the debug mode if all goto actions are complete.

    When Debugging is active it will cause all 'Goto' operations to be executed in single step mode.
    Many notester commands are built from a sequence of Note gotos.  By single stepping through these gotos
    the user can see (and reconfigure) the notes which define the behavior of Notester commands.


    For example if the user clicks '' and then types TAB-V the system will take a single step
    into the notester views menu.  The user could edit this page to change the views menu.
    By clicking the '' button again, the user the activates the menu and can select
    some view.   Now the system will display the definition page for that view.
    Editing this page will change the layout for that view.
    Clicking the '' button again, will the activate that layout.
    Finally clicking the '' button a final time will turn off debugging step mode since
    there are no more 'goto' operations for that command.

    As you can see the DEBUG STEP MODE is potentially a powerful way to reconfigure Notester.
    Use some care however, since these pages define the behaviors of notester itself, you
    can really torque notster badly by using these pages.

    * The 'Development Menu -> Revert Note' command can be used to revert any given note
      back to its factory default setting (if you really screw up some page)
    * The 'Advanced Menu -> Goto Raw Note' is similar to debug mode.  It can be used to 
      goto a note without activating its behavior.  Thus you can also use this command
      to directly goto some configuration page to change its behavior.
