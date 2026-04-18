# Edge Node HTTP service

## What this service does

This service receives sensor readings over HTTP, validates them, and returns a moving average.

- Exposes one endpoint: `POST /data`
- Accepts JSON payloads with:
	- `sensor_id` (string)
	- `value` (number)
	- `timestamp` (UTC format: `YYYY-MM-DDTHH:MM:SSZ`)
- Validates incoming data:
	- Requires `Content-Type: application/json`
	- Rejects malformed or schema-invalid JSON
	- Ensures `timestamp` matches the required UTC format
	- Ensures `value` is within configured `min_threshold` and `max_threshold`
- Maintains an in-memory rolling window of the latest 10 values
- Returns a successful response with:
	- `moving_average`: average of the rolling window
	- `timesamp`: server timestamp when the response is created

### Response behavior

- `200 OK`: data accepted and processed
- `400 Bad Request`: malformed JSON or wrong JSON structure
- `415 Unsupported Media Type`: missing/invalid `Content-Type`
- `422 Unprocessable Entity`: timestamp/value validation failed
- `500 Internal Server Error`: unexpected extraction/processing error

### Configuration

The service reads `config.json` at startup for:

- `ip` and `port` to bind the HTTP server
- `min_threshold` and `max_threshold` for value validation

## Installation guide
1. Clone the repository

```bash
git clone <your-repo-url>
cd edge_node_http_service
```

2. Configure `config.json`

Edit `config.json` and set these values for your environment:

- `ip`: interface to bind the server (for Docker, keep `0.0.0.0`)
- `port`: HTTP port exposed by the service
- `min_threshold`: minimum accepted sensor value
- `max_threshold`: maximum accepted sensor value

Example:

```json
{
	"ip": "0.0.0.0",
	"port": 8080,
	"min_threshold": -40,
	"max_threshold": 50
}
```

3. Start the service with Docker Compose

```bash
docker compose up --build
```
