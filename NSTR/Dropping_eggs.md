# Dropping\_eggs --

    You have a 100 story building and two eggs.  These are  
     especially strong eggs.  There is some floor below which  
     the egg will not break if dropped.  What is the worst case  
     upper bound on the number of drops you must make to 
     determine this floor? 

     THE SOLUTION (Done by my Cousin Ron and me)


     Let N the number of drops you need to find the first floor 
     that breaks eggs.  Go to the Nth floor and drop an egg.  If 
     it breaks you have N - 1 more drops to test the N - 1 floors 
     below.  If it doesn't breaks, go to the N + (N - 1) floor 
     and drop the same egg.  If it breaks you have N -2 drops 
     left to test the N - 2 floors between Nth floor and 2N -1 
     floor.  By a similar analysis you approach the top of the 
     building with

     N + (N - 1) + (N - 2) + (N - 3) + . . . + 1 <= 100

     N ( N + 1 ) <<>> 2 <= 100

     This is a quadratic equation which yields N <= 13.65.  If 
     N = 13 you can only analyze a building of 91 floors.  It 
     takes N = 14 to test a 100 floor building.

     Final solution.  You go to the 14th floor and drop an egg. 
     If it breaks you have 13 more drops to test floors 1 to 13. 
     If it doesn't break you go to the 27th (14 + 13) and drop 
     the first egg again.  If it breaks you have 12 drops left 
     to test the 12 floors above 14 and below 27.  You continue 
     up the building until you reach the 99th floor 
     (14+13+12+11+10+9+8+7+6+5+4) with three drops left.  If it 
     breaks you have three drops left to test the three floors 
     between the 95th floor and the 99th floor.
