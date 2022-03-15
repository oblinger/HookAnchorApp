# AR.LogicalNames --



    Has To Be There
    - Class of win object is same
    - Process .exe is same
    - Root window is same


    Soft Constraints
    - Same positionIndex which is computed among all objects within a root that are of same type
    - Text
    - Size Hor, Size Ver
    - Pos Hor, Pos Ver ????
    - Class tree is same  (fraction of nodes at each level of the contained tree that are same)







    -----------------
    - distance(entity, entity)  distance based on trajectory

    Binder.Group
    - Median (entity with smallest avg dist)
    - Set of entities whose avg distances are small
    - Set of alts for each environment


    Binder.update(entity, allowCreates)
    - Compares entity against all others.
    - Determines *the* lGroup for the entity
    -   also includes it as alts for other groups???
    -   creates new lGroup if none fit and allowCreates  (Only done of full traces at train time)
    - Can avoid calling on entities that already have an lName to increase speed




    Binder.getLName(entity, world) --> lName
    Binder.getEntity(lName, world) --> entity

    Binder.addLNameListener(object, lName, world)
