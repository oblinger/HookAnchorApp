

- FOLDERS:
	- inst_data/  	-- local versions of S3 data (or local data)
	- src/specs		-- versioned spec files (not named  _ spec _ .json) for S3 data
	- src/inst			-- source code for different inst types



/builds/ocr/yyyy-mm-ddvnnn/...
/builds/ocr/YYYY-v1/...
/models/ocr/yyyy-mm-ddvnnn
/models/ocr/latest
/u/do/
/u/jb/

$ cd src/specs
$ cp ocr/2024-01-01v007.spec   ocr/latest.spec

inst_data/ocr/latest/_ spec _ .json  ==  src/specs/ocr/latest.spec


inst_data/ocr/latest/latest.dvc  ==  src/specs/ocr/latest.spec


inst.save()


- S3 STORAGE
	- /SUB_PATH_TREE/YYYY-MM-DDv007.zip
	- /SUB_PATH_TREE/SUBSUB/YYYY-MM-DDv007.zip
	- /

- /builds/ocr/yyyy-mm-ddvnnn/yyyy-mm-ddvnnn.zip

- COMMAND LINE

	  $ do inst --save NAMES
	  $ do inst --mv 2024-01-23/bb/game/baller10 bb/game/baller10


QUESTIONS
- do you like my folder tree?
- Use caching S3 tree walk, or use directory file.
- Manual 'mv' and 'rm' operations on spec files.
- Either explicit 'do inst --save' or 'do inst --saves' or 'do inst --prompt'  to prompt for all non-saved files.
- Do I really get value from DVC if I am managing versions using GIT, and need to mange recursive containers explicitly anyway.
	- Just .zip and boto3
- Prefer S3 files with expected/meaningful names, easier to cleanup older files???





- API
	- Inst.save(name, force: True)