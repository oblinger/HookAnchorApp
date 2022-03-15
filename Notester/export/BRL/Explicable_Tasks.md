# BRL.Explicable\_Tasks --

    An EXPLICABLE TASK DOMAIN is a class of performance tasks where the teacher can solve the
    tasks, and can explain their performance in terms of concepts that recursively can
    be expalained, all the way down to a performance vocabulary known a priori to both the student 
    and teacher.

    We want to study only explicable task domains, because we are interested in exploring 
    domain independent ways of bootstrapping teacher knowledge to perform novel tasks.
    Focusing on explicable task domains keeps us away from the weaker methods of 
    inducing models from non-teacher driven data.

    -----
    Here is a stab at a more precise definition of an explicable task domain:

    An EXPLICABLE TASK DOMAIN is a class of performances tasks where:

    (1) There exists a teacher that can solve each instance of the performance tasks.

    (2) The teacher can in principle (and may in practice) explain each step of their performance.

    (3) We assume these explanations suffer from the frame problem, that is the explanations
        are only correct when a set of unstated assumptions are met.

    (4) Alhough missing, these we assume these frame assumptions are known to the teacher, 
        and can/will be provided in any performance context where that assumption is violated.
        e.g. the teacher says "when flying near the market, always take a photo of 
        large parked vehicles"   then later  "well if you are running very low on fuel, just go home"

    Thus the abmiguity in an explicable task domain lies entirely in the alternate
    ways that the provided knowledge might be combined into a functioning performance element.
    Ambiguity in these domains does not arise from model discovery:  the search thru a space of 
    syntactic forms for effective performance components.
