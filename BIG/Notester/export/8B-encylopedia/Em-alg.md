# 8B-encylopedia.Em-alg --

    Algorithm: 
     
    Variables: 
      Pages                  Set of web pages 
      Cats                   Set of category nodes 
      Links:page-><<>>     Maps each page to it set of out links 
      Map:page->cat          Maps each page into category node 
      Graph:cat-><<>>       Category out links 
      CatRep:cat->rep        Representative for docs in category 
     
    INPUT:  Links 
    OUTPUT: Map, GRAPH 
     
    InduceStructure(Links) 
       get Links 
       set OldMap <- 0 
       set Map <- GenerateRandomMapping() 
       while dist(Map, OldMap) > threshold 
    1.    CatRep <- UpdateCatRep(CatRep, OldMap, NewMap) 
    2.    Graph  <- UpdateGraph(Links, Map) 
          OldMap <- Map 
    3.    Map    <- UpdateMap  (Links, Graph, CatRep) 
       return Map, Graph 
     
    UpdateGraph(Links, Map) 
       Graph = NullMapping 
       for c in Cats 
          MarkLinkedPages(c) 
          for d in Cats 
             if ProbableLink(Links, Map, c, d) then 
                Graph(c) <- Graph(c) + d 
       return Graph 
     
     
    ProbableLink(Links, Map, c, d) 
       count <- 0 
       for p in ReverseMap(d) if marked(p) then count <- count + 1 
       let pairs <- |ReverseMap(c)| * |ReverseMap(d)| 
       if (count/pairs >= threshold) then return true 
       else return false 
     
     
    UpdateMap(Links, Graph, CatRep) 
       for p in Pages 
          Let Scores() = 0 
          <<>> Assign credit for in links 
          for pIn in ReverseLinks(p) 
             let cIn <- Map(pIn) 
             for c in Graph(cIn) 
                Scores(c) <- Scores(c) + InCredit 
          <<>> Assign credit for out links 
          for pOut in Links(p) 
             let cOut <- Map(pOut) 
             for c in ReverseGraph(cOut) 
                Scores(c) <- Scores(c) + OutCredit 
          <<>> Add similarity score for each category 
          for c in Cats 
             Scores(c) = Scores(c) + MatchScore(CatRep(c), p) 
          Map(p) = c where Scores(c) > Scores(x) for all x 
       return Map 
     
     
     
     
     
    MapClear(map) 
    ClearEntry(map,x) 
    ClearReverseEntry(map,y) 
    Map(map,x) 
    ReverseMap(map,y) 
    Next(map) 
    Assign(map, x, y) 
    Remove(map, x, y) 
    Test(map, x, y) 
     
     
     
