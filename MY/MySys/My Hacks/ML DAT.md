
## For Juan
- [ ] Figure out how from_template and Inst.do work & have plan

  

  
## Todo
- [ ] Inst examples
- [ ] Inst init with name or path and spec
- [ ] Combining config and code example; threshold and slice (how?)
- [ ] Inst Templating with folder template
- [ ] add .datconfig registry for module and value loads  
- [ ] convert inst names to use '.' instead of '/' (still needs testing)
- [ ]   
  
## Later  

- [ ] DVC - push, load
- [ ] Inst .move .copy .delete
- [ ] Inject indicies should not inject for degenerate indicies w/ only one value  
- [ ] Support cube of cubes with cached points in sub cubes  
- [ ] Implement the 'get_csv' method  
  
  
## Ask  
- [ ] Inst .move .copy .delete
- [ ] Want a name that ties to DVC.
- [ ] Right now Inst has ALL property functions. maybe it is more confusing than helpful.

- [ ] NOPE: What about foo:a.b.c.d  
- [ ] YEP: what about naming the report metrics_matrix? & having the config section be same?  
- [ ] GOT IT: How to: from ml_dat import Inst, do  
- [ ] NOPE: Accept the list into a constructor. must I do a shallow copy?  
- [ ] NOPE: Should .add_insts accept a list of insts or a star list of insts?  
- [ ] NOPE: I plan to split ml-cube from ml-dat    
  

## Done
- [x] add .columns
- [x] add .format
- [x] add .transform
- [x] add .get_base
- [x] dt.list should work when registered only in its source file  
- [x] add ability to have value redirecting loads????  
- [x] Change semantics of Inst.save to Inst.place  
- [x] Inst.set(Inst, new_value) should generate an error (or should affect the results.json ???)
- [x] Implement the 'merge_keys' method  
- [x] get rid of "do" folder default of "do"
- [x] make inst use the datconfig defined folder  
- [x] Support inst 'Cube' section that build data cube and cube reports  


## Ideas / Issues

do.mount(from, into) - 
do.set(key, value)
do.load --> do.get
do_index - tree of dict with cache nodes: load_path, import_name, module

#### .from_template
If we split do and Load.from_template 
Then how do we call from $ do


### MOUNT

do.mount(value: at:)
do.mount(live_value: at:)
do.mount(module: at:)
do.mount(module_tree: at:)
do.mount(source: at:)
do.mount(source_tree: at:)
do.mount(flat_names: at:)

	# .datconfig
	mounts:
		- {value: 555, at: foo.bar} 				# loads constant value
		- {live_value: do_string, at: foo.bar}   # computes dynamic value and loads it
		- {source: path, at: foo.bar}            # loads .py .json .yaml
		- {source: module_name, at: foo.bar}     # loads .py using its module path
		- {source_tree: path, at: foo.bar}       # loads TREE of .py .json .yaml
		- {source: module_name, at: foo.bar}     # loads TREE of .py using its module path
		- {source_flat: path, at: foo.bar}       # loads TREE of .py .json .yaml into flat namespace
