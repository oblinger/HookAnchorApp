

Pat,

You really put a bee in my bonnet after our random conversation!  
I had it in my mind to reach out to you regarding URF once I saw your name on those RDF docs about two years ago.  The reason I had not, is because my efforts around this ecosystem that I call "Uniform" have been aimed at completing a first pass of UNICORE which is the first four rings of the uniform spiral along with a reference implementation of the semantic assembly language with sits at the top of these four rings.

   					
		ASM 	:= FLOW  + VAR   + PKG   + CTX 	
		FUNC    := LEX   + UCOMP + EXE   + LANG	
		NODE    := URF   + UNIT  + OPT   + UFORM
		MATH 	:= GRAPH + TYPE  + ADDR  + SPACE + ATOM		

A formulation of the uniform "structural constructs" of computation is contained in the first two rows above while the "functional constructs" are contained the top two rows above.  (each row is called a ring within the spiral; the idea is that each is constructed so the powerset of the combinations of its constructs are generally useful, and each row is defined using only the constructs from the rows below)

_This thing is BIG_.  For example UCOMP (Uniform Computation) is a group of three constructs:
- Its starts by including the defintions of an Abstract Rewrite System.
- The it defines a vanilla Term Rewrite System whose rule terms and variables are all expressed as URF.
- Then it defines a single TRS program that define a rough analog to predicate calculus which uses unit struture and graph operators in lieu of icky editing of mathematical equations.  (no self respecting computer scientist would EVER use a math equation as a data-structure!!  :-) )
- Finally the execution model (EXE) and the primatives of sematic assembly (ASM) are expressed as a "unit-calculus" program.

Needless to say such a thing is a tour-de-force!  My progress has been slower than I had imagined.  Honestly I thought I understood these layers in my mind, far better than I really do.  At each turn I get stuck, or I see an awesome simplification which ends up REFACTORING huge chunks of multiple rings within the spiral!

As a theory guy, you likely will look at the massive half-baked thing and thing "wow, if Dan had only put all that effort into one promising piece of it, he might have actually GOTTEN somewhere!"  That is definitely a valid criticm.  The reason I resist this is really central to the aims of uniform itself.

The dream is to iteratively progress towards a spiral of interoperable building blocks that are PRAGMATICALLY ideal for constructing DSLs (domain specific languages)...  I think all knowlege work should be executed within a DSL tailor to that work, but in today's world that would create a WORLD of hurt, since these DSLs would have subtle incompatibilities.  In my dream world they do not have such incompatibilities since they are just different recombinations of common primatives.

So success for me, is not specifically about having formal precision.  Success is encode constructs like "assignment semtantics" "type erasure" "lexical scope" or "control flow" as interoperable building blocks that a DSL hacker could use to quickly build a specialized DSL which happens to interoperate perfectly with all other uniform DSL languages.  So the goal is a social/hacker goal.  

Still the general consensus is that it is _IN PRINCIPLE_ impossible to build a single such ecosystem... tradeoffs in construction that favors of one DSL must come at the cost of those that favor a different DSL.  There can BE no such spiral of constructs which is uniform and universal.

I think the only way I will make progress on that debate is to simply CREATE THE SPRIRAL in sloppy form, and show that it starts to accrue these generality / compatiblity benefits.  THEN if that is ever established, the value of cleaning it up becomes more aparent.  (and dream upon dream becomes a collaborative endevor)  Also I documented and formalized too early in the first years of this... it was a waste since things kept getting refactored as more and more redundancies where wrung out of the forming crystal.

Still you put a bee in my bonnet, so I spent the last days going back and filling in many pieces of URF that were still in my mind, and not on paper.  (Earlier when I felt confident that in principle I could put it on paper and not get surprised, I moved on before actually putting it on paper.  Even now I slammed it out in a pretty dirty and incomplete form!  Apologies!)



~~~~~~~~~~~

SO WHY DOES URF EVEN EXIST?

For the first period of this effort I just assumed RDF was at the bottom.  There is a huge advantage to that since RDF is well accepted.

