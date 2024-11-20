# gRPC calculator app with Tonic


## example calling with grpcurl

```
cargo run
```

```
cd server
```
```
grpcurl -plaintext \
  -proto proto/calculator.proto \
  -import-path proto \
  -d '{"a": 2, "b": 3}' \
  '[::1]:50051' \
  calculator.Calculator.Add
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
