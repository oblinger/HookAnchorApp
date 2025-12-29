
# ML System Design

## General
- ASK: Clarifying questions about
  inputs/outputs, downstream use, tradeoffs, 
- SKETCH: High-level design
- DEBRIEF: bottleneck & scaling questions

- FULL SOLUTION - Relate back to foundational concepts
	- Efficiency
	- Monitoring
	- Harmful outputs
	- Data Preprocessing
	- Model Training
	- Building Inference infrastructure

## STEPS
1. DEFINE - Define the problem
2. PIPELINE - Design the data pipeline
3. MODEL - Create Model architecture
4. TRAIN & EVAL - Train & Eval
5. DEPLOY - Deploy model (how would you monitor)
6. WRAP UP - WrapUp Soln ()


### ML PROBLEM DEFINITION
- CONTEXT:  ACTORS, DATA: ALL, FORMAT, location, preprocess
- CONTEXT:  REQ (Fn / NonFunctional scalable, available, tooling: MLops, alerts, debugging)
- USE CASE
- REQS: Minimum accuracy & performance needed
- Traffic/Bandwidth
- Data Sources & Requirements
- Computational Resources and Constraints

### ML PIPELINE DESIGN
- Kinds of data available & needed
- How to collect data
- Feature engineering / Data Preprocessing
- Privacy concerns
- Avoiding data taint, and mix properly
- data quality 
- AWS Sagemaker; Lambda; 

### [[ML MODEL CHOICE]] 
- Identify core model type: Recommendation, Regression, Classification, Generation, Ranking
- [[DL Sys]] - 


### ML TRAINING

### ML MODEL EVALUATION
- Where it working: segment, 
- Overfit

### ML DEPLOYMENT
- Deploy when certain: A/B, Canary, Shadow, Feature Flags, 
- Knowledge Distillation, Batching Inference, 
- Detect drift
- FRAMEWORKS: NVCC (Cuda Compiler), ONNX (Triton Server), XLA for 
- CONSIDERATIONS:
	- Batching traffic
	- Simplifying (select single model from ensemble, distil model)
- MONITOR: Telemetry, GoldStanardData(w/ adds), metrics for drift, 

GPU Model Server Examples
-?-
Triton, vLLM, TorchServe

GPU Compilers Optimizer Examples
-?-
XLA, TensorRT, ONNX Runtime

### ML SYS DESIGN WRAPUP
- Debrief Prob Scope, pipeline, train, eval, and deploy
- Choices: Tradeoffs, bottlenecks, how to scale, adjust for future distribution shifts
- 


## RUBRIC FOR YOU ASSESSMENT
- Problem understanding
- Data & Feature Engineering: sourcing, labelling, limits
- Model: tradeoffs, eval reqs
- Deploy: edge/cloud, monitoring
- Collab/Comm: choices, technical concepts


#ml 
