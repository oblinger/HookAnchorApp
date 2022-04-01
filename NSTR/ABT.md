# ABT --

    [TODO]
    w  Paper outline
    a  Talk with Vit about experimental suppport
    n  find deadline
    -  Write the math section



    <<>>

    NOTATION
      Procedure         --  Fixed procedure used to track users behavior during playback.
      saPair_i          --  the ith state action pair during playback.
      PROC              --  Set of procedure steps, s, in the current playback Procedure.
      PROC0             --  Set of procedure steps, PROC, plus the special '0' step denoting a saPair that is off track (does not associate with any step, s)
      Pon_i(s)          --  Given that system is currently tracking, this is the probabilty that saPair_i 
                            associates with step s in PROC0.  P_t,i is a pdf over all s in PROC0.
      Pon_i('0')        --  Probability that saPair, i, is not associated with and step in the procedure, e.g. user is off track
      Ptrack_i          --  Max_s<<>>    where s is in PROC.   Probability that user is on track and we are correctly predicting the step they are on.
      TrackingThreshold --  [0,1]  threshold specifying minimum support required to conclude user is tracking within the procedure.
      TRACK_i           ==  '0' if Ptrack_i>>    where s in PROC
                            (this is the output of the algorithm.  returns '0' if not tracking user)


    INPUTS

      Psim_i(s)         --  Pdf over all steps, s, in procedure. Prob that saPair associates with step 's'.

      Pnext_i(f,s)      --  is Pdf over all s in PROC.  Probility of jumping (validly or invalidly) to 's' given in step 'f'

      Pjump_i(s)        --  Pdf over steps, s, where s&f in PROC
                            (rejoinder point if last tracked step was 'f'.  should be a spike at next step, plus smaller splike at 2 & 3 steps later)

    RECURANCE RELATIONS

      Pon_i*(s)         ==  Psim_i(s) * [ Pon_i-1('0') * Pjump_i-1(s) +  Sum_f <<>>  ] where f in PROC and s in PROC0

      Prejoin_i(s)      --  Given that user is off track before saPair_i, this is the a priori probability that user rejoins at step s.  s in PROC.
                        ==  Pjoin_i-1(s)                      iff TRACK_i == '0'
                            Sum_f <<>>   iff TRACK_i != '0'  


    INPUTS
      saPair_i
      Procedure

    DERIVED INPUTS
      Procedure -> Psim_i(s)
      Procedure -> Pjump(f,s)
      Procedure -> Pvalid_i(f,s)

    RECURENT RELATIOINS (COMPUTED VALUES)
      Pon_i(s)
      Pjoin_i(s)

    OUTPUT
      TRACK_i


      









    BUGS:
    - Check that returned step is still part of procedure.
