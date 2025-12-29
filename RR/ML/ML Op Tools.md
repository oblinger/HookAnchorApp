
#ml2 #learn

# Tools
(From https://www.datacamp.com/blog/top-mlops-tools)




## DATA PIPELINING
### [DVC](https://engineering.rappi.com/how-to-build-an-efficient-machine-learning-project-workflow-using-data-version-control-dvc-aaeaa9cfb79b)  - Data pipelining 
### [Pachyderm](https://www.pachyderm.com)  - 



## Visualization

[Medium Article](https://engineering.rappi.com/a-complete-mlops-toolbox-9c37f8ef5500)  

### Seaborn - visualization
- [Gallery](https://seaborn.pydata.org/examples/index.html)  
### [Plotly](https://plotly.com/python/)  
### [amCharts](https://www.amcharts.com/demos/) - HTML/JS charting
### [Missingno](https://www.geeksforgeeks.org/python-visualize-missing-values-nan-values-using-missingno-library/) - visualized missing data

## LLM Frameworks

[Qdrant](https://github.com/qdrant/qdrant);;; an open-source vector similarity search engine and vector database that provides a production-ready service with a convenient API, allowing you to store, search, and manage vector embeddings.
<!--SR:!2024-09-25,1,230!2024-09-25,1,230-->

[LangChain](https://www.langchain.com/);;; a versatile and powerful framework for developing applications powered by language models. It offers several components to enable developers to build, deploy, and monitor context-aware and reasoning-based applications.
<!--SR:!2024-10-16,8,250!2024-09-27,3,250-->

## Experiment Tracking

[MLflow](https://mlflow.org/);;; Experiment Tracking - an open-source tool that helps you manage core parts of the machine learning lifecycle. It is generally used for experiment tracking, but you can also use it for reproducibility, deployment, and model registry. You can manage the machine learning experiments and model metadata by using CLI, Python, R, Java, and REST API.
<!--SR:!2024-09-27,3,250!2024-11-01,5,230-->

[Comet ML](https://www.comet.com/site/);;; Experiment Tracking - a platform for tracking, comparing, explaining, and optimizing machine learning models and experiments. You can use it with any machine learning library, such as Scikit-learn, Pytorch, TensorFlow, and HuggingFace.
<!--SR:!2024-09-25,1,230!2024-09-25,1,230-->

[Weights & Biases](https://wandb.ai/site);;; Experiment Tracking - an ML platform for experiment tracking, data and model versioning, hyperparameter optimization, and model management. Furthermore, you can use it to log artifacts (datasets, models, dependencies, pipelines, and results) and visualize the datasets (audio, visual, text, and tabular).
<!--SR:!2024-10-09,1,210!2024-10-09,1,210-->

## ## Orchestration and Workflow Pipelines MLOps Tools

[Prefect](https://www.prefect.io/);;; ML Workflow - a modern data stack for monitoring, coordinating, and orchestrating workflows between and across applications. It is an open-source, lightweight tool built for end-to-end machine-learning pipelines.
<!--SR:!2024-10-10,2,230!2024-11-02,1,210-->

[Metaflow](https://metaflow.org/);;; ML Workflow - a powerful, battle-hardened workflow management tool for data science and machine learning projects. It was built for data scientists so they can focus on building models instead of worrying about MLOps engineering.
<!--SR:!2024-11-08,7,250!2024-09-25,1,230-->

[Kedro](https://kedro.org/);;; ML Workflow - a workflow orchestration tool based on Python. You can use it for creating reproducible, maintainable, and modular data science projects. It integrates the concepts from software engineering into machine learning, such as modularity, separation of concerns, and versioning.
<!--SR:!2024-10-29,2,230!2024-10-09,1,210-->

[Pachyderm](https://www.pachyderm.com/);;; ML Workflow - automates data transformation with data versioning, lineage, and end-to-end pipelines on Kubernetes. You can integrate with any data (Images, logs, video, CSVs), any language (Python, R, SQL, C/C++), and at any scale (Petabytes of data, thousands of jobs).
<!--SR:!2024-11-02,1,210!2024-09-25,1,230-->

## Data Pipelining

[Data Version Control](https://dvc.org/);;; Data Pipelining - an open-source and popular tool for machine learning projects. It works seamlessly with Git to provide you with code, data, model, metadata, and pipeline versioning.
<!--SR:!2024-09-28,4,270!2024-10-28,1,210-->

LakeFS;;; Data Pipelining -  an open-source scalable data version control tool that provides a Git-like version control interface for object storage, enabling users to manage their data lakes as they would their code. With LakeFS, users can version control data at exabyte scale, making it a highly scalable solution for managing large data lakes.
<!--SR:!2024-09-25,1,230!2024-11-02,1,210-->

[AirFlow](https://airflow.apache.org);;; Authors, Monitors, Schedules workflows
<!--SR:!2024-10-28,1,210!2024-09-25,1,230-->

## Feature Stores

[Feast](https://github.com/feast-dev/feast);;; Feature Store - an open-source feature store that helps machine learning teams productionize real-time models and build a feature platform that promotes collaboration between engineers and data scientists.
<!--SR:!2024-09-25,1,230!2024-09-27,3,250-->

[Featureform](https://github.com/featureform/featureform);;; Feature Store - a virtual feature store that enables data scientists to define, manage, and serve their ML model's features. It can help data science teams to enhance collaboration, organize experimentation, facilitate deployment, increase reliability, and preserve compliance.
<!--SR:!2024-09-27,3,250!2024-09-27,3,250-->

## Model Testing

## Model Deployment

[Kubeflow](https://www.kubeflow.org/docs/);;; Model Deployment - makes machine learning model deployment on Kubernetes simple, portable, and scalable. You can use it for data preparation, model training, model optimization, prediction serving, and motor the model performance in production. You can deploy machine learning workflow locally, on-premises, or to the cloud. In short, it makes Kubernetes easy for data science teams.
<!--SR:!2024-11-24,28,270!2024-12-22,2,230-->

[BentoML](https://www.bentoml.com/);;; Model Deployment - makes it easy and faster to ship machine learning applications. It is a Python-first tool for deploying and maintaining APIs in production. It scales with powerful optimizations by running parallel inference and adaptive batching and provides hardware acceleration.
<!--SR:!2024-09-27,3,250!2024-10-09,1,210-->

[Hugging Face Inference Endpoints](https://huggingface.co/docs/inference-endpoints/guides/access);;; Model Deployment - a cloud-based service offered by Hugging Face, an all-in-one ML platform that enables users to train, host, and share models, datasets, and demos. These endpoints are designed to help users deploy their trained machine learning models for inference without the need to set up and manage the required infrastructure.
<!--SR:!2024-09-25,1,230!2024-09-27,3,250-->

## Model Monitoring

## Runtime Engines

## End to End ML Ops Platforms

[Amazon Web Services SageMaker](https://aws.amazon.com/sagemaker/mlops/?sagemaker-data-wrangler-whats-new.sort-by=item.additionalFields.postDateTime&sagemaker-data-wrangler-whats-new.sort-order=desc);;; ML Op Framework - a one-stop solution for MLOps from Amazon. You can train and accelerate model development, track and version experiments, catalog ML artifacts, integrate CI/CD ML pipelines, and deploy, serve, and monitor models in production seamlessly.
<!--SR:!2024-11-04,8,250!2024-09-27,3,250-->

[DagsHub](https://dagshub.com/);;; ML Op Framework - a platform made for the machine learning community to track and version the data, models, experiments, ML pipelines, and code. It allows your team to build, review, and share machine-learning projects.
<!--SR:!2024-09-25,1,230!2024-09-25,1,230-->

[Iguazio MLOps Platform](https://www.iguazio.com/platform/);;; ML Op Framework - an end-to-end MLOps platform that enables organizations to automate the machine learning pipeline from data collection and preparation to training, deployment, and monitoring in production. It provides an open ([MLRun](https://github.com/mlrun/mlrun)) and managed platform.
<!--SR:!2024-11-12,11,270!2024-10-28,1,210-->