# graphql-rust
Example using GraphQL APIs in RUST

## Run App
```
cargo run
```

## Query examples

## Post get user by id
```
query dump($id: ID) {
    getUser(id: $id) {
        name,
        phone,
        organization
    }
}
GRAPHQL VARIABLES
{"id": "1"}
```

## Post get users
```
query dump {
    getUsers {
        name,
        phone,
        organization,
        email
    }
}

```