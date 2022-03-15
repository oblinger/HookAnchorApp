# ABL --



    ----
    <<>>
    <<>>
    <<>>    o3+m7      fp8035
    <<>>
    <<>>?id=fp8035

    submitted:  <<>>?CFID=20624&CFTOKEN=28668508
    ----------
    Reviewer #1:    Typo in para before Defn 4: "adding nodes an?? edges"


    REVIEWER #2:
    I found some of the definitions unclear. For example, if an alignment is a 1-1 correspondence (def 1), 
    why is it necessary to repeat the 1-1 constraint using the term 'deterministic' in proposition 2? It seems redundant. 
    >>> VIT: I recommend striking:  "therefore, the alignment is deterministic;"    ok?

    I am not convinced about proposition 3. It seems that the reason that user edits are respected by ABL is the requirement 
    for subsequent changes to be augmentations, not because they respect maximal consistent subsets of earlier data 
    (which after all was true of the pre-edited procedure). I think this is useful terminology to introduce, but you 
    may want to reconsider claiming these as sufficient conditions for a solution.


    >>> I agree that we do need future changes to be augmentations, but we need to have the points (a) and (b) in prop 3
        in order to guarantee correctness of generalization.  
    >>> VIT:  I am bothered by the fact that prop 3 does not mention training data prior to epsilon_h.
              I know we expect earlier data to be misordered, but we still use it in generalization.
              I do remember how or if we say that.


    >>>



    ----
    Dear Author,

    Please remember to sumbit your paper, "Augmentation Based Learning: Combining Observations and User Edits for Programming by Demonstration", for publication in the iui  proceeding.

    SAVE THIS EMAIL: Your IUI'06 paper id# is in the subject line above, and a link to submit your paper is below. Note the prefixes before your paper ID#s that relate to your paper type:
    fp=full papers, sp=short papers, inv=invited, w=workshops and t=tutorials.
    Formatting and submission instructions are available at:
    www.sheridanprinting.com/typedept/iui.htm

    Kindly adhere to the page limits and the submission deadline. Kindly submit on or before November 21st, 2005. The submission site is ready. Early submissions are encouraged and will be appreciated.

    It would be very helpful if you can complete, sign and fax your ACM
    copyright or permission form immediately after submitting the requested files for your submission by utilizing the link created when submitting and contained in the confirmation message. The title, authors and paper id information will be completed on the form by the submission system. So please be sure to enter the information in correctly.

    For paper submissions and additional information please use the following site: <<>>?id=fp8035

    Thank you,

    Lisa Tolles
    Sheridan Printing
    Email: iui@sheridanprinting.com




    ----
    Conference: IUI2006
    Track: Long papers


    Paper ID:  8035
    Title: Augmentation Based Learning: Combining Observations and User Edits for Programming by Demonstration
    Authors: Daniel Oblinger, Vittorio Castelli, Lawrence Bergman

    Dear Authors,

    All your reviews are available to you through the CRS that you used to submit your paper. Log on, click on your author role and you will now be able to see the reviews of your paper. 

    These reviews will assist you in improving your paper. Remember the deadline for uploading the final version is Nov 21, 2005 and that is a hard deadline. Also format and upload instructions are at <<>> The instructions are still being constructed so we suggest you wait a day to use that site.

    ----





    Examples for eval section

    <<>>

    Conclusions

    In the discussion/conclusion section I was thinking about listing the contributions of this paper
    and then an overall conlusion statment followed by supporting points for that overall 
    conclusion.  What are the points to be made in this last section?  Is this breakdown
    even the right breakdown for the end of this paper?



    CONTRIBUTIONS OF ABL AND THIS PAPER
    - A programming approach that combines PBD with conventional programming
    - A new learning algorithm that supports this combined form of programming,
      along with an unconventional decomposition of scripts that separates flow logic 
      from action variablization.
    - One of the first PBD approaches that can take advantage of multiple demonstration
      where the steps of those demonstrations have not been aligned by the author of the demonstrations.
    - Even if we ignore the editing it support ABL is the only multi-demonstrastion technique that can produce viewable procedures.
    - (We could also talk about demonstrating loops and conditionals, but this was
       not really the focus of the paper, so my intuition is to leave that out)
      

    THE PRIMARY CONCLUSION
    The central advangte of the ABL approach is that the 
    author is able to opt for either traditional explict
    programming, or by-demonstration programming, depending
    on which is most convenient at the time.
    This affords ABL with many of the advantages 
    of both of these two complementary approaches to programming:

    -- The author may think in terms of concrete action sequences
       when programming by demonstration, rather than the generalized 
       actions and flows required for traditional programming. 
    -- The author does not need design a demonstration sequence to force
       an explicit change in the script.  If they know what change 
       is required than can simply perform that edit directly.
    -- We have shown that the author can alternatve between these
       two programming styles with ease.  All knowedge gained
       by editing is immediately availble for subsequent by 
       by-demonstration programming and vice versa.

    then one sum-it-all-up-sentence at the end of this list, and that is it.






    ----
    Evaluation section

    - Support that the algorithm works
    - Support that the interleaved editing works.








    <<>>

        ------------
        INTRODUCTION
        ------------


    Motivation:  Disuss naturalness of "by-demonstration" transfer of knowledge.  This
                 suggests it as a programming methodology.  Such an approach looses 
                 some of the affordances of traditional programming (explicit control)

    Problem:     Combining by-demonstration programming with explicit programming in a system
                 that allows the two sources of input to be interleaved.

    Solution:    A new learning algorithm whose hypothesis space and training data can be updated
                 to follow the modifications made by explicit programming interleaved with the
                 by-demonstration learning.

    Benefit:     These methodologies provide complementary benefits, our combination affords us
                 the speed and simplicity of by demonstration programming, with the explicit 
                 control of traditional programming.
                 


        ---------------------------
        AUGMENTATION BASED LEARNING
        ---------------------------

    - Our goal is to define a space of hypotheses in a way that respects the constaints the
      user places on the evolving program by the editing that they are providing for the program.

    - Consider a case when the by-demonstration algorithm learn a procedure with step A followed
      by step B.  If the user then edits the procedure to have step B before step A, that should be
      viewed as a constraint for subsequent learning--The alorithm should not immediately revert
      the the original A->B ordering.  

    - To accomodate the constaints provided by demonstatration and by explicit programming, we
      decompose the procedure being learned into two parts.  One part is a set of action steps
      that make up the procedure, and the second part is the logic that controls the flow
      between these action steps.  This decomposition allows us to combine the constraints from
      both methodologies into a single system, and allows us to define how these potentially 
      conflicting information is combined in the integrated system.


    - REPRESENTATION OF PROCEDURE STEPS
      - By-demonstration learning of procedure steps
      - Programmin of procedure steps


    - REPRESENTATION OF CONTROL LOGIC
      - Action transition graphs
      - Notion of consistency with prior data, and consistency with prior edits
        - Definition of valid augumentation
        - Example of augmentations
      - Learning based on augmentations
      - Using a generalized "E-step" to provide data for the step learning above

    - INTEGRATING EDITING INTO THIS LEARNING
      - Restricting flow logic to human comprehensible constructs:  "if" "sequence" and "loop"



        -----------
        USAGE STUDY
        -----------




        ------------
        RELATED WORK
        ------------

    - HMM learning
    - Version space learning?
    - Other hybrid learning systems:  ???


       ----------------------
       DISCUSSION CONCLUSIONS
       ----------------------

