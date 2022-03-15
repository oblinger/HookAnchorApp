# Columbia -- Columbia course notes (Spring 2003)


     <<>>
     <<>>
     <<>>
     <<>>
     <<>>


     <<>>
     &D <<>>
     &W <<>>

    ---------------
    - No slides.
    - Soln for HW #3.
    - Check for Duda & Hart
    - Compute Averages



    final probs
    Problem 2 in the final and HW 5
    Problem 3 in the final can be restated for k-means rather than 1 nearest neighbor.
    Cheers,







    Let D be a uniform distribution of points over the unit square in the
    cartesian plane from (0,0) to (1,1).  All points below the y=x^2 curve
    are labeled positively and all points above the curve are negative.

    Formally elements of D are triples:  < label, x, y >    where
       0 <= x <= 1
       0 <= y <= 1
       if y < x^2 then label = '+'  otherwise label='-'


    Assume L is the zero-one loss function, that is assume that   L(a,b)=0  if a=b  and  L(a,b)=1 if a!=b.


    Find the split on the x-axis that minimizes the loss function L for problem distribution D.
     
