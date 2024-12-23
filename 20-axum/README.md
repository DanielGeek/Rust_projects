# Rust Axum projects

Axum is a backend API framework for Rust. It's written by the same team that makes Tokio.rs and therefore is 100% compatible with Tokio. It's meant to be easy to get started with but powerful enough to run your project on.

## Help commands
```bash
cargo watch -x run
cargo doc
cargo doc --open
cargo add serde@1.0.147
cargo add serde@1.0.147 -F derive
```


**Legend**

- [ ] Unit
  - [ ] Standard
    - [ ] Lesson
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
  - [ ] ***Apply middleware to routes***
    - [ ] ***Set CORS headers***
    - [ ] ***Using layers to share data between routes***
    - [ ] ***Creating middleware function***
      - Explore creating custom middleware again
      - During recording state that versions are very important
      - New version of Axum will change how middleware works
      - WARNING: Derive Clone on struct being added to Extensions
  - [ ] ***Return the appropriate status code and message when erroring***
    - [ ] ***Returning error status codes***
    - [ ] ***Returning success status codes***
  - [ ] ***Return JSON data***
    - [ ] ***Respond with JSON data***
  - [ ] ***Validate incoming data***
    - [ ] ***Validating JSON with Serde***
    - [ ] ***Custom Extractor with Validation***
- [ ] Data
  - [ ] ***Creating a database***
    - [ ] ***Set up a local database with Docker***
  - [ ] ***Connect a database***
    - [ ] ***Introducing SeaORM***
    - [ ] ***Connecting to the database***
    - [ ] ***SeaORM Models***
    - [ ] ***Passing Database To Handlers***
  - [ ] ***CRUD data in the database including soft deletions***
    - [ ] ***Create a row in the database***
    - [ ] ***Get one item from the database***
    - [ ] ***Get all items from the database***
    - [ ] ***Using filters***
    - [ ] ***Atomic updates***
    - [ ] ***Patch updates (maybe re-record using into_active_value?)***
    - [ ] ***Deleting data***
    - [ ] ***Soft-deleting data***
- [ ] ***Security***
  - [ ] ***Authentication***
    - [ ] ***How auth works***
      - [ ] ***creating account***
      - [ ] ***login***
      - [ ] ***guard route***
      - [ ] ***logout***
      - [ ] ***use middleware***
    - [ ] ***Make it secure***
      - [ ] ***Hashing the password***
      - [ ] ***use a JWT***
- [ ] ***Helper Utilities***
  - [ ] ***Custom errors***
- [ ] ***Devops***
  - [ ] ***Deployment***
    - to own server
    - VPC
    - Container service
    - TLS
- [ ] ***Addendums***
  - [ ] ***Updating Router lessons to Axum 0.6***
    - [ ] Update version
    - [ ] Router function returns Stateless Router
    - [ ] Verify everything is working with Thunderclient
    - [ ] Use App State instead of Extension
      - middleware_message
  - [ ] ***Updating Data lessons to Axum 0.6***
    - [ ] Update version
    - [ ] Custom JSON extractor
      - Add missing generic
      - Add state to function signature
      - Brought in Extract trait
      - Added _ to type for extract
    - [ ] Return stateless router
    - [ ] Move body consuming extractors to the bottom of function signatures
    - [ ] Custom JSON extractor again
      - Add missing 'static lifetime bound
    - [ ] Full test
    - [ ] Put DatabaseConnection in AppState
      - Update handlers to pull in State
      - Update guard middleware to use state
    - [ ] Update guard middleware to use extractors directly
- [ ] *Project*
  - [ ] *Create a server that passes all tests*
    - [ ] ***Introduce the project***
    - [ ] ***Hello world with tests***
    - [ ] ***Create user***
    - [ ] ***Handle duplicate username***
    - [ ] ***sign in***
    - [ ] ***handling bad username/password***
    - [ ] ***logging out***
    - [ ] ***testing bad tokens***
    - [ ] ***creating a task***
    - [ ] ***getting all tasks***
    - [ ] ***get one task***
    - [ ] ***Update a task***
    - [ ] ***Soft-delete a task***
    - [ ] *Refactor db queries to module*
- [ ] *Conclusion*
  - [ ] *Next steps*
