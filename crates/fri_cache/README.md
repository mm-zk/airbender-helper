
## Example commands

curl -X POST http://127.0.0.1:8085/sendFRI \
  -H "Content-Type: application/json" \
  -d '{
    "public_input": "user123",
    "bytecode_hash": "abcd1234",
    "payload": "This is my FRI payload"
  }'



curl "http://127.0.0.1:8085/getFRI?public_input=user123&bytecode_hash=abcd1234"

