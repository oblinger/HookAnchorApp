  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will 
do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get 
converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess 
it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. 
Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be 
the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's 
talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On 
Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted 
into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going 
to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro 
expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the 
structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is 
assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or 
discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be 
expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed 
to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the 
execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then 
runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro 
extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform 
Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw 
itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no 
recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression 
needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And 
others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's 
called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name 
macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the 
YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. 
Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, 
so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, 
the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get 
there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure 
itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just 
have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe 
get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows 
us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do 
evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit 
different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back 
quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are 
evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the 
expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get 
evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report 
as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario 
entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro 
structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the 
difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be 
evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro 
expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the 
macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And 
those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands 
itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function 
will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way 
it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of 
its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be 
expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its 
parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know 
when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that 
back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then 
it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need 
to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis 
notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is 
evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and 
pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So 
it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a 
report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or 
actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's 
probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we 
can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this 
together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do 
recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there 
could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is 
Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to 
do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions 
It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore 
expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to 
execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is 
different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that 
execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template 
language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and 
it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those 
constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a 
scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's 
returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it 
until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. 
If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And 
then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's 
going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the 
simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of 
every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an expression head And Name of these functions evaluate their 
arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And 
the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be 
called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the 
keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python 
function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always 
a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure 
which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded 
before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the 
structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been 
macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it 
calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these 
scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have 
expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it 
evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the 
structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then 
EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after 
you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the 
exclamation point e v expressions will get evaluated at that AI. And, the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's 
take the execution of I was building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being 
passed in and find all of this scenario entries. Then for each scenario entry, gonna execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it 
And every expression of in the macro structure Every EXPR within the structure. Is gonna get executed. And the results of that execution are going to be the structure that results from the macro 
expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a AI bit like Templates template language where the structure is assumed to be constant and there's special 
indicators of the parts that need to be evaluated. So an expression with normal is gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI 
AI So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions For example, a rate expression might be 
still in that structure. So the macro expansion of expressions can return expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be 
constants. Including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a 
scenario object, it actually macro expands itself gets the result of that macro Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns 
those as a dictionary. The report function will actually take its argument Perform macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it 
executes the whole thing. So maybe the way it works is after you do load time expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans 
through a structure, Exiting cutting. Each of its expressions. Executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro 
expanded, so that's already done. There may be expressions in it AI rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the 
way a Scenario works. It does not evaluate its parts. Other functions do evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add 
another function property. That lets us know when a function is an expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that 
evaluates the arguments Actually, I shake that back. I think there should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs 
function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any 
evaluation just gets its structure as its argument. We need to do a little translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in 
the list. It's just in the YAML or in the parenthesis notation. They need to be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in 
Python. In the case of vmax macro, none of the stuff is evaluated. And In the case of a function, they are Maybe the first argument is always a context object, so that we can get context variables 
out Or maybe we play the trick where we do the e x e c and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you 
run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the 
simulator. And return the result. If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself. That you're gonna build a 
report from. And that's gonna be a point or two or a ref or actually structure that has scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking 
for the Scenario, and then you can run them all. So there's probably a helper method for report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets 
the scenarios as a list that have been expanded out so we can run them And that allows us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or 
farm them out or something like that. So putting all this together, we have a macro decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. 
Then we do macro Explicit. Macro expansion. Then we do recursive execution. Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non 
Explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the 
way we think about it. When you do evaluation, it is Bash a back quote. The whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this 
semantics? Are there any issues Is there a better way to do this?  Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function 
is going to go through and perform all of the insertions It will do Value. On Any of the expressions that are the exclamation point e v expressions will get evaluated at that AI. And, the exclamation 
point includes get evaluated. And the underscore expressions get converted into EXPR structures. AI Then let's take the execution of I was building a report as an example. So I think report is gonna 
have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of this scenario entries. Then for each scenario entry, gonna 
execute a macro EXPR.md. AI I think is different than an expand. Macro expand takes a structure and it scans it And every expression of in the macro structure Every EXPR within the structure. Is 
gonna get executed. And the results of that execution are going to be the structure that results from the macro expand. So it's a lot like LISP but the difference is it's a little bit like Lisp, and 
it's a AI bit like Templates template language where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with normal is 
gonna evaluate that as a function and it's gonna return a number. Or discrete is gonna return a discrete choice. AI AI So then at the end of macro expansion, you're gonna have a complete structure 
with just constants in it. Some of those constants can still be expressions For example, a rate expression might be still in that structure. So the macro expansion of expressions can return 
expressions. If needed. And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants. Including expressions. And those are loaded in for simulation. And the 
evaluation of the simulation is what's returned at the end of the execution of a scenario. So if you evaluate a scenario object, it actually macro expands itself gets the result of that macro 
Explicit. Passes it to the simulator, runs it until termination, then runs all of the scoring functions. And returns those as a dictionary. The report function will actually take its argument Perform 
macro expansion on it. Well, AI quite sure. If it performs macro extension on the whole thing. AI And then maybe it executes the whole thing. So maybe the way it works is after you do load time 
expansion. You then perform macro expansion. And then you perform Recursive execution. In recursive execution Scans through a structure, Exiting cutting. Each of its expressions. Executing each of 
its expressions. When you execute a scenario, it's going to throw itself through a simulator It's already been macro expanded, so that's already done. There may be expressions in it AI rate 
expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a Scenario works. It does not evaluate its parts. Other functions do 
evaluate their parts. So I'm thinking that the head of every expression needs to be defined in Python We probably need to add another function property. That lets us know when a function is an 
expression head And Name of these functions evaluate their arguments. And others do not. We probably have a helper function that evaluates the arguments Actually, I shake that back. I think there 
should be two decorators. One called Expr subdictionary. And the other that's called EXPR macro. And an EXPR function performs function evaluation on all of its arguments and then it calls the Python 
function. And an EXPR macro And maybe they can just be called function Name macro. But anyway, a macro does not do any evaluation just gets its structure as its argument. We need to do a little 
translation. If the fixed arguments actually occur after the keywords, in the YAML or list or Well, actually, it's not in the list. It's just in the YAML or in the parenthesis notation. They need to 
be moved before the keywords. When you call it as a Python function or macro. Because that's the way it has to be in Python. In the case of vmax macro, none of the stuff is evaluated. And In the case 
of a function, they are Maybe the first argument is always a context object, so that we can get context variables out Or maybe we play the trick where we do the e x e c and pass it the lexical scope 
so that it's defined in the right scope. I'm not sure which. But either way, the function then runs So When you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate 
any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator. And return the result. If it's a report, I think a report is not a macro. It's 
just one of its variables is the name of the structure or the structure itself. That you're gonna build a report from. And that's gonna be a point or two or a ref or actually structure that has 
scenarios in it. And it will already have been macro expanded So you just have to scan that structure looking for the Scenario, and then you can run them all. So there's probably a helper method for 
report. Which is run all scenarios. And it calls that and gets Bash. Maybe get all scenarios. Scenario me gets the scenarios as a list that have been expanded out so we can run them And that allows 
us also for GPU optimization. To have these scenarios in a form that allows us to, AI, run them in parallel or farm them out or something like that. So putting all this together, we have a macro 
decorator and function decorator We have expressions, And the way that we do evaluation is first, we do expansion. Then we do macro Explicit. Macro expansion. Then we do recursive execution. 
Recursive execution or maybe we just call it evaluation. The eval function is a bit different in that anytime it gets non Explicit structure, it needs to scan it because there could be expressions 
inside of it. And it returns a copy of the structure. So it's almost like it's a back quote. Structure or maybe that is the way we think about it. When you do evaluation, it is Bash a back quote. The 
whole structure is back quoted. And then EXPRs are something like a comma that are evaluated. Do you think about this semantics? Are there any issues Is there a better way to do this?



