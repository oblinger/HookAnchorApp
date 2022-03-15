# Docs.MOVE ITEM --


    MOVE COMMAND -- Moves within list.  This more advanced command allows user to quickly
    move items around within a single list.  The command waits for a second character to
    specify where the item should be moved to.  So for example 'ME' would move the current
    item to the end of the list.  (This command is very flexible&quick.  It can be use
    in sevaral ways while processing lists.  See <<>>)

      DESTINATION KEY
      [S]TART  of list
      [E]ND    of list
      [U]P     in list
      [D]DOWN  in list
      [?]      inline help

    These destination characters assumes user has added blank lines within the list to
    divide items into separate groups.
      [P]REVIOUS group
      [N]EXT     group
      [1]        first group
      [2]        second group
      ...
