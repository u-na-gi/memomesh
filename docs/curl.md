```
curl -vv -X POST http://localhost:8788/api/v1/auth/login \
-H "Content-Type: application/json" \
-d '{
  "username": "test_user",
  "password": "test_password"
}'
```