# Meeting.2007\_05\_16\_NewStartBrainstorm --

    ----------------------------------------------------------------------

    Dan Oblinger: Programmatic Level : Text Understanding Brainstorming

    What is the revolution that we're trying to foster?
    What is the goal? Should be a phrase.
    What are the inputs/outputs?
         Text + ??? -> ???

    David's take on this: 
    Input = (i) small, hand-built KB in a given domain
           (ii) access to a set of texts (in that domain) 
    Output = expanded/enhanced knowledge base
         better/faster/stronger, i.e. measurable improvement 
             in the knowledge base performance

    What are blinders (constraints) we'll put on it in a 3-5 yrs program?
    What are the training wheels that we're going to take off as we go along?

    ----------
    Q1: What are the attributes which define the system? 
        ("Things that we would be sad about if they weren't in the program")

     - Person-hour cost of KR is much lowered ("massive amplication")
       through semi-automatic use of text
     - exploit redundancy in text to overcome errors and ambiguity
       Dan says this is a big one to emphasize.

     Massive amplification: Raise KR rules/min/person 
     
    Dan: Redundant text can do more than overcome errors & redundancy, it
       can learn (build) parsers (say), e.g., learn a parser that can 
       parse a whole ontology.
    David: Well, we know how to build probabilistic CFG parsers (trained
           on treebanks). But there's a real challenge in using those
           downstream. 
           Also work done on trying to learn a parser whose parse output is
           nicely suited for walking to generate a LF.
    Dan: You can do some of this unusupervised, in a co-training sort of way.
         Can think of this as a massive EM problem.
         It's currently expensive writing parsers for specialized domains.

    ----------
    Q2: Program Statement - crisp, please!

    1. "Deep Text Understanding: Learn inferentially-rich world
        models given text describing the world"

    But: phrasing above seems to focus solely on descriptive text (= bad?).

    David: You need a performance task, eg QA/Problem-Solving.
        "This system can solve really tough problems."
    DanO: Well, apart from SAT I'm not sure how you'd clarify [assess] that.
    Dan prefers "inferential closure", David not.

    What are the bounds on the task?

    ----------
    Q3: Inputs/Outputs

    INPUTS
    Ontology + Inference Mechanism + Specialized info for parsing (?)
    Corpus: Web, descriptive text, college level
    [ Ralph: Plus you also need a mapping from the ontology to the language, for
      the ontology to be meaningful ]

    OUTPUT
    Micro-theories = logically consistent representations of a passage of text.
    + Behavior (e.g., SAT, Parameterized QA, Fact agreement vs. computation group)

     - - -

    Dan: Can you build the system not knowing what the pre-defined ontology will be?
    You could imagine this as a challenge task.
    Dan: I'd be very tempted, given this model, to have the target corpus = the Web.

    Noah: How do you bridge the "knowledge gap" (cliff), this is the crux of the
          problem.

    Dan: Would like the system to work no matter what ontology you give it.
    [This doesn't go down well!]
    Tomas: Given text, plus given task -> solve some problems.

    David: You have Allen's time interval calculus = target KR. You have 
    sentences "A is before B". But in real life these sentences come in all 
    sorts of complicated forms, it's a very difficult NL problem to map to
    this calculus. But: This isn't sufficiently ambitious.

    Dan: If you gave me these black boxes (KBs), and I can parse the Web
    according to them (map into them), there's be a lot of stuff you could do.

    David: In order for this to make sense programmatically, those black
    boxes have to be already built. 
    Dan: You can build 10,000 of these.
         But I think you guys are pushing on a deeper integration problem (?).

    Dan: [Another text] 
         Take black-box special theories, and learn to translate (map) text into 
         those general theories. Your system's going to learn to parse things
         on the Web and map them into those theories.

    Ken: This picture does nothing to address the gap between text and KR.

    Noah: You have the KB, the corpus, and a set of tasks.
    Dan: I'd like you to extend the types of inference you can do, e.g., 
         I'd like it to learn how to do diagnosis.

    Dan: I want it to do lots of things.
    Pete: How about diagnose any mechanical device?
    Dan: Well, I don't want the NL hard-wired to a particular inference process.
    Pete: Well, there are only a few small, fixed number of problem-solving 
          methods (PSMs). Suppose we are going to be given one, we don't know which. 
          Can we then read text to make it work?

    Diagnosis: Makes certain knowledge demands on the things its to diagnose:
    What are its parts? how are they connected? What are their functions
    and behaviors? What are the failure modes of those functions/behaviors?
    What are the dependencies between them?
    If you are given a PSM, and a clear spec of its knowledge requirements,
    then Mobius can go out and read to find the knowledge.

    To some extend Mobius is already doing this: It has a predefined
    ontology (CLib) which defines what it understands and what it does
    not. It then reads, pulling in facts within that scope and ignoring
    the rest.
