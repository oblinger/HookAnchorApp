#pp  [PhD](https://beatlab.mcmaster.ca/assets/Shreshth_Saxena_CV.pdf)  @McHaster.  
shreshth.stan@gmail.com

- https://beatlab.mcmaster.ca/assets/Shreshth_Saxena_CV.pdf

- [Saxena](spot://saxena)

### TODO
- NDA, Ripling setup
- Brainstorming session with Greg, Dan & Juan

# LOG

### 2024-03-29  SOW

Objective:  Make a material improvement in our ability to predict player IDs and specifically to predict shooter IDs.  Improvement should be measured as improvements in F1 or area under precision-recall curve as measured on the LT10h or other relevant dataset.

Context:  
- We have trained and use a specialized double digit jersey number reader that is systematically used to check a region of the expected chest area for bounding boxes when their pose indicates they are facing towards or a away from the camera.
- Using a tracker we extend these single OCR detections into a sequence of player detections from the subset of the tracklet that we are confident have not been mixed with other tracklets (based on the geometry of the tracklets in image space)
- Resulting datasets are used to train a "gallery" player images for each unique player, and this is used to perform supervised induction 

### 2024-03-19  Intro

- worked with Guy @Valerie.   @ Eliete Sport. 


- Whats to become an applied researchers
- would love to be publishing in computational CogSci
- Neurips, Behavior, Chi, Etra(sub community)

- audio visual attention (his current research)


CV stuff
- State-based CNN
- lung cancer prediction (standard model)
- object detection and tracking and segmentation

internship with Inspect labs in India
- train GANs to generate images of scratched car in order to determine parts.
	- Use classic one
	- they drift too easily 

Sports vision app
- Ball model  -  Mobile Net - light weight ML
	- CNN - model
	- its a better version of CoreML
	- had to learn swift


Now doing eye-tracking on a browser
- trying to use WebGPU
- using posenet, fixed it to run on iOS
- like SURF. ()


- Current Project:  https://livelab.mcmaster.ca/events/livelab-10th-anniversary-the-innocents/
- https://github.com/ShreshthSaxena/TrackSS
- 






