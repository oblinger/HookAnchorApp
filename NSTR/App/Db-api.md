# App.Db-api --

    DATABASE DESIGN 
     
    SPARSE DATA TABLE 
    This contains info about each the sparse skill information for 
    each employee.  Each row contains three values: 
     
       Employee-ID   Feature-Name  Value-of-feature-for-this-employee 
     
     
    HIERARCHY TABLE 
    This table contains a description of the parent-child relationships 
    in the space of features.  Each row contains: 
    (It may also have back pointers to the children for each node too) 
      
       Name Parent Description-String 
     
     
     
    API DESIGN 
     
    <<>>** Adds associations to a mapping that express all sparse features for a  
     *  given user (id).  In the mapping each feature is associated with its value. *<<>> 
    public Map addSparseFeatures (Map dest, String id) <<>> 
     
     
    <<>>** Assigns 'value' for a given 'feature' on a single user 'id'. 
     *  If value is null then that feature's previous value is removed. *<<>> 
    public void setSparseFeature (String id, String feature, String value) <<>> 
     
    <<>>** Scans the new feature map looking for features that have changed from 
     *  the old map, and setSparseFeature to update the changed features. 
     *<<>> 
    public void setSparseFeatures (String id, Map newFeatures, Map oldFeatures) <<>> 
     
     
    <<>>** Selects users by specified feature and value.  If value="*" then 
     *  select all users that have any values specified for that feature. *<<>> 
    public selectBySparseFeature (String feature, String value) <<>> 
     
     
    <<>>** Returns a mapping with an entry for each column in that users row 
     *  in the database.  (operates on previously computed result set.) *<<>> 
     public Map[] getResultMaps () <<>> 
        HashMap m = new HashMap(); 
        addDenseFeatures(m, ...) 
        return m; 
    } 
     
    getResultSet()  <<>> should simply return precomputed resultSet 
    applyConstraints() <<>> Should build and run SQL stmt and place result in 
                       <<>> class variable. 
     
    AQa 
     
     
     
     
     
