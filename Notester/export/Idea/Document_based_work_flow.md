# Idea.Document\_based\_work\_flow --




    State/Triggers:
    - fields filled in
    - person processes document (fills in a role-specific "done" box)
    - pushed documents from external sites
    - 


    Actions:
    - Send document to person for update.
    - Instantiate document
    - Send document to external sink
    - Accept documents from external source





    DataStructures:

    - document with set of named fields

    - Field:
      - Field name
      - Permissions:   Read/Edit    (state/trigger based)
