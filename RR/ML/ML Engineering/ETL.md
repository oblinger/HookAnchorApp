

ETL
-?-
Extract, Transform, Load
A data pipeline process for preparing raw data for ML models into an ML ready datastore (snowflake, S3, BigQuery)


ETL Tools
-?-
Apache Airflow (DAG workflow),    AWS Glue, G Cloud Dataflow, Azure Data Factory
dbt (Data Build Tool) SQL&Version Control
BigData:  Apache Spark (batch/streaming) / Flink (Stream) / Beam (unified batch/streaming API)
ML way:  Airflow -> Spark -> Feast -> MLflow (ML pipelines)


  ---
  The Three Steps:

  1. Extract

  Pull data from various sources:
  - Databases (SQL, NoSQL)
  - APIs
  - Files (CSV, JSON, Parquet)
  - Streaming sources (Kafka, Kinesis)
  - Web scraping

  2. Transform

  Clean and prepare data for ML:
  - Handle missing values
  - Remove duplicates
  - Feature engineering
  - Normalize/standardize
  - Encode categorical variables
  - Join/aggregate datasets
  - Filter outliers

  1. Load

  Store processed data:
  - Data warehouse (Snowflake, BigQuery)
  - Feature store (Feast, Tecton)
  - Training dataset storage (S3, GCS)
  - Model-ready format (TFRecord, Parquet)

  ---
  ML-Specific Considerations:

  - Feature stores replace traditional data warehouses
  - ELT (Extract-Load-Transform) increasingly common with cloud data warehouses
  - Real-time transformations for online inference
  - Version control for reproducibility
  - Data validation to catch distribution shifts

  ---
  Common Tools:

  - Batch: Apache Airflow, Prefect, Luigi
  - Streaming: Apache Spark, Flink, Beam
  - Cloud: AWS Glue, Azure Data Factory, GCP Dataflow
  - ML-focused: Feast, Tecton, Hopsworks

  ---
  Bottom line: ETL is the data plumbing that gets messy real-world data into clean, ML-ready format.