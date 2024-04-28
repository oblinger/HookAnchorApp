

  

  
## Todo  
- [x] add .columns
- [x] add .format
- [x] add .transform
- [x] add .get_base
- [ ] convert inst names to use '.' instead of '/' (still needs testing)  
- dt.list should work when registered only in its source file  
- add .datconfig registry for module and value loads  
- add ability to have value redirecting loads????  
- Inject indicies should not inject for degenerate indicies w/ only one value  
- Change semantics of Inst.save to Inst.place  
  
## Later  
- [ ] get rid of "do" folder default of "do"
- [ ] Split ml-cube into a separate package  
- [ ] Add a `requirements.txt` file  
- [ ] Add a `setup.py` file  
- [ ] make inst use the datconfig defined folder  
- Support cube of cubes with cached points in sub cubes  
TODO for cube.py  
- Implement the 'merge_keys' method  
- Implement the 'get_csv' method  
- Support inst 'Cube' section that build data cube and cube reports  
  
## Ideas / Issues

do.mount(from, into) - 
do.set(key, value)
do.load --> do.get

do_index - tree of dict with cache nodes: load_path, import_name, module

to_excel
.columns {}
.values {}

#### USE index

#### .instantiate
If we split do and Load.from_template then how do we call from $ do
  
## Ask  
- What about foo:a.b.c.d  
- what about naming the report metrics_matrix? & having the config section be same?  
- How to: from ml_dat import Inst, do  
- Accept the list into a constructor. must I do a shallow copy?  
- Should .add_insts accept a list of insts or a star list of insts?  
- I plan to split ml-cube from ml-dat  
- Right now Inst has ALL property functions. maybe it is more confusing than helpful.  
  