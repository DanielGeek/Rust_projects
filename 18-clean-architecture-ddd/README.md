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
diesel migration run (After created table)
```

## Test it
```text
cargo run
```

## API Endpoints
```text
POST user: http://localhost:4000/api/v1/user/
{
    "name": "Jack",
    "phone": "0900",
    "email": "Jack@semicolon.com",
    "address": "08-04 xyz tower"
}
```

```text
GET user: http://localhost:4000/api/v1/user/{email_user}
```
