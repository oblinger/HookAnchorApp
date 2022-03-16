# Prelim.Bio-examples -- Bio-examples

    General Syntax for Plausible Knowledge:
       []
       
    Spaces:
               <--    Implication (No space)
               <      Choice
               <+     Overriding Combination   NO!
               <#     Parametric
    Predicate OverridingSpace.
    TOP-LEVEL THEORY
    SameFold(k,u) < Best(k, KnownSeqs, Lambda(x).AlignedScore(x,u))
    Best(best,list,fn) <--
      ForAll x in list  fn(best) >= fn(x)
    AlignedScore(k,u) < ResSimFn(), GapCostFn(), AlignAlg(), AlignmentScore()
      AlignmentScore( AlignAlg(k, u, ResCostFn(), GapCostFn()) )
    GapCostFn(Rk,Ru,len) < GAP, GOP
      GOP + len * GEP
    GOP in [0,1000]
    GEP in [0,100]
    GapCostFn() <# GAP, GOP
      GOP + len * GEP
      GOP in [0, 1000]  GEP in [0, 100]
    ResSimFn(Rk,Ru) < Choices(ResSimFn), WeightedAverage()
      Fn = WeightedAverage(Choices(ResSimFn)); Fn(Rk,Ru)
    WeightedAverage(FNi) <# Wi
      |Wi| = |FNi|, Wi in [-Inf,+Inf]
      Lambda(x_vec).(Sum j=1 to |Wi|   Wj * FNj(x_vec)) <<>> Sum j=1 to |Wi|  Wj
    Choices(RefinementVar) <-- ...
    ResSimFn(Rk,Ru) < ContextFn(), CtxCostMatrix(ContextFn)
      ... context fn based array lookup
    ContextFn(res) < SolventExposure(), ExpLow, ExpHigh
      0 if SolventExposure(res) <= ExpLow
      1 if ExpLow <= SolventExposure(res) < ExpHigh
      2 if ExpHigh <= SolventExposure(res)
    ExpLow  in [0,50]
    ExpHigh in [0,150]
    ContextFn(res) <  SecondaryStructure, Chirality, StructureIndex, ...
    ContextFn(res) < Set Aggrigation
    CtxCostMatrix(ContextFn) < Data, APM(Data,ContextFn)
    METRIC(ContextFn) < Data
      RMS-Diff(APM(Data,ContextFn), APM(Data, NullCtxFn))
      
      
      
