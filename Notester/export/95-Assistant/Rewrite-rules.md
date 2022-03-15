# 95-Assistant.Rewrite-rules -- Mail translation rules

    $@ibm research  -> $ 
     
    Xlator: 
      if pattern begins or ends with constant, then test & clip it 
      if pattern is  then match all 
      if pattern is  constant  then search and match all 
     
    Returns "" or match string if '$' wild is present 
     
     
     
