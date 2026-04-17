
echo "Sending bad requests..."
echo -e "\n-----------------------------\nSending request with bad header"
curl -i -X POST http://127.0.0.1:8080/data -H "Content-Type: aplication/json" -d '{"sensor_id": "s-1","value":42.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo -e "\n-----------------------------\nSending request with bad json key"
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensr_id": "s-2","value":2.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo -e "\n-----------------------------\nSending request with missing json key"
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo -e "\n-----------------------------\nSending request with bad json syntax"
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"value":12.5 "timestamp":"2026-04-17T12:00:00Z"}'
echo -e "\n-----------------------------\nSending request with bad time format"
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-1","value":42.5,"timestamp":"2026-d4-17T12:00:00Z"}'
echo -e "\n-----------------------------\nSending request with value as a string"
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-1","value":"42.5","timestamp":"2026-04-17T12:00:00Z"}'
echo -e "\n-----------------------------\nSending request with value higher than default threshold"
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-1","value":142.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo -e "\n-----------------------------\nSending request with value lower than default threshold"
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-1","value":-42.5,"timestamp":"2026-04-17T12:00:00Z"}'

echo "Sending good requests..."
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-1","value":42.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-2","value":2.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""
curl -i -X POST http://127.0.0.1:3000/data -H "Content-Type: application/json" -d '{"sensor_id": "s-3","value":12.5,"timestamp":"2026-04-17T12:00:00Z"}'
echo ""

