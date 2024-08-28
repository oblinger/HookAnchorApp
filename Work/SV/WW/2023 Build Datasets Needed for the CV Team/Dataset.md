(See [[Data]])

TODO:
[Obj DS-T02](https://docs.google.com/document/d/1HXuS-r_d7J4zaHIiqKGg-p2U0uVDHE7oooCSgyXjoe0/edit):	-[meta](https://docs.google.com/document/d/1ESUxDwrI3L714VihJTbfeoU81PVRvTIYu-FEVILmL3c/edit)   ?how much more do we need
[Evt DS-T18](https://docs.google.com/document/d/1ugmtrxTsFGpFLIsKX0h8d9D2JFDdGM09EA19iwpS6DA/edit):	^grz
- Baller BB Edges
- VB ???




- [[2023 Build Datasets Needed for the CV Team]]


# LOG

### 2023-10-02  Grz Data Requests

We have following models in use right now:  

1. object detection (person, hoop, ball). Martin working on extending this with backboard class
2. pose
3. OCR - detection and recognition models
4. color
5. backboard segmentation
6. player id
7. goal detection

Each model has different needs.  
Martin is actively working on 1 and 5. I'm working on 3. Sarthi might change color model (4) to non-DL solution.  


- **OBJ DETECTION** - we could always use more data for detector training (1) -- We should make a script for a given game from the annotation portal it grabs 500 random frames and add it to annotation queue at CVAT. 

- **EVENT DATASET** - we should automate extracting from game we already annotate the shots for training (7).

- **OCR** - with getting more data for OCR I would hold for a moment more

- getting more background person annotated non-players for (6) wouldn't harm but it is low priority 
- I don't think we need dedicated data for pose (2), because public datasets are good enough for us.


**Metrics**
- Execution Ratio
- Money Metric
	- Make/Miss P & R
	- Shooter ID  Acc| "True positive"
	- Points Acc | "True positive"
- Tracking
	- Track length
	- Track purity
	- track accuracy
- Halo Metric



- Where players are even when they are behind other players, (also for the ball)
- Block shots, Airballs (dont go near hoop)
- Fouls 
- Timeouts / Halftime


## GROUND TRUTH DATA COLLECT

CF: 1000 frames 176hrs
==> 10.5 minutes per frame @ $7.15 / hour.




