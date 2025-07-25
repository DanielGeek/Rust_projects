# AI Meetup Talk

## Restrictions

- 20 minutes of talk
- 10 minutes qa

## Goals for the talk

By the end of this talk you will be

- inspired to build your own project with an AI Agent
- know where to start with creating your AI agent project

## Ideas

- Cooking "helper"
  - user asks for help cooking something
  - agent creates a list of steps with times for each step (in json)
  - agent watches user and reads out steps and reprimands if steps are off
  - restart from current step if user misses time
- Todo app

- Video game assistant

## Tech

- Rust
- Ollama
- Postgres

## Helpful Commands

```bash
cargo run -F log_all
```

```bash
create table tasks (id serial primary key, name text not null);
```

```bash
docker exec -ti ai_postgres psql -U postgres
```

```bash
cargo new --lib db
```

```bash
docker run -d --name ai_postgres -e POSTGRES_PASSWORD=keyboardcat -v ai_postgres:/var/lib/postgresql/data -p 9432:5432 postgres
```

```bash
cargo new --lib bb_ollama
```

```bash
cargo watch -x run
```

```bash
cargo run "create a new task to drink water"
```

```bash
ollama ls
```

```bash
ollama run llama3.2-vision:11b-instruct-fp16
```

## Research

### Todo

- Tool usage with Ollama (pass in tool list. Examples on Postman)
- Which model to use?
  - llama 3.18b
    - chat
    - tools (correct tool choice)
  - llama 3.2-vision
    - chat
    - vision
  - llama 3.2 1b
    - chat
    - tools (bad tool choice)

### game assistant

- grab a screenshot
- use api to describe image
- transcribe mic
- grab screenshot from windows pc

### cooking assistant

- grab image from camera

## Talk

- Title (download link for slides / code)
- Main
  - Problem statement
  - description of the agent
  - demo / video of the agent at work
  - high level non-technical overview of agentic structure
  - break down demo and show how specific parts are wired up
    - tools being used
      - Ollama
      - llama 3.2?
- Conclusion

  - Summary
    - why silly ai projects are amazing
  - About me / Download talk and code

  ## Demo

  ### Stories

  As a doer, I want to

  - [ ] create a task
    - [ ] I want to create the slides for the ai meetup
  - [ ] view all tasks
  - [ ] view next task
  - [ ] complete a task
  - [ ] uncomplete a task
  - [ ] delete a task
