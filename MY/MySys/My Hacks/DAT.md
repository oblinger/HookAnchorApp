EC2 Instance dan-pipeline
. ssh -A -i cv-ops-general.pem ubuntu@3.237.7.41

 conda env create -f environment.yml


git submodule init
git submodule update
pip install -e ./dvc-dat.  # done by the environment.yml


## Todo
- [ ] See if system works
- [ ] Folder of video files into DatContainer with Dat per file
- [ ] Folder of folder of video files with prefix left right or center


# DVC DAT
## Later  
- [ ] DVC - push, load
- [ ] Inject indicies should not inject for degenerate indicies w/ only one value  
- [ ] Refactor the Mount commands???
- [ ] Support cube of cubes with cached points in sub cubes  
- [ ] Implement the 'get_csv' method  
- [ ] set result (don't now since it might not be JSON writable)
  
  
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
- [x] Figure out how from_template and Inst.do work & have plan
- [x] Add a 'do' register method
- [x] Convert do module to singleton
- [x] Add metric Matrix for minutes
- [x] add run duration
- [x] add args kwargs to spec.main
- [x] remove datconfig.do_folder. ?????
- [x] Dat .move .copy .delete
- [x] Inst examples
- [x] Inst init with name or path and spec
- [x] Combining config and code example; threshold and slice (how?)
- [x] Inst Templating with folder template
- [x] add .datconfig registry for module and value loads  
- [x] convert inst names to use '.' instead of '/' (still needs testing) 
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


### INST Constructor
- spec = literal dict tree to save as spec for the new inst
- spec_template = template whose expansion & path expansion is used for the new inst
- path = literal absolute or relative path where inst should be created.  (error if exists)
- overwrite = flag allowing folder for auto erasure and overwriting
- path_template = template expanded to produce final path

  main: {
      folder_template: "{cwd}/{YYYY}/{MM}/{DD}"
    }

.stage(bool)


Inst(spec=, path=)     "anonymous/Inst{unique}"

check: DVC 
TAGS: DVC, ML-FLOW, DATA
### RENAMES
ML-DAT  ==>  DVC-DAT

DVC-DAT - Thin python wrapper for DVC-based ML-datasets and workflows
- Python bindings for pulling/pushing data folders directly from DVC.
- Git-like work flow for staging then pushing updates to the DVC data-repo.
- Folder declaratively configured via json/yaml to dynamically launched python actions
  (supports ML-Flow experiment and model building workflows)
- Pandas DataFrames and Excel reporting over metrics applied to trees of data folders

- Data wrapper
- An OO-like ability to associate python script actions declaratively in the data folders.
  (supports ML-Flow experiment and model building workflows)
- Declarative data execution

- Declarative data execution/management with git-like interface for remote storage.

- Thin wrapper over DVC, Git, and Python module loading that support
  folders-artifacts with optional script actions.
- (supports ML workflows leveraging DVC, MLFLOW, Pandas and Excel.)


Dat.get(spec, "this.that")

### MOUNT
do.mount(value: at:)
do.mount(live_value: at:)
do.mount(file: at:)
do.mount(module: at:)
do.mount(shallow_files: at:)

do.mount(module_tree: at:)
do.mount(file_tree: at:)

do.load("dotted.key")
do.mount("dotted.key" file:)
do.mount("dotted.key" VALUE)
do.set("dotted.key" module:)


Dat.get(dict, "dotted.key")
Dat.get(dat, "dotted.key")
Dat.get(dat.get_spec(), "dotted.key")


	# .datconfig.yaml
	mounts:
		- {value: 555, at: "foo.bar"} 		    # loads constant value
		- {live_value: do_string, at: foo.bar}   # computes dynamic value and loads it
		- {file: PATH, at: foo.bar}              # loads .py .json .yaml
		- {module: module_name, at: foo.bar}     # loads .py using its module path
		- {all_files: path, at: foo.bar}         # loads TREE of .py .json .yaml into flat namespace


    # .datconfig
    mounts: |-
	    mount value: 555 at foo.bar



    $ do foo
    # foo.py
    index = {...}

     def my_metric():
          ...

      def fn(...)
      do.in_package()
      do.mount(fn, at:".my_metric.regression", fn)



# LOG

### 2024-06-17  New Mount Design



    top_folder/subfolder/.../file.part.subpart...subsubpar
Tree of Values:
- Above string is the name-string for a value within the tree of values.
- The file for this value is relative to the sync folder or any other mounted folder.
- First system will scan all mounted folders for a local copy of the indicated file, if found that value is loaded.
	- Else if the indicated folder is in the sync folder it is used
	- else a DVC pull is performed on the relevant folder within the sync tree and is used.
- Anytime a DVC pull is done, all values under that folder are scanned and added
- once per hour a git pull is done to see if new versions of files exist.
	- Then a DVC pull is done on currently loaded DVC entries???




    sync_folder: ...
    mounts:
    - folder: 
       at: ...
    - value:
       at: ...
    - file:
       at: ...

### 2024-06-08  Notes on conflict

If local files are updated at the same time that a remote DVC push has occurred:
- git pull will work and will overwrite old checksum
- dvc pull will generate an error noting that existing files conflict with OLD checksum so pull cannot happen.
- If one does a dvc push it will succeed w/o issue in this context.

### 2024-06-12  notes

  
 STUFF NOW IN DATA REPO  
   dvc init  # only done once  
   dvc remote add -d storage gdrive://....  
  
c d algorithms2  
 git clone https://dan-oblinger@bitbucket.org/SVEngineering/data.git  
 cd data  
  
  
 dvc config  
 mkdir -p data