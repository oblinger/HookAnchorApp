# Basketball Highlights API Reference

## Overview
This document provides the technical specification for the Basketball Highlights Processing API, a service that analyzes recorded basketball game videos to extract shot events and game metadata using computer vision and AI techniques.

## API Endpoints

### Authentication
All API requests require authentication using an API key provided in the request headers:
```
Authorization: Bearer <API_KEY>
```

### Submit Processing Job

**Endpoint:** `POST /api/v1/jobs`

**Description:** Submit a basketball game video for processing

**Request Body:**
```json
{
  "video_url": "https://example.com/game.mp4",
  "callback_url": "https://client.com/webhook/results",
  "processing_options": {
    "priority": "standard",  // "standard" or "expedited"
    "include_metadata": true
  }
}
```

**Response:**
```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "queued",
  "estimated_completion": "2025-08-27T14:30:00Z",
  "created_at": "2025-08-27T02:30:00Z"
}
```

**Status Codes:**
- `201 Created`: Job successfully created
- `400 Bad Request`: Invalid video URL or parameters
- `401 Unauthorized`: Invalid or missing API key
- `429 Too Many Requests`: Rate limit exceeded

### Check Job Status

**Endpoint:** `GET /api/v1/jobs/{job_id}`

**Response:**
```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "processing",  // "queued", "processing", "completed", "failed"
  "created_at": "2025-08-27T02:30:00Z",
  "updated_at": "2025-08-27T08:15:00Z",
  "estimated_completion": "2025-08-27T14:30:00Z"
}
```

## Callback Webhook

When processing is complete, the system will send a POST request to the specified callback URL with the results.

### Webhook Request Format

**Method:** `POST`

**Headers:**
```
Content-Type: application/json
X-Webhook-Signature: <HMAC-SHA256 signature>
X-Job-Id: <job_id>
```

**Body:**
```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "completed",
  "processing_time_seconds": 28800,
  "video_url": "https://example.com/game.mp4",
  "annotations": {
    "BasketballShotPrediction": [
      {
        "time": "00:00:46.200000",  // Timestamp in the video of when the shot was attempted (HH:MM:SS.microseconds)
        "version": "0.1.0",  // Version of the annotation schema
        "kind": "BasketballShotPrediction",  // Name of the annotation schema
        "id": 1,  // 1-indexed monotonically increasing integer representing the shot ID
        "player_color": "navy",  // Jersey Color of the player that attempted the shot (or null if unknown)
        "player_number": 3,  // Jersey Number of the player that attempted the shot (or null if unknown)
        "points": null,  // Points that the shot would contribute toward if made (1, 2, 3 or null if unknown)
        "made": true,  // Whether the shot was made (true) or missed (false) (or null if unknown)
        "court_coordinates": null,  // Court coordinates as (x, y) values between 0 and 1, with the origin in the top-left corner (or null if unknown)
        "metadata": {  // Additional metadata about the shot
          "hoop_id": 2,
          "ball_id": 4,
          "track_id": null,
          "shooter_id_scenario": null
        }
      }
    ]
  }
}
```

### Webhook Retry Policy
- Initial delivery attempt immediately upon completion
- Retry attempts at: 1 min, 5 min, 15 min, 1 hour, 6 hours
- Maximum of 5 retry attempts
- Exponential backoff with jitter

### Webhook Signature Verification
To verify webhook authenticity:
1. Extract the `X-Webhook-Signature` header
2. Compute HMAC-SHA256 of the request body using your webhook secret
3. Compare the computed signature with the header value

**Example (Python):**
```python
import hmac
import hashlib

def verify_webhook(body, signature, secret):
    computed = hmac.new(
        secret.encode(),
        body.encode(),
        hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(computed, signature)
```

## Data Schema

### BasketballShotPrediction Object

| Field | Type | Description | Required |
|-------|------|-------------|----------|
| `time` | string | Timestamp when shot was attempted (HH:MM:SS.microseconds) | Yes |
| `version` | string | Schema version (e.g., "0.1.0") | Yes |
| `kind` | string | Always "BasketballShotPrediction" | Yes |
| `id` | integer | 1-indexed shot identifier, monotonically increasing | Yes |
| `player_color` | string/null | Jersey color of shooter (e.g., "navy", "white") | Yes |
| `player_number` | integer/null | Jersey number of shooter | Yes |
| `points` | integer/null | Point value (1, 2, or 3) if made | Yes |
| `made` | boolean/null | Shot success (true=made, false=missed) | Yes |
| `court_coordinates` | object/null | {x: float, y: float} normalized 0-1, origin top-left | Yes |
| `metadata` | object | Additional tracking information | Yes |

### Metadata Object

| Field | Type | Description |
|-------|------|-------------|
| `hoop_id` | integer/null | Identifier for the basketball hoop |
| `ball_id` | integer/null | Identifier for the basketball |
| `track_id` | integer/null | Player tracking identifier |
| `shooter_id_scenario` | string/null | Shooting scenario classification |

## Error Handling

### Error Response Format
```json
{
  "error": {
    "code": "INVALID_VIDEO_URL",
    "message": "The provided video URL is not accessible",
    "details": {
      "url": "https://example.com/game.mp4",
      "http_status": 404
    }
  }
}
```

### Common Error Codes
| Code | Description |
|------|-------------|
| `INVALID_VIDEO_URL` | Video URL is malformed or inaccessible |
| `UNSUPPORTED_FORMAT` | Video format not supported |
| `VIDEO_TOO_LONG` | Video exceeds maximum duration |
| `RATE_LIMIT_EXCEEDED` | Too many requests |
| `INVALID_CALLBACK_URL` | Callback URL is malformed |
| `AUTHENTICATION_FAILED` | Invalid API key |
| `JOB_NOT_FOUND` | Job ID does not exist |
| `PROCESSING_FAILED` | Internal processing error |

## Rate Limits
- **Standard tier:** 10 requests per minute, 100 jobs per day
- **Premium tier:** 100 requests per minute, 1000 jobs per day
- Rate limit headers included in all responses:
  - `X-RateLimit-Limit`: Maximum requests per window
  - `X-RateLimit-Remaining`: Requests remaining
  - `X-RateLimit-Reset`: Unix timestamp when limit resets

## Video Requirements
- **Supported formats:** MP4, MOV, AVI
- **Maximum file size:** 10 GB
- **Maximum duration:** 4 hours
- **Minimum resolution:** 720p (1280x720)
- **Recommended resolution:** 1080p or higher
- **Frame rate:** Minimum 24 fps

## Processing Guarantees
- **Standard SLA:** 12-hour turnaround for 1-hour videos
- **Expedited SLA:** 4-hour turnaround for 1-hour videos (premium)
- **Availability:** 99.9% uptime
- **Data retention:** Results stored for 30 days

## API Versioning
- Current version: `v1`
- Version specified in URL path: `/api/v1/`
- Backward compatibility maintained within major versions
- Deprecation notices provided 6 months in advance
- Version sunset after 12 months deprecation period

## SDK Support
Official SDKs available for:
- Python: `pip install basketball-highlights-sdk`
- Node.js: `npm install @basketball-highlights/sdk`
- Java: Maven Central repository

## Testing Environment
- **Base URL:** `https://api-sandbox.basketball-highlights.com`
- **Test API Key:** Contact support for sandbox credentials
- **Test video URLs:** Available in developer portal
- **Webhook testing:** Use webhook.site or similar services

## Support
- **Documentation:** https://docs.basketball-highlights.com
- **Status Page:** https://status.basketball-highlights.com
- **Technical Support:** support@basketball-highlights.com
- **Response Time:** < 24 hours for standard inquiries