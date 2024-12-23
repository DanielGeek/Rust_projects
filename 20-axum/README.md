# Rust Axum projects

Axum is a backend API framework for Rust. It's written by the same team that makes Tokio.rs and therefore is 100% compatible with Tokio. It's meant to be easy to get started with but powerful enough to run your project on.

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
- [ ] *Hello world*
  - [ ] *Spin up a hello world server*
    - [ ] ***Installing Rust (cancelled)***
    - [ ] *Using the course repo*
    - [ ] ***Setting up VS Code***
    - [ ] ***Create hello world Axum server***
    - [ ] ***Auto restart the server***
    - [ ] ***Open the documentation***
- [ ] Routing
  - [ ] ***Create a router to handle http methods***
    - [ ] ***handling HTTP methods***
  - [ ] ***Create an extractor to get a string from the body***
    - [ ] ***Extracting string from body***
  - [ ] ***Create an extractor to get JSON from the body***
    - [ ] ***Receiving JSON in a Post***
  - [ ] ***Create an extractor to get a path variable from a request***
    - [ ] ***Handling path variables***
  - [ ] ***Create an extractor to get query parameters***
    - [ ] ***Handling query params***
  - [ ] ***Create an extractor to get the headers***
    - [ ] ***Extracting the User Agent Header***
    - [ ] ***Extracting a Custom Header***
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