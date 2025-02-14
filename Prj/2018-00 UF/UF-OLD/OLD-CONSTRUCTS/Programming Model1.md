type == structural spec     template matcher for structure instances
class == functional spec    template matcher for functional instances

### TODO
- VAR: what context are default value forms executed in?  I hope to access current lex location (see $root below)



### Usage Examples


p = some_program

uf.code.rewrite := std.2019.lib.TRS_simple


### Compile

::
	lang uf, @rubaic
	with dialect<-'english::
	
		pkg std.2018.lang.uniform.code;;
			transform tree_extend_by( `__ docs __.$dialect )
			_ _ _ 

			parse: "-- translates textual form into code form"
			


::
	pkg std.2018.lang.uniform.code

	$root		||= ^UP
	$rewrite	||= uf.TRS(max_depth: 1000) 	// The default rewrite engine to use

	$ _ := pipe( normalization, expansion, )

	@cached
	def op _get_packages(with_name):
		return $root.tree.select(head('$with_name) && up.head('Code))

	def op parse(input Textual):
		return normalize( scan(input, syntax) )
		
	def op normalize(c Code):
		rewrite(c, _get_packages("normalizations"))

	def op expand(c Code):
		rewrite(c, _get_packages("expansions"))



### Eval Primatives

CTX
~ ctx.create(ctx_form Code) -> child Ctx
~ ctx.enter(parent_env Env) -> ctx_env Env
~ ctx.exit(ctx_env Env)

CTX.GND.eval.1 <- 
DEF(ctx_form):
	$e := lang.denote(ctx_form).enter(env)
	$result := eval(ctx_form.rest, env:e)  

FN.GND.eval.2 <-
DEF(actual_args, ctx_form):
	$e := env.NEW(^ops: ctx_form.ops)
	e += actual_args =~ ctx.first
	$result := eval(ctx_form.rest, env:e)  


GET.GND.eval.2 <-
	op(form)::
		with form =~ `GET($key, $default)::
			...

GET.GND.expand.2 <-
	op(form)::
		with form =~ `GET($key, $default):
			if is.constant($key):
				$offset := ...
				$base := self.as(expansion)
				return `DEREF($base, $offset)
			else:
				return `with($offset:=...) {return DEREF(self.as())}
				
				