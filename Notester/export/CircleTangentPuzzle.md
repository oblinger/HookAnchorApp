# CircleTangentPuzzle --

    Alex, 
     
    Great puzzle!  I really didn't think about it until I got your message. 
    Then I blew an afternoon on this puppy; it was hard.  I really should have 
    remembered that the perpendiculars of a triangle meet at a point 
    immediately, but my high school geometry is quite shaky after years of 
    disuse.  Great puzzle, thanks! 
     
     
    I am very interested in your solution to my puzzle.  As I recall, I had 
    reduced it to the limit of some integral.  If the integral diverged in 
    the limit then there were ``too many'' circles to spead out. 
    I couldn't solve the integral, and I couldn't bound it w. something 
    I could solve. 
     
    Is their a clever way to avoid the complex integral or is there a lot of 
    calculus needed?  I am really interested in your solution, but 
    I don't want to spoil it for myself, someday I will sit down and solve it. 
    If you get a chance, write your solution up, and send it.  I have 
    some buddies who are also interested in this one. 
     
     
    New Puzzle: (not as tough as the last one but still fun) 
    - Some whole numbers can be expressed a the sum of a sequence of consecutive 
      natural number (eg. 5+6+7 = 18). 
    - Give a closed form characterization of those numbers that 
      can NOT be expressed in this way.  Prove it. 
     
     
    If you have another puzzle I'd love to hear it.  After your last one I have 
    promised myself that I would not spend more than an hour on any single puzzle 
    until I finish my thesis.  Still if you another, I could keep it till I have 
    more time. 
     
    My decision to go to IBM was a hard one.  I really feel that I would make 
    a bigger splash faster in your group at SGI.  In the end it was the 
    long term possibility for personal growth that drew me to IBM. 
    Salary was not a strong factor in my decision.  When you factor in cost of 
    living, and annual bonuses IBM's offer was stronger, but both offers were 
    very good.  I'd be happy to talk specific numbers for salary at both locations, 
    but is that appropriate?  Specifically, is SGI open about their salary offers 
    for incomming people?  If so, just let me know, and I'll give you the details 
    for all of the companies I interviewed with. 
     
    In any case, I wrote up my construction of the tangent line below, please 
    send your circles in the plane proof & another puzzle if you have one. 
    Keep in touch. 
     
    Cheers,  
    Dan 
     
     
     
     
     
     
     
     
     
     
    Puzzle: Given a circle, a diameter line intersecting the circle at A and B, 
            and a straight edge construct a tangent to the circle at B. 
     
    Proof: 
    - Use lemma 1 to construct 5 perpendicular by pigeon whole principle 
      one side of the circle will have at least three perpendiculars:  
    - WLOG assume the lines are ordered circle followed by J, K, L 
    - Pick two points on J, and two on L, use box lemma to get 
      equidistant points on K. 
    - Use inverse box lemma, point B, equidistant points on K and parallel L 
      to construct parallel line thru B, this is parallel to K which is 
      perpendicular to AB so it is a tangent to the circle at B. 
      QED. 
     
    LEMMA 1: Constructing a perpendicular to AB: 
    - Choose arbitrary points U and L on the circle above and below AB. 
    - Extend the lines AU and LB until they meet call that point C. 
    - Extend AL and UB to a point called D. 
      AUB and ALB are right angles by theorem. 
      They both are perpendiculars of the triangle ACD, and since 
      all three perpendiculars of a triangle meet at a single point 
      the line AB is the third perpendicular, thus AB is perpendicular to CD 
     
    THE BOX LEMMA:  Three parallels constructs colinear equidistant points 
    - Given three parallel lines ordered in space A, B, C with 
      three non-colinear points: A-low, B-mid, C-low on each line respectively. 
    - Construct C-high by extending A-low thru B-mid 
      Construct A-high by extending C-low thru B-mid 
      Construct B-high by intersecting A-high thru C-high with line B 
      Do the same to get B-low from A-low thru C-low intersected with B 
    - Notice by lots of congruent triangles that  
      B-low to B-mid has the same length as B-mid to B-high 
     
    THE INVERSE BOX LEMMA: colinear equidistant points constructs third parallel 
    - Given two parallel lines B and C and three points on B as above 
      where B-low to B-mid is same length as B-mid to B-high and 
      a point A-low. 
    - Construct C-low by extending A-low thru B-low 
      Construct C-high by extending A-low thru B-mid 
      Construct A-high at intersection of C-low thru B-mid and C-high thru B-high 
    - Notice by the same triangles as above that the line A-low thru A-high  
      is parallel to lines B and C. 
     
      
     
