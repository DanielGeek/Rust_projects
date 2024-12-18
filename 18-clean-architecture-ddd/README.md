# Clean Architecture (Domain Driven Design (DDD))

- Application
- Presentation
- Domain
- Infrastucture

## Docker config
```text
docker compose up -d > /dev/null 2>&1
docker ps
```

## Config Diesel CLI
```text
cargo install diesel_cli --no-default-features --features postgres
diesel --version
diesel setup(delete the initial setup)
diesel migration generate create_user > /dev/null 2>&1
```
