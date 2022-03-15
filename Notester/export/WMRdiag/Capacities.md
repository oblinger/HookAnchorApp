# WMRdiag.Capacities -- Sizes and capacities

    WORST CASE 
    utt   = 20  Words per min (assume abbreviated panda response) 
    min   = 15  Min per day 
    day   = 5   Days per week 
    week  = 16  Weeks of collected data  
    class = 30  Students per class 
    dsp   = 10  Displays per screen 
     
    20*15*5*16 = 24K  rows per student 
     
    x10 = 240K  (For one per student screen) 
     
    x30 = 7.2M  (For one per book screen) 
     
     
    LIGHT CASE 
    utt   = 10 
    min   = 15 
    day   = 2 
    week  = 4  (just recent info) 
     
    1.2K  x10 = 12K 
     
     
     
    MEMORY CONSUMPTION 
    - each row: 20 bytes 
    - 24K  x30 x20 = 14.4M 
    - 1.2K x30 x20 = 720K 
     
     
     
     
     
     
    Counting speeds 
    28M/sec   C++ 
    .2M/sec   Java 
     
     
     
     
