echo "Sending good requests..."
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-1","value":42.5,"timestamp":"2026-04-17T12:00:00Z"}'
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-2","value":2.5,"timestamp":"2026-04-17T12:00:00Z"}'
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'

echo "Sending bad requests..."
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: aplication/json" -d '{"sensor_id": "s-1","value":42.5,"timestamp":"2026-04-17T12:00:00Z"}'
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensr_id": "s-2","value":2.5,"timestamp":"2026-04-17T12:00:00Z"}'
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","vlue":12.5,"timestamp":"2026-04-17T12:00:00Z"}'