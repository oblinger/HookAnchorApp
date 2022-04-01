# Old-Prj.DB-design --

    OVERALL DB-API DESIGN 
    - Rel-DB (DB Open/Select. Operations that select a set of users as result-set) 
    - Sparse (DB get/set.     Operations over result set; creation of new rows) 
    - Hierarchy (DB skill hierarchy access) 
     
     
     
     
     
    PERSON DB 
    - one row for each person 
    - inverse index on specified columns 
     
     
    TAXONOMY 
    - one row for each column in PERSON-DB 
     
     
