# Bio.Atm-ctx -- Contact neighbor context

    Terms: 
       Contact()  is true for two contacting atoms, a & b. 
       Polar(Atom)     is true for polar atoms. 
       #PP(Res)        Number of polar-polar contacts for a res.  Also #NN & #X 
       Res(Atom)       returns the residue an atom is in. 
       ResType(x)      returns the amino-acid name for a residue or an atom. 
       Prot(x)         returns the protein containing the residue or atom, x. 
       Distance(a,b)   3D physical distance between atoms a & b 
       Radius(a)       Distance of an atom from the C_A atom. 
       Asimuth(a)      Angle an atom makes with the C_A radial. 
       Direction(a)    Orientation angle wrt the C_A - Cb line. 
       ContactSet(Res) Set of tuples describing all contacts made by atoms in Res. 
        
     
    Definitions: 
     
    Contact()  iff  Prot(a)=Prot(b)=P and Distance(a,b) < 4.0 
     
                      T if a in <<>> 
    Polar(a)        = F if a in <<>> 
                      ? otherwise 
     
    #PP(Res)        = Count  where Contact(), Res(a)=Res, 
                      Polar(a), and Polar(b).  Similarly #N, #PN, #NP, and #X. 
     
    Radius(a)       = Distance(a, C_A(Res(a))) 
     
    Shell(a)        = Round(Radius(a) <<>> ShellWidth) 
    V_Shell(a)      = 0 if Radius(a) < ##, 1 if Radius(a) < ##, ... 
     
    Steps(a)        = Number of links from C-alpha 
    Step_Dist(a)    = Number of steps straight away from the C_A. 
     
    Azimuth(a)      = [0..180] size of the angle formed between the atom, and 
                      the C_A radial.  The C_A radial is a ray originating 
                      at the C_A and extends away from the backbone.  This ray 
                      lies on the bisecting line for the angle Nb - C_A - Cb. 
                      Nb, and Cb are the backbone nitrogen, and carbons 
                      adjacent to the C_A in the backbone. 
     
    Direction(a)    = [0..360] Size of angle formed by a - C_A - Cb when all are 
                      projected into the plane perpendicular to the C_A radial. 
     
    CtxTuple() = ), ..., CtxAttrN()>, where  is 
                      a contact pair, and CtxAttr1, ..., CtxAttrN are the 
                      context attribute functions. 
                       
    CtxSet(Res)     = <<>> 
                      CtxSet is a multi-set. 
     
     
     
    >Ordinality Contexts 
      Split #PP(), #NN(), and #X() into three categories: 0,1,>=2 
      27 contexts 
      OrdCtx(Res) = [0..26] 
      OrdCtx%(type, Ctx)       Fraction of counted res of type 'type' w. context 
                               'Ctx'. 
      Similarity(K_Res, U_Res) Monotonic & + wrt CountsOrdCtx(U_Res, OrdCtx(K_Res)) 
     
     
     
    >Independent Contact Contexts 
    - Radius   5 
    - Asimuth  5  
    - Polarity 3 
      Contexts: 75 
      DB-size:  200x200<<>>20 = 2000 per residue 
     
      IndCtx()     = [0..n] Ctx for contact pair: R x A x P 
      IndCtx%(Type,Ctx) = Fraction of counted res of 'type' with 'context' 
      Contact_Cost(Type,) Monotionic & - wrt IndCtx%(Type,IndCtx()) 
     
      Cost(K_Res,U_Res) = Sum over  where Contact() and K_Res=Res(a) of 
                             Contact_Cost(ResType(U_Res), ) 
     
     
    Combinatorial Contexts 
    - Polarity 3 
    - Radius 3 
    + Number of values 3 
     
    3^9 ~ 30000 contexts 
     
     
    >Hybrid Contexts 
      Linear combination of ordinality & Indepnedent contexts. 
     
     
    >The Ordinality Context Conversion Function 
     
    - Each res falls into one OrdCtx and has one ResType. 
      OrdCtx%(Type,Ctx) = Prob( Type=ResType(res) ^ Ctx=OrdCtx(res) ) 
     
      OrdCtx%(Type,Ctx) = Prob( Type=ResType(res) ^ Ctx=OrdCtx(res) | Good(res) ) 
     
      Prob( Good(res) | Type=ResType(res) ^ Ctx=OrdCtx(res) ) = 
            OrdCtx%(Type,Ctx) * P(Good) <<>> P( Type=ResType(res)^Ctx=OrdCtx(res) )  
     
     
    >The Independent Context Conversion Function 
     
    Each  pair  
     
     
       
    Equations: 
     
    Cos T = A.B <<>> (|A| |B|) 
     
     
     
    OLD: 
     
       Res_Ctxs_1       X <0, ..., 4> 
       Ctx_Matrix(Res) <<>> Res_Ctx -> <0,1,2> 
     
       #N(Res)         Number of nonpolar contacts for a residue 
       #PN(Res)        Number of polar - nonpolar contacts 
       #NP(Res)        Number of nonpolar polar contacts 
       #X(Res)         #PN() + #NP() 
     
