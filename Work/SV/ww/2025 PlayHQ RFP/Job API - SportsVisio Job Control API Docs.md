# SportsVisio Job Control API Docs

# Overview

This document provides the technical specification for the AI Job API, a general job submission and tracking system for AI-related tasks.

# Security

- All communication with the API will be done over HTTPs
- All requests to the API will use bearer authentication using an API key as a token in the `Authorization` header:
    
    ```
    Authorization: Bearer <API_KEY>
    ```
    
- Webhook requests will include a signature from the API and can be verified by the client using a webhook secret
    - The signature will be stored in the header `X-Hub-Signature-256` and start with `sha256=`
    - Validation should be done by computing the `HMAC-SHA256` of the request’s body + the webhook secret following [Github’s standard](https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries#validating-webhook-deliveries)
- For v1, we will directly provide an API key (no API key management endpoint)

# API Endpoints

## Submit Processing Job

**Endpoint:** `POST /v1/jobs` 

**Description:** Submit a job for processing.

**Content type:** application/json

**Request Body:**

```json
{
	"job": {	// Job spec. The structure will depend on the type of job to submit. See the documentation of specific job types for details
		"type": "the-job-type",  // Type of job to submit
		...
	},
	"webhooks": [  // Optional webhooks that will be called on certain job events
	  {
		  "url": "<https://client.com/webhook/results>",  // URL where the webhook POST request will be sent to
		  "secret": "abcdef",  // Secret that'll be used to sign the webhook request
		  "events": ["job.fail", "job.success"]  // Events on which the webhook will be called. Currently only "job.fail" and "job.success" are supported
	  },
	],
}

```

**Response body:**

```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",  // Unique UUIDv4 identifier for this job
  "created_at": "YYYY-MM-DDTHH:MM:SSZ",  // Timestamp of the job's creation in ISO 8601 format 
  "estimated_completion": "YYYY-MM-DDTHH:MM:SSZ"  // Timestamp of the estimated job's completion in ISO 8601 format
}

```

## Check Job Status

**Endpoint:** `GET /v1/jobs/{job_id}`

**Description:** Get the status, errors, and result data associated with a job.

**Response body:**

```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",  // Unique job ID
  "job_type": "the-job-type",  // The type of the job
  "status": "processing",  // The job's current status: "queued", "processing", "completed", "failed"
  "created_at": "YYYY-MM-DDTHH:MM:SSZ",  // Timestamp of the job's creation in ISO 8601 format
  "updated_at": "YYYY-MM-DDTHH:MM:SSZ",// Timestamp of the job's latest update in ISO 8601 format
  "estimated_completion": "2025-08-27T14:30:00Z",  // Timestamp of the estimated job's completion in ISO 8601 format
  "errors": [  // Errors associated with the job (or null)
	  {
		  "code": "error_code",
		  "timestamp": "YYYY-MM-DDTHH:MM:SSZ",  // Timestamp when the error occurred in ISO 8601 format
		  "details": "Detailed description of the error."
	  }
  ],
  "results": {...}  // Results associated with the job if available (or null)
}
```

- The job’s `results` structure will depend on its `job_type` . Check documentation of the specific `job_type` for details.

## Check All Jobs’ status

**Endpoint:** `GET /v1/jobs`

**Description:** Get a list of the jobs currently with a status of “queued” or “processing”.

**Response body:**

```json
[
	{
		"job_id": "550e8400-e29b-41d4-a716-446655440000",
		"job_type": "the-job-type",
	  "status": "processing",  // The job's current status: "queued", "processing", "completed", "failed"
	  "created_at": "YYYY-MM-DDTHH:MM:SSZ",  // Timestamp of the job's creation in ISO 8601 format
	  "updated_at": "YYYY-MM-DDTHH:MM:SSZ",// Timestamp of the job's latest update in ISO 8601 format
	  "estimated_completion": "2025-08-27T14:30:00Z"  // Timestamp of the estimated job's completion in ISO 8601 format
	},
	{
		"job_id": "other-job-id",
		"job_type": "other-job-type",
	  "status": "queued",
	  "created_at": "YYYY-MM-DDTHH:MM:SSZ",
	  "updated_at": "YYYY-MM-DDTHH:MM:SSZ",
	  "estimated_completion": "2025-08-27T14:30:00Z"
	},
]
```

# Webhooks

## Job status webhook

When processing is complete (or it fails), the system will send a POST request to the specified callback URL with the results. The request’s contents is the same as calling `GET /v1/jobs/{job_id}` plus an additional `webhook_timestamp` field containing the timestamp when the webhook was called.

### Webhook Request Format

**Method:** `POST`

**Headers:**

```
Content-Type: application/json
X-Hub-Signature-256: <HMAC-SHA256 signature>
```

**Body:**

```json
{
  "webhook_timestamp": "YYYY-MM-DDTHH:MM:SSZ",  // Timestamp of the webhook call in ISO 8601 format
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "job_type": "the-job-type",
  "status": "completed",
  "created_at": "YYYY-MM-DDTHH:MM:SSZ",
  "updated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "estimated_completion": "2025-08-27T14:30:00Z",
  "errors": [],
  "results": {...}
}

```

## Webhook Retry Policy

- Initial delivery attempt immediately upon completion
- If the webhook request doesn’t result in a 200 code, the backend will attempt up to 5 retries
    - Time between retries will be 5 minutes with exponential backoff and jitter
    - i.e. 1 min, 5 min, 25 min, 125 min, 625 min

# Rate Limits

Rate limits depend on the type of operation being performed in the API.

All responses will contain the following two additional headers:

```
X-RateLimit-Limit: <Total number of requests allowed>
X-RateLimit-Remaining: <Number of requests remaining until the limit is reached>
```

# Error handling

The API follows the standard for HTTP status codes when returning responses.

## Standard HTTP status codes in use

| Status code | Name | Description |
| --- | --- | --- |
| 200 | OK | Request went through as expected. |
| 400 | Bad Request | There was a problem with the request’s structure/values. |
| 401 | Unauthorized | No valid API key found in request. |
| 404 | Not Found | The requested resource was not found. |
| 422 | Request validation error | Request has wrong format |
| 429 | Too Many Requests | Too many requests were attempted too quickly. |
| 500 | Internal Server Error | An error internal to the server. |

## Internal error codes

All non-200 responses bodies will follow the same json structure:

```json
{
	"error": {
		"code": "api_error_code",
		"details": "Detailed intormation about the error."
	}
}
```

| Error code | Description |
| --- | --- |
| `video_invalid_url` | Video URL is malformed or inaccessible |
| `video_unsupported_format` | Video format not supported. |
| `video_too_long` | Video exceeds maximum duration. |
| `callback_invalid_url` | Callback URL is malformed. |
| `processing_failed` | Internal processing error. |
| `other`  | Miscellaneous error. |

# API Versioning

- Current version: `v1`
- Version specified in URL path: `/v1/`
- Backward compatibility maintained within major versions
- Deprecation notices provided 6 months in advance
- Version sunset after 12 months deprecation period

# Environment

- Base URL: `https://ai.sportsvisio-api.com`
- **Test API Key:** Contact support for sandbox credentials
- **Test video URLs:** Available in developer portal

# Support

- **Documentation:** [https://sportsvisio.com](https://docs.basketball-highlights.com/)
- **Technical Support:** [support@sportsvisio.com](mailto:support@basketball-highlights.com)
- **Response Time:** < 1 business day for standard inquiries

# Changelog

- September 17, 2025 - Added webhook’s secret in the job creation request
- September 25, 2025 - Add `job_type` to the job’s status and remove any job-specific structures