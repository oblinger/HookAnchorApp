# Bio.Check-Trends-In-Table --

    >   Check if expected bio trends show up in data. 
    *   Reduce tables by residue size (S,M,L), and check: 
     
    -   In Alpha small <--> large should be less frequent. 
        Result: Partly holds.  When Lg is there, its there for a reason 
        Alpha   Sm->Lg 15.2   Lg->Sm 31.4 
        Coil    Sm->Lg 15.2   Lg->Sm 38.1 
     
    -   In Alpha table check for small & linear side chains. 
        Result:  Fails 
        Alpha  Sm 39.1   Med 39.1   Lg 21.9 
        Coil   Sm 52.2   Med 30.1   Lg 17.1 
     
        Beta   Sm 44.6   Med 32.3   Lg 23.1 
        Turn   Sm 55.3   Med 28.1   Lg 16.5 
     
    -   Check if more diff. sized residues match when exposure is high 
        Yes.  Buried  Sm->Lg 14.3   Lg->Sm 31.8 
              Exposed Sm->Lg 15.2   Lg->Sm 39.8 
     
     
     
    Visually Inspect if about twice as many small residues occur in coil vs. alpha 
