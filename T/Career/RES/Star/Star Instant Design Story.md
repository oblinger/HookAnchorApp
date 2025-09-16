
# ( Cross posted to [[War]]; which is now master )


#### Instant Design Story

1. Non-ML-based Edge extraction
2. Hand-built energy-based edge cleanup
3. NN-based obstruction detection
4. Combine NN & Geo edge detection


**Edge Extraction**
- Find flat planes
- glue together
- derive edges

**Energy-based cleanup**
- remove small planes
- remove super sharp corners
- replace line pairs that are nearly 180-degrees
- (sides of roof were not symmetric; fixed this for sales)

**NN MODELS** - pixel level models (RGB+depth)
	- Tree detection
	- obstruction dectection
	- pixel based-edge detection   (weakest model)

NN-models
- Do batch normalization
- Use residual connection
- grab resnet-50.
- Started with plain units.
- We always trained from scratch 

- do contrast learning (never did that to do pretraining)
- 2% delta from best to worse model, so we needed to care.  (semantics segmentation accracy at the pixel level)  
	- was 97 - 98% at pixel level

ultimate failures came from mis labelling one part of roof and failed from there (e.g. mis-labelled one part... and it kills )

- IDEA:  Do object detect, instead of  (use mask CNN to do it)
	- Grabbed the RTR from facebook


OTHER IDEAS:
- RoofGan paper: [https://arxiv.org/abs/2012.09340](https://arxiv.org/abs/2012.09340)  code: [https://github.com/yi-ming-qian/roofgan](https://github.com/yi-ming-qian/roofgan)
- Object detection:  DETR: [https://github.com/facebookresearch/detr](https://github.com/facebookresearch/detr)
- RTF frome facebook

- Hand written  CV
	- derivative of DSM data, flat planes  (Grz Bartchek; MIT)
	- hand written CV geo algorithms
	- apply RGB data (Energy minimization)
		- corners should meet at non-nearly straight angles (energy constraints)
	- NN for detecting obstructions
		- then idea lets swap edges from NN into the existing pipeline for 

