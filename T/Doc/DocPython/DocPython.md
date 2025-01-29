.[[DocPython]].
  ,   [[DocConda]], [[DocJupyter]], [[DocNumPy]], [[DocPycharm]], [[DocPyDatetime]], [[DocSeaborn]], [[DocTensorFlow]], [[DocPythonCards]], 
  , , ,
  , ,
  , [[DocTorch]], [[Torch Tensor]],
  , ,
  , ,
  DELS: ,[[DocTorch]], [[Torch Tensor]],
  , [[Torch nn.Module]], [[Torch Training Examples]], 
  DELS: ,[[DocTorch]], [[Torch Tensor]],[[Torch Training Examples]],
  DELS: [[DocMatplotlib]],[[DocTorch]], [[Torch Tensor]],[[Torch Training Examples]],[[Torch Template]], [[DocMatplotlib]], 

[[DocTorch]]

:: [[DocJupyter]]

:: [[Doc/DocPython/DocJupyter]]

:: [[DocNumPy]],   [[DocPyDatetime]],   [[DocTensorFlow]]
- [[Doc/DocPython/DocJupyter]]
- [[DocConda]], [[DocSeaborn]], [[DocPycharm]], 

(See file:///ob/topics/Docs/src/Blam_Python/Blam_Python.md )

# iPython

Quick Tour                     https://mail.google.com/mail/u/0/#search/13664167/14dbe99e009fc109

gsv
# Python Topics

## C++ Integration
- https://docs.python.org/2/c-api/index.html

## Performance
import timeit
>>> setup1 = """from collections import defaultdict
... s = [('yellow', 1), ('blue', 2), ('yellow', 3), ('blue', 4), ('red', 1)]
... d = defaultdict(list)"""
>>> stmt1 = """for k, v in s:
...     d[k].append(v)"""
>>> setup2 = """s = [('yellow', 1), ('blue', 2), ('yellow', 3), ('blue', 4), ('red', 1)]
... d = {}"""
>>> stmt2 = """for k, v in s:
...     d.setdefault(k, []).append(v)"""
>>> timeit.timeit(setup=setup1, stmt=stmt1)
1.0283400125194078
>>> timeit.timeit(setup=setup2, stmt=stmt2)
1.7767367580925395

## MetaClass gymnastics

class Person:

    display_name = DisplayName()

    def __init__(self, salutation, forename, surname):
        self.salutation = salutation
        self.forename = forename
        self.surname = surname


class DisplayName:

    def __get__(self, instance, owner=None):
        parts = []
        if instance.salutation:
            parts.append(instance.salutation)
        if instance.forename:
            parts.append(instance.forename[0] + ".")
        parts.append(instance.surname)
        return " ".join(parts)


# Python SETUP

## PyCharm

#### Configuring paths 2014/7/16

     Pycharm -> #, -> Project Structure
       {[Select folder then click "Mark As [!Sources]"]}
       # this will add that folder to the PYTHON PATH

#### Configuring paths
  #, -> Project Interpreters -> !ConfigureInterpter -> Paths
     !'+'   {[Add folder that you want as roots added to PYTHONPATH




## Python Interpreter setup
  $ sudo pip install <pkgname>    # when import PKGNAME is missing
  $ yum search <c_pkgname>        # when pip fails because 'c' package is missing
  $ yum install <c_pkgname>       # installs pkg  (usually need the -devel pkg)

## Python Anaconda (ipython notebook)

- http://continuum.io/downloads
  ! MacOSX 64big python2.7 graphical installer


# Py Docs
- Py Tutorial          http://docs.python.org/3/tutorial/modules.html
- Py for data science  https://www.kaggle.com/wiki/GettingStartedWithPythonForDataScience
- Tut for NLP words    http://nbviewer.ipython.org/url/norvig.com/ipython/How%20to%20Do%20Things%20with%20Words.ipynb

- Quick Ref http://rgruet.free.fr/PQR25/PQR2.5.html  (import into blam)

https://zapier.com/engineering/debugging-python-boss/


EXAMPLES
- Flask  https://github.com/mitsuhiko/flask/blob/master/flask/blueprints.py
- ?Praw https://github.com/praw-dev/praw/blob/master/praw/helpers.py
- ?Pyrimid  https://github.com/Pylons/pyramid/blob/master/pyramid/authentication.py


TO LOOK AT WHEN STUDYING
- http://climateecology.wordpress.com/2014/02/10/a-side-by-side-example-of-r-and-python/
http://blog.scrapinghub.com/2014/03/26/optimizing-memory-usage-of-scikit-learn-models-using-succinct-tries/
http://www.datasciencecentral.com/profiles/blogs/sample-data-science-project-optimizing-all-business-levers-simult



## Python usage
- As Sci Stack  http://www.r-bloggers.com/the-homogenization-of-scientific-computing-or-why-python-is-steadily-eating-other-languages-lunch/
- Building ML stack on Vagrant    http://jeroenjanssens.com/2013/12/07/lean-mean-data-science-machine.html


## Doc Topics

### Python speed
-  http://code-redefined.blogspot.com/2011/03/cython-made-my-python-code-go.html



