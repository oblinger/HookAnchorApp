# ASP Evaluator Design Notes

Voice transcript describing the evaluation semantics for the ABIO spec language.

---

Okay. Let's talk through how execution works. So after you load the YAML file, you call the expand function. And the expand function is going to go through and perform all of the insertions. It will do value on any of the expressions that are the exclamation point `!ev` expressions — those will get evaluated at that time. And the exclamation point includes get evaluated. And the underscore expressions get converted into EXPR structures.

Then let's take the execution of building a report as an example. So I think report is gonna have an argument which it's gonna want to execute. And I guess it's going to scan the structure of what's being passed in and find all of the scenario entries. Then for each scenario entry, gonna execute a macro expand.

I think macro expand is different than expand. Macro expand takes a structure and it scans it. And every expression in the macro structure — every EXPR within the structure — is gonna get executed. And the results of that execution are going to be the structure that results from the macro expand.

So it's a lot like LISP but the difference is it's a little bit like Lisp, and it's a bit like template languages where the structure is assumed to be constant and there's special indicators of the parts that need to be evaluated. So an expression with `normal` is gonna evaluate that as a function and it's gonna return a number. Or `discrete` is gonna return a discrete choice.

So then at the end of macro expansion, you're gonna have a complete structure with just constants in it. Some of those constants can still be expressions. For example, a rate expression might still be in that structure. So the macro expansion of expressions can return expressions if needed.

And in the case of a scenario, it's then passed to a simulator and all of it is assumed to be constants — including expressions. And those are loaded in for simulation. And the evaluation of the simulation is what's returned at the end of the execution of a scenario.

So if you evaluate a scenario object, it actually macro expands itself, gets the result of that macro expansion, passes it to the simulator, runs it until termination, then runs all of the scoring functions, and returns those as a dictionary.

The report function will actually take its argument and perform macro expansion on it. And then maybe it executes the whole thing.

So maybe the way it works is:
1. After you do load time expansion
2. You then perform macro expansion
3. And then you perform recursive execution

In recursive execution, it scans through a structure, executing each of its expressions. When you execute a scenario, it's going to throw itself through a simulator. It's already been macro expanded, so that's already done. There may be expressions in it — rate expressions. That's fine. They all get thrown to the simulator with no recursive evaluation inside of them. That's just the way a scenario works. It does not evaluate its parts. Other functions do evaluate their parts.

So I'm thinking that the head of every expression needs to be defined in Python. We probably need to add another function property that lets us know when a function is an expression head. And some of these functions evaluate their arguments and others do not. We probably have a helper function that evaluates the arguments.

Actually, I take that back. I think there should be two decorators:
- One called `@function` (EXPR function)
- And the other called `@macro` (EXPR macro)

An EXPR function performs function evaluation on all of its arguments and then it calls the Python function. And an EXPR macro does not do any evaluation — it just gets its structure as its argument.

We need to do a little translation. If the fixed arguments actually occur after the keywords in the YAML or in the parenthesis notation, they need to be moved before the keywords when you call it as a Python function or macro. Because that's the way it has to be in Python.

In the case of a macro, none of the stuff is evaluated. And in the case of a function, they are. Maybe the first argument is always a context object, so that we can get context variables out. Or maybe we play the trick where we do the exec and pass it the lexical scope so that it's defined in the right scope. I'm not sure which. But either way, the function then runs.

So when you run the function in the case of a scenario, it's a macro. So it's not gonna evaluate any of its parts. They've already been macro expanded before you even get there. They're gonna pass it to the simulator and return the result.

If it's a report, I think a report is not a macro. It's just one of its variables is the name of the structure or the structure itself that you're gonna build a report from. And that's gonna be a pointer or a ref or actually a structure that has scenarios in it. And it will already have been macro expanded. So you just have to scan that structure looking for the scenarios, and then you can run them all.

So there's probably a helper method for report which is "run all scenarios." And it calls that and gets back — maybe "get all scenarios" — gets the scenarios as a list that have been expanded out so we can run them. And that allows us also for GPU optimization to have these scenarios in a form that allows us to run them in parallel or farm them out or something like that.

So putting all this together, we have:
- A `@macro` decorator and `@function` decorator
- We have expressions
- The way that we do evaluation is:
  1. First we do expansion
  2. Then we do macro expansion
  3. Then we do recursive execution (or maybe we just call it evaluation)

The eval function is a bit different in that anytime it gets non-explicit structure, it needs to scan it because there could be expressions inside of it. And it returns a copy of the structure. So it's almost like it's a backquote. Structure — or maybe that is the way we think about it.

When you do evaluation, it is like a backquote. The whole structure is backquoted. And then EXPRs are something like a comma that gets evaluated.

Do you think about this semantics? Are there any issues? Is there a better way to do this?
