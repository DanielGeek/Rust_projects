# CRUD App with Rust tokio, axum, Postgres SQL, Docker and Kubernetes

## Run project
```
cargo run
```

## Config docker
```
sudo docker build . -t crud:latest
```

## Run docker container
```
cd docker
docker compose up
```

## Conf Kubernetes db
```
cd deployment
cd db
kubectl apply -f db-namespace.yaml
kubectl get namespaces
kubectl apply -f db-deployment.yaml -n db-namespace
kubectl get pods -n db-namespace
kubectl describe pod postgres-deployment-6b5bbfb784-xdz85  -n db-namespace

kubectl apply -f db-service.yaml -n db-namespace
kubectl get service -n db-namespace
```

## Conect to Kubernetes container postgres deploy
```
kubectl exec -it postgres-deployment-6b5bbfb784-xdz85 -c postgres -n db-namespace -- bash
psql -U postgres info_db
\dt
\q
exit
```

## Conf Kubernetes app
```
cd deployment
cd app
kubectl apply -f app-namespace.yaml
kubectl get namespace
kubectl apply -f app-deployment.yaml -n app-namespace
get pods -n app-namespace
kubectl logs rust-app-deployment-66dd5fc55b-vdvgc -c rust-app -n app-namespace
kubectl apply -f app-service.yaml
kubectl get services -n app-namespace
```

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