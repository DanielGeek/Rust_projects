# Rust Axum V0.0.6.1 projects

Axum is a backend API framework for Rust. It's written by the same team that makes Tokio.rs and therefore is 100% compatible with Tokio. It's meant to be easy to get started with but powerful enough to run your project on.

## Help commands

```bash
cargo watch -x run
cargo doc
cargo doc --open
cargo add serde@1.0.147
cargo add serde@1.0.147 -F derive
```

## Help Docker Commands

```bash
docker compose up
docker compose exec database /bin/bash
psql -U postgres -d postgres
\dl
\dt
select * from tasks;
exit;
docker compose down
docker volume ls
docker volume rm 20-axum_db-data
docker compose up -d --wait
docker compose logs database
```

## SeaORM CLI help commands

```bash
# You will need set your DATABASE_URL Connection inside your .env file
# Inside folder lessons/data or project_solution folder execute the following command
sea-orm-cli generate entity -o src/database
```

## Legend

- [x] Unit
  - [x] Standard
    - [x] Lesson
    - [x] ***Why Rust / Why Axum?***
      - Strong type system
      - Tooling
        - compiler messages
        - rust analyzer
        - package manager
      - Fast development time (once you familiar with the framework / language)
      - Fun
- [x] *Hello world*
  - [x] *Spin up a hello world server*f
    - [x] ***Installing Rust (cancelled)***
    - [x] *Using the course repo*
    - [x] ***Setting up VS Code***
    - [x] ***Create hello world Axum server***
    - [x] ***Auto restart the server***
    - [x] ***Open the documentation***
- [x] Routing
  - [x] ***Create a router to handle http methods***
    - [x] ***handling HTTP methods***
  - [x] ***Create an extractor to get a string from the body***
    - [x] ***Extracting string from body***
  - [x] ***Create an extractor to get JSON from the body***
    - [x] ***Receiving JSON in a Post***
  - [x] ***Create an extractor to get a path variable from a request***
    - [x] ***Handling path variables***
  - [x] ***Create an extractor to get query parameters***
    - [x] ***Handling query params***
  - [x] ***Create an extractor to get the headers***
    - [x] ***Extracting the User Agent Header***
    - [x] ***Extracting a Custom Header***
  - [x] ***Apply middleware to routes***
    - [x] ***Set CORS headers***
    - [x] ***Using layers to share data between routes***
    - [x] ***Creating middleware function***
      - Explore creating custom middleware again
      - During recording state that versions are very important
      - New version of Axum will change how middleware works
      - WARNING: Derive Clone on struct being added to Extensions
  - [x] ***Return the appropriate status code and message when erroring***
    - [x] ***Returning error status codes***
    - [x] ***Returning success status codes***
  - [x] ***Return JSON data***
    - [x] ***Respond with JSON data***
  - [x] ***Validate incoming data***
    - [x] ***Validating JSON with Serde***
    - [x] ***Custom Extractor with Validation***
- [x] Data
  - [x] ***Creating a database***
    - [x] ***Set up a local database with Docker***
  - [x] ***Connect a database***
    - [x] ***Introducing SeaORM***
    - [x] ***Connecting to the database***
    - [x] ***SeaORM Models***
    - [x] ***Passing Database To Handlers***
  - [x] ***CRUD data in the database including soft deletions***
    - [x] ***Create a row in the database***
    - [x] ***Get one item from the database***
    - [x] ***Get all items from the database***
    - [x] ***Using filters***
    - [x] ***Atomic updates***
    - [x] ***Patch updates (maybe re-record using into_active_value?)***
    - [x] ***Deleting data***
    - [x] ***Soft-deleting data***
- [x] ***Security***
  - [x] ***Authentication***
    - [x] ***How auth works***
      - [x] ***creating account***
      - [x] ***login***
      - [x] ***guard route***
      - [x] ***logout***
      - [x] ***use middleware***
    - [x] ***Make it secure***
      - [x] ***Hashing the password***
      - [x] ***use a JWT***
- [x] ***Helper Utilities***
  - [x] ***Custom errors***
- [x] ***Devops***
  - [x] ***Deployment***
    - to own server
    - VPC
    - Container service
    - TLS
- [x] ***Addendums***
  - [x] ***Updating Router lessons to Axum 0.6***
    - [x] Update version
    - [x] Router function returns Stateless Router
    - [x] Verify everything is working with Postman
    - [x] Use App State instead of Extension
      - middleware_message
  - [x] ***Updating Data lessons to Axum 0.6***
    - [x] Update version
    - [x] Custom JSON extractor
      - Add missing generic
      - Add state to function signature
      - Brought in Extract trait
      - Added _ to type for extract
    - [x] Return stateless router
    - [x] Move body consuming extractors to the bottom of function signatures
    - [x] Custom JSON extractor again
      - Add missing 'static lifetime bound
    - [x] Full test
    - [x] Put DatabaseConnection in AppState
      - Update handlers to pull in State
      - Update guard middleware to use state
    - [x] Update guard middleware to use extractors directly
- [ ] *Project*
  - [ ] *Create a server that passes all tests*
    - [x] ***Introduce the project***
    - [x] ***Hello world with tests***
    - [x] ***Create user***
    - [x] ***Handle duplicate username***
    - [x] ***sign in***
    - [x] ***handling bad username/password***
    - [x] ***logging out***
    - [x] ***testing bad tokens***
    - [x] ***creating a task***
    - [x] ***getting all tasks***
    - [x] ***get one task***
    - [x] ***Update a task***
    - [x] ***Soft-delete a task***
    - [ ] *Refactor db queries to module*
- [ ] *Conclusion*
  - [ ] *Next steps*
