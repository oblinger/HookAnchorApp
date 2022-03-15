# Forum.Message\_Tree --

    A message tree is computed from a stream of messages.
    Each message in the stream is associated with some NODE in the message tree.
    All message types are either amalgamating, or separating, this designation
    is used to separate message out into a tree.


    The following computation derives the message tree:
    - Any message that is not associated with any other message is associated
      with the usenet group or forum note where it was posted.  In either case
      it is assigned to it's own node, and that node is a child of relevant
      usenet group node, or forum node.

    - Any message with an amalgamating type associated with another message inherits
      that message's node.

    - Any message with an seperating type associated with another message is assigned
      a new node that is a child of the other messages node.



    AMALGAMATING TYPES
    - (all augmentative types)  Summary, Varient, Edit, Support, Dissention
    - Disscussion


    SEPARATING TYPES
    - Topic
    - Position
