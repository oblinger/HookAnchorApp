# NH.Targets --


    Places to route attachments, new children, line entries


    - Targets are specially designated local or global target notes.
    - The name of a target is the note's shortname, and 
      its one char code is the first char of the shortname
    - A target is obtained by scanning the parantage of a note looking for
      immediate children that are targets or a global target.



    Implementation
    - targetMap     targetParentFullname --> List of possible immediate children targets.
    - Note properties:   localTarget, globalTarget
    - getTarget(prefix)  scans for specified target
