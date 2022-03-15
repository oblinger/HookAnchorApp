# Note\_Life\_Cycle --




    (1) Archiving
    Note life cycle:
    - A note is born, it may be a journal type note.
    - A note may be completed (copied) onto its project's journal|log|deleted sub note.
      (Its modification time is updated in this process)
    - A completed note may be brought back by simply restoring its parent  (which was stored in some field?)
    - All journal or completed notes older than a certain date are archived
    - Archiving is done in modification date order by:
      Sorting all archivable notes by mod time and then archiving them.
      The earliest of these notes must be later than the currently archived notes
      since any earlier journal notes would have been written out, and new notes or any newly
      completed notes will have modification times later than the previous archiving.
      Since the latest archived note's mod time must be prior to the archiving time itself.

    - Archiving could be in date ordered file names, say with a year designator.
      Archiving could be in LogDB and text format.
    - Archiving could be used on active notes only, or could grab all edits for an active note
      (this would result in out of date ordering for non Journal entries.

