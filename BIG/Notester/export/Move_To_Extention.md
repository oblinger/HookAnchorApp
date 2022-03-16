# Move\_To\_Extention --

    Start Regedit by navigating Start | Run and typing regedit in the dialog box. 
    Navigate to this key in the Registry file:
    HKEY_CLASSES_ROOT\AllFilesystemObjects\shellex\ContextMenuHandlers
    Note: The Send To key is already there. 
    Right-click on the ContextMenuHandlersfolder and select New | Key to add a new key. 
    Give it the name Copy To and then double-click the (Default) value of the new key's right-hand pane. 
    Type in this code as the new data:

    <<>>


    Right-click on the ContextMenuHandlersfolder and select New | Key to add a new key. 
    Give it the name Move To and then double-click the (Default) value of the new key's right-hand pane. 
    Type in this code as the new data:

    <<>>


    Click OK and close Regedit 
    Copy To and Move To should now be available options on the right-click context menu in Windows Explorer. 
