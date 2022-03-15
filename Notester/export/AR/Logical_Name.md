# AR.Logical\_Name --

    [TODO]
     <<>>
     <<>>


    TRACE
     getEntityByUname
     getUnameById
     getIdByUname

    TRACE ENTITY
     getUnameString()    In logical name resolver
     getUnameString(.)    
     


    PATH
    - integrate ln and lnr .java
    - 3 methods uname only.
    - add lname support
    - test on click tester.
    - IN PARALLEL: cvt main system to 3 methods


    INPUTS:      id <--> entity <--> lname <--> lname-string
    - Trace ID
    - Trace Entity
    - Logical Name
    - Cooresponding Entity


    LOGICAL NAMES SPECIFIC STUFF IN TRACE
    - getLogicalNameList(spec, trace)  ???


    TRACE
    - getEntityObject(spec)                  id;coorespondingEntity;lname
      - id(in this trace) ->  entity
      - entity            ->  logical-name -> entity (in-this-trace)
      - logicalName       ->  entity (in-this-trace)
    - getEntityId(spec)
    - getLogicalName(spec)    




    Just ID?
    - hasEntity(id)
    - getEntityOrCreate(id)
    - setEntity(id,entity)
    - deleteEntity(entity)


    OLD STUFF
    - getEntityByLogicalName
    - getLogicalNameById
