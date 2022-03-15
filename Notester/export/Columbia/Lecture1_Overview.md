# Columbia.Lecture1\_Overview --


    ===  INTRO TO CLASSIFICATION CLASS OUTLINE  ===
    - Intro: intuitive grasp of terms before diff algs
             going intrduce Classification (Supervised learning)

    - Motivate: Profitable C-card customer
      Protein shape, Speech impediment
      Handwriting recognition, Vision, Stock Fraud Detection

    EXAMPLE1: Rover's food grab.

    CLASSIFICATION: instances,features,target,train-set,test-set,feat-space
    - Def: classification is choosing best hyp to fit the data
    - Similar goals to statistics but no closed formula is being solved
      usually an iterative alg that considers alternative choices

    = Training Set
      Set of examples the from which the learner is 
    = Feature
    = Feature Vector (for example)
    - Example label


    = HYPOTHESIS, Concept
    = HYPOTHESIS SPACE, concept space, or Model of domain
      Space of alternative patterns that might be found in the data.
    = Target concept
      the "real" concept that as part of the world, is generating the
      trainging data

    = Objective function (loss function, cost function, utility measure)
    - Test set

    - Classification algorithm


    EXAMPLE: (Apply defs to these problems)
    - Rover's food grab  (person's size, how far away)
    - Rover's relief
    - Rover's Diet       (green stringy)  very good; little sick
      color, shape, hardness, texture


    IMPORTANT properties of classification problems <<>> algorithms
      - HYP SPACE
        (unit circles vs. convex polygons vs. axis aligned boxes)
      - Objective fn: find best hyp for rover #1
      - ROC Curvess


    GENERALIZATION
      - Idea: use tiny circles
      - Def generalization
      - REPRESENTATION
      - Hyp space size opposite of amt of generalization
      - Noise???

    BIAS
      - Def of bias
       (want weak bias so we are covered)
      - hyp space size =<<>> strength of bias

    VARIENCE  -- inherent noise in output
      - stability of output
      - related to accuracy

    Different biases
      - the goal of the engineer useing classification
      - Impt bias: local vs. global trend

    bias varience tradeoff  more less flexx
      - STRONG vs. WEAK BIAS
        # examples needed to learn concept; learnable in space
      - Tradeoff   (weak,bad@noise    vs. strong fewEx, good@noise
      - Overfit   (problem of dimensionality)
      - Curse of dimensionality
    - overfit vs. overgeneralizsation


    HOW LEARNING IS USED
    - diff alg diff bias; diff setting diff bias; diff feature diff bias
    - example of simplifi
    - Data preparation
    - Using learning as a balancing act.


    NO? version space
    YES? Bias


    Add ons?????
    - Cross validation
    - Discrete vs. continuous attributes
    - Noisy training data
    - probabilistic classification
    - feature extraction
    ? supervised vs. unsupervised learning

    - bayes riskxs
