  [REF](REF.md)  [DeepLearning](DeepLearning/DeepLearning.md)  [ML](ML.md)
  [Research](Research/__Research__.md)  [DeepLearning](DeepLearning/__DeepLearning__.md)  [ML](ML/__ML__.md)


## Online Services
*** Text
**** Hemmingway text analysis tool
http://www.hemingwayapp.com/
## Financial 
**** Rent vs Buy
http://www.nytimes.com/interactive/2014/upshot/buy-rent-calculator.html?partner=rss&emc=rss&_r=0


****  Companies
*****  Marketing
- Vertical response -- email marketing
## Lang
### Clojure
#### Setup on OSX
Clojure setup                  https://gist.github.com/technomancy/2395913
### Haskell
* 

#### Tutorials
* Penn Slides:  https://www.seas.upenn.edu/~cis194/fall16/   (BEST)
* Stan Slides:  http://www.scs.stanford.edu/11au-cs240h/notes/basics.html
* Best?  http://learnyouahaskell.com/chapters
* Real World Haskell. good? http://book.realworldhaskell.org/read/
* Learn you a haskell - weird intro. http://learnyouahaskell.com/learnyouahaskell.pdf
* Yet another haskell tutorial http://learnyouahaskell.com/learnyouahaskell.pdf
* HAskell in 10min. https://wiki.haskell.org/Learn_Haskell_in_10_minutes
* Wikipedia (basic concepts) Abstract Rewriting System https://en.wikipedia.org/wiki/Abstract_rewriting_system
_
# RESEARCH
## ML
### ML Problem Steps

DATA UNDERSTANDING / VISUALIZATION
- which features are predictive?
- how do features 

DATA CLEANING
- missing data: cells / rows
	- Impute: Average, Median, 
- duplicated: rows / columns
- Feature normalization:
	Linear, Logarithmic, Percent Rank (binning)
- Synonym Identification

FEATURE ENGINEERING
- Constructing features (based on domain)
- DATASET TRANSFORMATIONS
	- Windowing / Averaging rows
	- Table joinings
- Similarity-based feature construction
	- Clustering / PCA for features

MODEL BUILDING
- Algorithm Selection
- Test Runs (see what is predictive)
- Iterate cleaning/engineering/transform

