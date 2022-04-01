# NH.Streams\_Proposal\_9\_2005 --


    * All log notes in single stream
    * Single parent sub-typing in stream
    * Second hierarchy is by project-sub-project
    * Stream ordering is done by last edit time
    * Lists are treated like non-stream items, they are only logged when they are deleted.

    STREAM LOGGING
      WHERE  * Stream entries are logged on seperate logs for each supertype of the entry
             * Stream entries are logged on seperate logs for each project and superproject for the entry
      WHEN   * Stream entries are logged when they are manually archived.
             * Stream entries are logged when they are deleted.
             * They are temporarily logged everytime the archive is viewed.
             * Non-stream (reference) notes are optionally logged when they are deleted.
            

    VIEWING STREAMS
      * Recent streams entries are viewed by search
      * Full streams are viewed as a text file anytime
