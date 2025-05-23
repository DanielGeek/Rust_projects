# gRPC calculator app with Tonic


## example calling with grpcurl

```
cargo run --bin server
```
```
cargo run --bin client
```

```
cd server
```
## Sum
```
grpcurl -plaintext \
  -proto proto/calculator.proto \
  -import-path proto \
  -d '{"a": 2, "b": 3}' \
  '[::1]:50051' \
  calculator.Calculator.Add
```
## Divide
```
grpcurl -plaintext \
  -proto proto/calculator.proto \
  -import-path proto \
  -d '{"a": 2, "b": 2}' \
  '[::1]:50051' \
  calculator.Calculator.Divide
```
## Admin Counter without token
```
grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
```
## Admin Counter with token
```
grpcurl -H "Authorization: Bearer some-secret-token" -emit-defaults -plaintext '[::1]:50051' calculator.Admin GetRequestCount
```

## using reflection
```
grpcurl -plaintext \
  -d '{"a": 3, "b": 4}' \
  '[::1]:50051' \
```

## List gRPC services
```
grpcurl -plaintext '[::1]:50051' list
```
