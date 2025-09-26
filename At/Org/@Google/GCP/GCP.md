.[[GCP]].  >[[@Google]]
  [Console](hook://GCPconsole) 

- [[rem1]] 



[[2025-06 Alg2]]  

## Recipes

### Start Instance  (See [here](https://www.notion.so/sportsvisio/GCP-run-pipe-remotely-232719d6f2bd80af9679d42e5c834d80?source=copy_link) for the original)
- START NEW INSTANCE   ([GCP Console](hook://GCPconsole)) 
	WEB -> GCP Console -> Compute Engine -> Instance Templates -> "base-container-runner" -> rt!  "..." on right -> Create VM
	- **do-test1**    	{{Rename instance to be "do-test" or whatever, on first line}}
	- **us-east1-b**   {{Select zone "us-east1-b"}}
- LOCAL CONSOLE  (S5)
	- gcloud compute ssh --project "ai-worker-launcher" --zone "us-east1-b" "ubuntu@do-test1" --ssh-key-file=~/.ssh/google_compute_engine
- FIND AND RUN DOCKER CONTAINER
	- [console.cloud.google.com](https://console.cloud.google.com/artifacts/docker/ai-worker-launcher/us-east4/sv-ai-docker/sv-ai-pipeline?project=ai-worker-launcher&pli=1&invt=Ab28Lg&inv=1)  
	- Select the Tag as a string.
      `just run dev`          <-- this will the dev docker container
      `just run 0.0.0-1c6da8_playon_download_data`
    - At the end of this you should be in inside a TMUX this container.
- GRABBING DATA
	- `just pull gamesets/bb/NORM/NORM/G1 --gather`           # Grab game
	- `just pull runsets/bb/NORM/update-2025-08-04/G1`     # LS
	- `./scripts/data_manager.py ls gamesets/bb/NORM`       # Grab runset
- RUN TINY TEST GAME
	- Pull game from the server:
	  `python scripts/data_manager.py pull data gamesets/bb/TINY/TINY/G1-1m`
	- Run the game runner:
	  `python scripts/meta_run.py dat configs/runs/bb/prod.yaml --name runsets/bb/SCRATCH/my-test-run --game gamesets/bb/TINY/TINY/G1-1m`
	- Let's go look at the results
	  `cd data/runsets/bb/SCRATCH/my-test-run` 
- GET PICKLE DATA
	```python
import sys; sys.path.insert(0, '/algorithms2/data'); import rem
from utils.sv_pipeline_messages_manager import SVPipelineMessagesManager
p = "data/runsets/bb/NORM/update-2025-08-04/G1/3/output.pickle"
m = message_manager = SVPipelineMessagesManager.from_state(p) 

print(f"state.keys = {m.state.keys()}")   # views: ['center', 'mview'] 
print(f"state['center'].keys = {m.state["center"].keys()}")  # message_keys: ['time', 'fps', 'filename', 'image-size', 'runtime-metadata', 'object-detections', ...]
print(f"object-detections = {m.state["center"]["object-detections"]}")  # frame_numbers: [1, 2, 3, ...]
print(f'detect-msg = {m.state["center"]["object-detections"][1580]}') # message dataclasses: DetectionMessage(...)

import importlib; importlib.reload(rem)

```

]

/src/sync/communications/models.py 

pprint
Test 123  test 123

cat > /tmp/test_tunnel.py << 'EOF'
import urllib.request
import socket

try:
    response = urllib.request.urlopen('http://localhost:8888', timeout=2)
    print('localhost:8888 accessible')
except Exception as e:
    print('localhost:8888 not accessible:', str(e)[:50])

try:
    response = urllib.request.urlopen('http://host.docker.internal:8888', timeout=2)
    print('host.docker.internal:8888 accessible')
except Exception as e:
    print('host.docker.internal:8888 not accessible:', str(e)[:50])

print('Container hostname:', socket.gethostname())
print('Container IP:', socket.gethostbyname(socket.gethostname()))