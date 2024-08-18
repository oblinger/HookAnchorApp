#18189
runsets/bb/LT10/v2.0.0a1_1784ba34/01/3/output.pickle
gload gamesets/bb/LT10/01
git checkout feature/stationary
git checkout feature/scripts
pip install --upgrade pydantic lshapely imageio

./analysis_stationary_dan.py ~/ob/proj/sv/algorithms2/data/tmp/LT10T-1-2-3/01/3/output.pickle ~/ob/proj/sv/algorithms2/data/tmp/LT10T-1-2-3/01/3/center_viz.mp4

./analysis_stationary_dan.py ~/dataf/runset/ob/proj/sv/algorithms2/data/tmp/LT10T-1-2-3/01/3/output.pickle ~/ob/proj/sv/algorithms2/data/tmp/LT10T-1-2-3/01/3/center_viz.mp4

open ~/ob/proj/sv/algorithms2/data/tmp/LT10T-1-2-3/01/3/output_stationary_analysis_dan.mp4
open ~/ob/proj/sv/algorithms2/data


 gsutil -m rsync -r  "gs://sv-ai-data/runsets/bb/LT10/v2.0.0a1_1784ba34/01/"
 gsutil -m rsync -r  "gs://sv-ai-data/gamesets/bb/LT10T"


if os.getenv('HOME') == '/Users/oblinger':  # Setup for oblinger  
    print('Setting up for oblinger...')  
    os.environ["ASSETS"] = f"/Users/oblinger/ob/proj/sv/assets"  
    sys.path.append(f"/Users/oblinger/ob/proj/sv/algorithms2")  
    sys.path.append(f"/Users/oblinger/ob/proj/sv/algorithms2/src")


    sys.path.append(f"/{os.getenv('HOME')}/ob/proj/sv/assets")  
    sys.path.append(f"{os.getenv('HOME')}/ob/proj/sv/algorithms2/src")  
    sys.path.append(f"{os.getenv('HOME')}/ob/proj/sv/algorithms2/src/synch")  
NOTE: -----------------------  DAN O paths  
os.environ["ASSETS"] = f"{os.getenv('HOME')}/ob/proj/projects/trash/algorithms2/assets"  
sys.path.append(f"/Users/oblinger/ob/proj/sv/algorithms2")  
sys.path.append(f"/Users/oblinger/ob/proj/sv/algorithms2/src")



## Off Court. x1800

#### LT10/01
- 4:33 8100

#### LT10/02
- 4:50 8700