But the point of Uniform is to be ... well ... Uniform.  Now the triple itself IS BEAUTIFUL and also quite uniform.  but it did not play well with others.  Of course it is representationally complete so it is _sufficient_ for encoding software, but it is not ideal.  In particular, it is messy and very non-uniform in its encoding of some very common data structures.  After working on many levels of the uniform stack I tentatively arrived at these 11 modifiers as a "powerset" building block (see Figure #1 below).  My goal was to have one ring in the spiral to describe a "container" construct.  That construct was to be expressed as a vertex node with some combination of these modifiers attached to it.  The key here is that 
	(1) the POWERSET of these attributes be useful
	(2) this collect is "pragmatically complete" meaning that engineers worried about simplicity, performance etc. will be happy to just use combos of this set, and compose any additional complexity ontop of this set. 
	(3) and the resulting combinations should be "uniform".   This means if I write an algorithm the does a depth-first traversal of a tree, that same algorithm should run without modification... regardless if the tree nodes are ordered or not, whether the are listy, dagy, static, etc.  It should not matter.  It only matters that they are iterable and that they are bounded since those are the properties those are the only attributes that are intrisically connected to the correctness of a depth first traversal algorithm.

But RDF broke this.  Try an I might, I could (parsimonously) express these structures in a uniform RDF way.  (Actaully this is not true!)  I did find a uniform representation.  Each graph arc was expressed as four triples one triple from the source vertex to a blank node, two more were kind of 2/3rds of a reification and the last expressed ordering.  YUCK!!  this was to be the "uniform" way I expressed all data?  OH I HAD TO WASH MY HANDS EVERY TIME I DREW A PICTURE OF THAT THING!  Eventually I saw that instead of expressing triples as  <X, Y, Z> I might express them as X -> [<Y, Z>, ...] and URF was borne.

It was only later that I understood the central failure of RDF as being the lack of handles.  And realized that URF needed to be organized so that handles FOR data are expressible naturally as part OF the data.


~~~~~
Finally I was proud of URF and wanted to claim that it embedded ALL data in the graph, but vexingly graph literals still seemed like a way that one could sneak latent semantics into non graphical forms.  Hence the GLU graph reduction was constrcucted as a kind of proof that, in principle, URF is a pure graph form, while also retaining all the practicality and performance benefits of having URI.  (basically GLU literals can be treated as opaque string handles when you want, and you can parse their structure as a graph when you want.)


~~~~~~
[Figure #1]

**UNICORE STRUCTURAL TYPES**
Uniform defines the following commonly occuring types of graph structure pattern by associating combinations of these attributes to each graph vertex.

**UNIFORM TYPES (each are a property of a graph vertex)**
**Ordered**	--  The edges for a vertex have a well defined stable order
**Keyed**	--  Edges have a vertex (called a _predicate_ or _key_) labelling them 
**Mappy**	--  Vertex is keyed, and no two edges have the same key vertex
**Listy**   --  Vertex is mapy, ordered, and keys indicate pos in order (is 0..n)
**Set**		--  A unit is a set if value vertices (its objects) are unique
**DAG**		--  A unit is the root of a DAG if its space is asyclic
**Tree**	--  A unit is the root of a tree iff there is exactly one 
		path to each of its descendents.
**Static**	--  A unit is static if its edges can never change
**Finite**	--	A unit is finite if it has a bounded number of edges
**Bounded** --  A unit is bounded if all children are bounded and it is finite
**Undirected** --    ... each unit is its own inverse

bits: ord, map, 

**GRAPH-TYPES THESIS** -- These 11 vertex attributes form a power-spiral ring underlying all computation.  (e.g. The powerset of these combinations are useful enough that engineers to build from these semantic combinations without extension or refactoring them.)

**ADJECTIVE NAMINGS**:
	Ordy, Keyed, Mapy, Listy, Sety, Dagy, Taxy, Fixy, Finy, Boundy,

These alternate names are expressed as short adjectives allowing one to parsimoniously name elements in powerset of combination types:  e.g. the "Source Code" data type is a: fixy-boundy-ordy-map.




At first it seemed the triple was BEAUTIFUL

In addition to getting your take on URF, I would like to have a video call where I show you about 30 slides that over-views all of UNICORE (slides are a modular and efficient way to think when your thoughts are getting refactored faster than you can type them)

I know about ontological style 


