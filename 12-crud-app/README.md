# CRUD App with Rust tokio, axum, Postgres SQL, Docker and Kubernetes

## Methods
- POST example Body JSON
- PATH: http://localhost:3000/user
```
{
  "name": "Dave",
  "occupation": "Doctor",
  "email": "Dave@example.com",
  "phone": "+92-332-40506"
}
```

- GET example All
- PATH: http://localhost:3000/users
<br />
- GET example By Id
- PATH http://localhost:3000/user/2
<br />
- PUT example Body JSON
- PATH: http://localhost:3000/user/1
```
{
  "name": "David",
  "occupation": "Engineer",
  "email": "david@example.com",
  "phone": "+92-332-40506"
}
```
- DELETE example By Id
- PATH http://localhost:3000/user/1