⏺ This is a solid design. The Lisp backquote analogy is exactly right — structure is implicitly quoted, and Exprs are the "unquote" points. Let me work through the semantics and raise a few design questions:

  Summary of the Three Phases

| Phase        | Input         | Output               | What Happens                               |
| ------------ | ------------- | -------------------- | ------------------------------------------ |
| expand       | Raw YAML      | Tree with Expr nodes | !include evaluated, !_ converted to Expr   |
| macro_expand | Expr tree     | Mostly-constant tree | Traverse, call functions, splice results   |
| eval         | Expanded tree | Final results        | Run scenarios/reports, execute simulations |

  Key Design Question: 
  - Can macro_expand and eval merge?
	  - Both traverse the tree and dispatch on Expr heads. The difference is which functions are available and what they do:
  - macro_expand time: normal, discrete, choice → sample and return values
  - eval time: scenario, report → run simulations, collect results

  You could unify them with a single traversal that checks the head's phase annotation:

  @expr_function(phase="expand")  # runs during macro_expand
  def normal(ctx, mean, std):
      return ctx.rng.normal(mean, std)

  @expr_macro(phase="eval")  # runs during eval, receives unevaluated structure
  def scenario(ctx, structure):
      # structure already macro-expanded
      return ctx.simulator.run(structure)

  Single traversal, different function sets registered for each phase. This avoids duplicate tree-walking code.

  How Rate Expressions Survive

  The key is that michaelis_menten should be defined as a macro that returns an Expr:

  @expr_macro(phase="expand")
  def michaelis_menten(ctx, structure):
      # Don't evaluate — return self as data for the simulator
      return Expr("michaelis_menten", structure)

  Or simpler: rate expression heads simply aren't registered for the expand phase, so they pass through unchanged as Expr nodes.

  Context Object vs Lexical Scope

  Strongly recommend context object. Reasons:
  1. Explicit — you can see what's available
  2. Testable — easy to mock
  3. No magic — exec with locals injection is fragile
  4. Composable — nested contexts can shadow/extend parent

  @expr_function(phase="expand")
  def normal(ctx, mean, std):
      return ctx.rng.normal(mean, std)

  The context carries: RNG state, variable bindings, simulator handle, current path in tree, etc.

  Decorator Implementation Sketch

  def expr_function(phase="expand"):
      """Args are evaluated before function is called."""
      def decorator(fn):
          fn._expr_kind = "function"
          fn._expr_phase = phase
          return fn
      return decorator

  def expr_macro(phase="expand"):
      """Function receives raw structure, handles own evaluation."""
      def decorator(fn):
          fn._expr_kind = "macro"
          fn._expr_phase = phase
          return fn
      return decorator

  **Open Questions**
  1. Should there be an explicit "quote" expression? Right now, rate expressions survive by not being registered. An explicit {_: quote, ...} would make this intentional.
  2. Error handling: When an Expr head isn't found in any phase, is that an error or does it pass through? (I'd say: error during eval, pass-through during expand)
  3. Multiple macro_expand passes? Some systems need this (e.g., macros that expand to other macros). Do you need it here?

  Should I write this up as an addition to the EXPR doc?
