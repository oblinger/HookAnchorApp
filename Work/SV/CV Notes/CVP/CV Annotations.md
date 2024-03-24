




# LOG 

### 2023-10-13  Timings

ANNOTATION COST ESTIMATOR
So @maxim using your data [[HERE]] as input, I derive a labor multiplier as follows:

3619 frames in 8 hrs is 3619/8/60 = 
-  7.5 frames/min_labor

Assuming 10 fps in some target video was sufficient then:
- 600 frames/min_output = 10 frame/sec_output * 60 sec/min

Labor Multiplier = 80x
- min_labor/min_output = 600 frames/min_output / 7.5 frames/min_labor

Now using this 80x labor multiplier we see 10 videos of 10 minutes should be 100/60 hours of video which should take:
   133 task_time = 100/60 hours * 80



  
Report from baseball task:  
  
**Ball detection dataset**

- 1 o 2 bboxes per frame
    
- no overlap and no occlusion, the object is clearly visible most of the time  
      
    Created 3 tasks in CVAT. Task 1 was the first one, used to explain the goals and demonstrate the usage of the tool to new team members.  
      
    Aprox. 1 min of video per task (video from task 1 was sampled 30fps) the videos for task 2 and 3 at 60 fps (original frame rate).
    

Task 2 and 3 were deployed at the same time so two annotators worked simultaneously on them.

- Task 1: 1700 frames.
    
    - 8hs to accomplish: 1 annotator
        
- Task 2: 3480 frames
    
- Task 3: 3619 frames
    
    - The average time informed by annotations team was 8hs.  
        NOTE: need to confirm if 8hs per task or for both at the same time.
        

NOTE 2: The informed hs are TOTAL hs including QC.



### 2023-10-13  notes

ANNOTATION - Metadata about a game potentially used in computing the stats for that game.
- Intended to generalize across all sports
- Broken out by type, but inheriting from a standardized core list of fields
- Expressed in APIs as a JSON object with the following extensible list of potential fields:



STANDARDIZED FIELDS
- type
- value				
- video_time
- game_time

