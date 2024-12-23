curl -X POST http://localhost:8080/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "character_id": "char123",
    "message": "Hello, this is a test message!"
  }'
