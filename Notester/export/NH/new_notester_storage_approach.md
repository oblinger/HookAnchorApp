# NH.new\_notester\_storage\_approach --

      best choice?
    - NotesterDB                 (adaptor just adds 4 access methods + Pluggable output filters (for HTML etc.))
      LogDB extends AbstractDB   (set method is private)
      NoteOps.java               (singleton contains a NotesterDB; high-level note ops from Notester.java)


    - Build seperate NotesterDB.java class  create, delete, rename, update, search, getNote(int), etc.
      - Controls lower level to achive compacting
    - Build seperate NotesterStore.java with get/set implementation  (Allows plugging many output forms) 
      - Seperate out Log read and log write

    * Wayback; DB.#34 numbering; search; Auto compactify notester.log

    Naming:   #7923_Full.name.of.note
              @ShortName                This is the virtual note whose children are all notes with specified short name

    ------

    Backend Methods:
    - Create
    - Rename
    - Update
    - Delete

    Implementation methods
    - Set (null used to delete)

    ------


    Disk Based Hash Table
    - Bucket Hash Table with in memory bucket ptr table
    - Two file seeks per read
    - Limited in-memory element cache  (maybe handled at higher level)
    - Both table and elements caches are write thru caches
    - Hash values are either all Objects or all ints
    - Bucket format: int obj1 int1 obj2 int2 ....
    - Max size 4gig
    - Auto compacting   (maybe this is handled at higher level)





    Underlying structure
    - Update Log (like current format)
    - Toplevel int hash points to active entries in log
    - History int hash points to history of changed entries
    - Could support multiple Log files 
