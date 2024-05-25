# Rust – CRUD API with SQLX and PostgreSQL

Description: Set up a PostgreSQL server with Docker, migrate the SQL queries to the database using SQLX-CLI, build an API that runs on the Actix-web HTTP server, and persist data in the Postgres database using SQLX

References: https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/


- run project `make run`

# API paths
# Get Method
curl -X GET http://localhost:8080/api/games | json_pp

# GetById Method
curl -X GET http://localhost:8080/api/games/game/{id} | json_pp

# Create Method
curl -X POST http://localhost:8080/api/games/game \
-H 'Content-Type: application/json' \
-d '{
    "field_name":"Soccer Planet", 
    "address": "444 NW Miami FL 32342", 
    "day": "Mon 12-00"
}'

# Update Method
curl -X PUT http://localhost:8080/api/games/game/9e5fba4c-55a1-4167-8980-7a6d371bfc8a \
-H 'Content-Type: application/json' \
-d '{
    "field_name":"Soccer Planet UPDATED",
    "address": "UPDATED ADDRESS" 
}'

curl -X DELETE http://localhost:8080/api/games/game/effcf693-8b6e-403a-853f-e77e399a330d \