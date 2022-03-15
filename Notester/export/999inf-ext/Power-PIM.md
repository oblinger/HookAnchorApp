# 999inf-ext.Power-PIM -- General personal info manager

    GOALS:  Widely Configurable; Modular; Transformational 
     
     
     
    DESIGN POINTS: 
    * Each node is a record with: 
      - unique-id 
      - name 
      - one-line title 
      - body-text 
      - parent 
      - children an ordered list of  
      - future/past links for entry 
      - revision timestamp 
      - author 
      - default view (list of transforms & view) 
     
    =============== 
    ===  VIEWS  === 
    =============== 
    * By default each of the following views only show the most 
      recent version of a each node (nodes with no future). 
     
    COLLAPSING TREE VIEW -- Nodes displayed using some or all of the following: 
      - title line 
      - body text 
      - children 
      - with child indentation 
     
    HYPER LINK VIEW -- Each child is a hyperlink 
     
    LIST VIEW -- Children are viewed as a selectable list 
     
    FORM VIEW -- Node's fields are substituted into a form node, whose body 
    is used for displaying the node. 
     
    ================================= 
    ===  MAINTAINANCE PROPERTIES  === 
    ================================= 
    These are properties of a node that are automatically maintained. 
     
    ALPHABETICALLY ORDERED -- Children are alphabetically ordered by one of 
    their fields. 
     
    CRONOLOGICALLY ORDERED -- Children are ordered by an ordinal field. 
     
    MANUAL ORDERING -- Children are ordered manually by author.  Nodes added 
    through transformation are added at the beginning/end. 
     
     
    ==================== 
    ===  TRANSFORMS  === 
    ==================== 
    A transform computes one node from another.  There are a number of properties 
    that this relationship may possess: 
    * The transformation may be one-time, manually-refreshable, or auto-updating. 
    * Changes may be computed incrementally 
    * It may be reversable 
     
     
    COPY TRANSFORM -- Makes a duplicate of a node whose fields are linked 
    to the original.  Links between specific fields and children can 
    be established or broken.  By default only the children are linked. 
     
    MERGE TRANSFORM -- A node created whose children are the grandchildren 
    of the input node.  Incremental.  Reversable. 
     
    FILTER TRANSFORM -- A node containing a subset of the children of the 
    orignial node is created.  Incremental.  Reversable. 
     
     
     
    SPECIFIC TASKS: 
    - address book 
    - hierarchical todo list 
    - Combined todo list groups  
    - Project task tree linked with todo list linked with calendar 
      * Setting node as active recursively makes children visible on todo 
     
     
