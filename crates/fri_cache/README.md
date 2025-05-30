
## Example commands

```shell

curl -X POST http://127.0.0.1:8085/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "sendFRI",
    "params": {
      "public_input": "user123",
      "bytecode_hash": "abcd1234",
      "payload": "This is my FRI payload"
    },
    "id": 1
  }'


curl -X POST http://127.0.0.1:8085/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "getFRI",
    "params": {
      "public_input": "user123",
      "bytecode_hash": "abcd1234"
    },
    "id": 2
  }'
```