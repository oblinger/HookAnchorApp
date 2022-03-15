# Peter\_Clark --




    Hi Dan - 

    Very interesting talking with you today. Below are some interesting questions from Bill (also on the telecon) about the program - the issue of interactivity is particularly interesting: just as a teacher knowingly guides the learner, so the learner guides the teacher as to the bits they aren't clear about.
    Perhaps
    this is an important element to include?

    Bruce Porter's work on the Component Library is at:
        <<>> (browse the
    ontology)
        <<>>  (general page) Although the axioms are in idiosyncractic frame-based syntax, they are essentially first-order logic axioms.

    Suggested Uppper Merged Ontology <<>> doesn't contain axioms, it's largely a taxonomy but has been collaboratively built. Adam Pease is the guy who's championed this effort.

    If you look under "ontologies" on my list of pointers to related work
    at:
        <<>>
    you'll see a number of publically available ontologies. However, they are all either linguistically oriented or special purpose, I personally think the Component Library (Clib) is a good starting point.

    I'll put some CPL (our controlled language) examples together shortly so you can see what's going on there.

            Best wishes,

                Pete


    -----------------------
    Hi Dan -

    Below is an example of our controlled language CPL output for a toy recipe for baking bread. It gives a flavor of the output.
    Some notes:
     - It uses the Univ Texas ontology, but I had to add Flour, Dough, 
       Bread, Yeast, and Rise as there wasn't anything close in the
       ontology
     - CPL uses WordNet to help get the concepts, e.g., there's no
       concept of "chef" in the UT ontology, but WordNet tells CPL
       that a chef is a person and person *is* in the UT ontology.
     - CPL doesn't currently allow imperative statements, so the
       recipe is given as a story.
     - The output below omits some of the additional ground literals
       produced, in particular the instance-of (class) links and
       the input-word (lexical) links. The full output is in the
       file bread.txt attached so you can see the complete output.
     - All variables are existentially quantified. 
     - Note the system does coreference ie. can process multiple
       sentences.
     - There's an error in the interpretation, it's misinterpreted
       "place in the oven" to mean "the placing is done while inside 
       the oven" rather than "place with destination oven".
     - CPL will also generate Horn clause (Prolog-like) rules for
       English if-then sentences, not shown in this example.

    Also I'm attaching
     - The CPL User Guide, which teaches a user how to write in CPL
     - An extended set of examples from the Halo project (in physics)
       giving the original question, our CPL reformulation of it,
       and the CPL output.

    I hope this helps you get a better idea of what CPL does!

    Are there other examples you'd like us to have a look at? 
    We'd be happy to do so. We are open to guidance for how to explore this further with you.

         Best wishes,

            Pete

    ======================================================================

    #|
    THE FOLLOWING CPL FORMULATION WAS PROCESSED:
    "There is 400 g of flour.
    There is 25 g of yeast.
    There is 0.2 liters of water.

    The chef mixes the flour, the yeast, and the water.
    The result of the mixing is a dough.
    Then the dough rises for 40 minutes.
    Then the chef places the dough in an oven.
    Then the dough cooks.
    The result of the cooking is some bread."
    |#

    (quantity ?Flour6978 ?X6977)        ; There is 400 g of flour.
    (value ?X6977 (:pair 400 *gram))
    (quantity ?Yeast6981 ?X6980)        ; There is 25 g of yeast.
    (value ?X6980 (:pair 25 *gram))
    (quantity ?Water6986 ?X6985)        ; There is 0.2 liters of water.
    (value ?X6985 (:pair 0.2 *liter))

    (agent ?Mix6987 ?Chef6991)      ; The chef mixes the flour, the
    yeast, and the water.
    (object ?Mix6987 ?Flour6978)
    (object ?Mix6987 ?Yeast6981)
    (object ?Mix6987 ?Water6986)

    (result ?Mix6987 ?Dough6995)        ; The result of the mixing is a
    dough.

    (next-event ?Mix6987 ?Rise6997)     ; Then the dough rises for 40
    minutes.
    (object ?Rise6997 ?Dough6995)
    (duration ?Rise6997 ?X6999)
    (value ?X6999 (:pair 40 *minute))

    (next-event ?Rise6997 ?Place7000)   ; Then the chef places the dough
    in an oven.
    (agent ?Place7000 ?Chef6991)
    (object ?Place7000 ?Dough6995)
    (is-inside ?Place7000 ?Oven7003)

    (next-event ?Place7000 ?Cook7004)   ; Then the dough cooks.
    (object ?Cook7004 ?Dough7005)

    (result ?Cook7004 ?Bread7009)       ; The result of the cooking is
    some bread.

    ;;; -- end --   

    See the attachment bread.txt for the full output.