_
### Intro stuff
* [PRACTICAL AI](https://practicalai.me/)  


Kamil,
My responsiveness SUCKS EGGS!!  Sorry about that.  I saw your note and then it got buried!  Very sorry.

but here are some thoughts:

**FIRST**, you are a strong dev, and I think having reasonably strong mathematical background.  Thus you should be aiming for after introductions that go a bit deeper both into the algorithms on ML AND the practical use of those algorithms.

**PLATFORM** -- In 2019, I think the winning platform for a programmer is definitely the python datascience ecosystem.  This includes: Jupiter Notebook, Numpy/SciPy/Sci Kit Learn/matplotlib, Tensor Flow / Keras, Open CV (for vision)

These tools have tons of examples online, tutorials, and also many people have nice annotated examples of using those tools to attack specific datasets.  Once you know the basics of these tools you can easily follow along with their online documented analysis.  

**AREAS** -- For machine learning there are several key areas to know about.  You might just start with a few... if you kept going you might learn a bit about each area.  Here is how I break up the world:
- STATS -- ML build ontop of formal statistial methods:  Linear regression, logistic regression, baysian learning 
- CLASSIC ML -- Decision trees, rule learning, reinforcement learning, forests/boosting/bagging etc.
- DEEP LEARNING -- Tensor flow, convolutional nets, recurrent nets
- MODALITY SPECFIC LEARNING
	- IMAGE RECOGNITION
	- SPEECH RECOGNITION / HAND WRITING RECOGNITION 
	- VISION (tracking, video modelling, etc.)
	- NLP -- Natural Langauge Processing (text ML)

hit some stuff in the first three areas.

**TUTORIALS** -- There are **so** many tutorials out there.  It is hard for me to pick, and also I don't know the intro stuff so well.  But I did recently come across this combo theory + practice tutorial that I thought was really really nice beginner intro for someone like you:
    https://practicalai.me/

What I like about it, is it is pretty small, and each area, they first give you some theory, but then also connect this directly to doing some ML using that theory.  For you, I think you should aim for this combo.  Try to understand the jist of the algorithm (even if not all the details.)  Then try to actually *run* the algorithm inside of some python package.  This tutorial does both.

If the theory does not provide enough detail, you can just google "logistic regression" or whatever, and get several alternate summaries in just a few pages.

I will ask around and see if others have some intro tutorials they love.



**KAGGLE**  --  Kaggle is a competition that provides datasets and then people compete to try to build best classifiers.  These dataset have **EXPLODED** there are now thousands of downloadable datasets that you can play with.  My thinking is two fold.  One you get a bit more advanced, you can down load a dataset and try to do somthing with it.  (Note:  this is going to be a bit of a challenge to get awesome results, since this is real data, messy and such)  The second way is to google for people doing analysis on the popular Kaggle datasets.  You can see how others try to do ML over these sets.


**MIT / STANFORD / ANDREW NG** — I know Andrew Ng (former prof at Stanford) has a coursa class on ML.  I think there is one on ML and a different one on Deep learning.  These are very good deeper classes, they will focus more on the theory behind these algorithms, and I think they will go deeper than most online tutorials you will find.  But they are a bigger investment of time, and I would still play around with the tools and libraries along with those classes if you took one.

_ 
## DEEP LEARNING
### ALGS
#### CNN - Convolutional Neural Network
Distinguishing Features: 3D-vol-of-features, Local-connect, Share-weights, pool
#### RNN - Recurrent Neural Network

Hopfield Network - 
BAM - Bidirectional Associative Memory
NT - Neural Turing machines
#### GANs
#### Auto-Encoders
#### LSTMs
#### TDNN - Time Delay Neural Network
Like CNNs, but share weights across the time dimension

### Code Examples
Py+Numpy recurrent-net sentence+image https://github.com/karpathy/neuraltalk
### Papers
#### Learning Procedures From Text
https://dl.acm.org/doi/fullHtml/10.1145/3184558.3186347
## STATS
### Dirichlet Distribution / Dirichlet Processes 
- non-parametric pdf over pdfs
- used as a prior for baysian models
- Dirichlet process is “just” a glorified Dirichlet distribution.
- Draws from a DP are probability measures consisting of a weighted sum
of point masses.
_
### STATISTICAL INFERENCE ALGS
#### EM -- Expectation Maximization
_
#### MCMC -- Markov Chain MonteCarlo
- Idea:  Randomly create many markov chains, and compute some statistic over the many chain outcomes (e.g. play go)

- GIBBS SAMPLING -- tractible way to sample from a multi-variate distribution.
- a kind of MCMC alg
- (as simple form Metropolis-Hasting algorithms)
- ALG. (see implementation section for pseudo-code http://en.wikipedia.org/wiki/Gibbs_sampling)
  builds first sample X(1)= [x1, ..., xn]
  then  X(2) is function of X(1) where xi is function of X(2) for all Xj < i and a function of X(1) for all xj > i

- So the sequence of X(k) are not independent, but form a markov chain.
  so sometimes first 1000 values are dropped, and only every 100th entry is used to compensate

_
#### Variational Bayes
- variational methods are like EM, but add regularization by integrating information from prior distributions. 
- This avoids the singularities often found in EM but introduces some subtle biases to the model. 
- Inference is often notably slower, but not usually as much so as to render usage unpractical.
## ML/NLP People
### NLP Friends
-
- Chid Apte / Chris Welty / Vittorio Castelli / 
- John Langford
- UIMA:   Thilo Goetz IBM/UIMA,  

- woman from MR program

_
## CS
### Rewrite Systems
- MAUDE http://maude.cs.illinois.edu/w/images/e/e0/Maude-2.7.1-manual.pdf
- Wiki Rewrite https://en.wikipedia.org/wiki/Rewriting
- Goood Slides overview http://cs.okstate.edu/~kmg/class/5313/fall13/notes/ten.pdf
- Extensive course slides. https://www.cs.vu.nl/~tcs/trs/
- Good simple / TU Vien slides:  http://cl-informatik.uibk.ac.at/users/georg/teaching/ohp/1-handout.pdf
- Computing with https://core.ac.uk/download/pdf/82345385.pdf
- Drexel quick slides. https://www.cs.drexel.edu/~mwb33/teach_pres/mwb33_teaching_presentation.pdf
- OTHER SLIDES
https://web.fdi.ucm.es/posgrado/conferencias/MaribelFernandez-slides.pdf
- TRS overview - drexel - https://www.cs.drexel.edu/~mwb33/teach_pres/mwb33_teaching_presentation.pdf
* Practical Graph rewriting. http://research.cs.queensu.ca/~blostein/LNCS1073.pdf
* Academic overview of term rewriting http://research.cs.queensu.ca/~blostein/LNCS1073.pdf
* Lamba-the-ultimate blog on term rewrite implementations. http://research.cs.queensu.ca/~blostein/LNCS1073.pdf
* Old Academic term rewrite paper. http://www.cs.tau.ac.il/~nachum/papers/survey-draft.pdf http://www.cs.tau.ac.il/~nachum/papers/taste-fixed.pdf
* 
#### Code pointers
* DMS code xform toolkit -- http://www.semanticdesigns.com/Products/DMS/DMSToolkit.html



# LOG

### r2022-00-00  - Military spending video
https://youtu.be/FQbEWKy7i64


### r2021-00-00  - Lefty Attributes


Lefties die younger than righty.  Most of the difference is accidents or war.
But there is still more difference even when accidents are removed.
https://www.ncbi.nlm.nih.gov/pmc/articles/PMC1059767/
https://www.newscientist.com/article/mg13017662-900-left-handers-dont-die-young-after-all/


Not smarter:
https://www.psychologytoday.com/us/blog/brain-myths/201303/three-myths-and-three-facts-about-left-handers

Lefty facts:
https://blog.massmutual.com/post/left-handers-day
https://www.psychologytoday.com/us/blog/brain-myths/201303/three-myths-and-three-facts-about-left-handers


_