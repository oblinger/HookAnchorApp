# Usenet Connection --


    >>> 
    How a standard USENET message can serve as forum message.
    - A usenet message is also a forum message if it contains a 
      Forum identifier line.  This is any line in the post body or subject that
      begins with 'ForumTopic', 'ForumPosition', or 'ForumSupport'. 
      The line's format is:  ForumXxxxx <<>> [parent-node]
      Examples:
        ForumTopic     Gun Control [Social Issues]
          - This defines and summarizes a forum topic called node name
        ForumPosition  Carrying guns reduces crime
          - This defines a position forum node call 'node name'
        ForumSupport   [Carrying guns reduces crime]
          - This supports the specified node
        ForumMessage   [Gun Control]
          - This is a discussion point under 'gun control'

    - Message field are inferred from the usenet post as follows:
      - MESSAGE ID     is the post's usenet ID. ???
      - MESSAGE AUTHOR is the post's from email address.
      - MESSAGE DATE   is the post's date (as long as its close to real date)
      - MESSAGE TITLE  is the post's subject
      - MESSAGE PARENT inferred from the [] in the forum line or 
                       the usenet ReplyTo structure
      - MESSAGE TYPE   is extracted from the identifier line
      - MESSAGE NAME   is extracted from the identifier line
                       (strip non alphanumeric)
      - MESSAGE SCOPE  inferred from topic, parent, thread, or newsgroup, in
                       that order.
