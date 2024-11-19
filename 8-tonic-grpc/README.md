# gRPC calculator app with Tonic


## example calling with grpcurl
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
