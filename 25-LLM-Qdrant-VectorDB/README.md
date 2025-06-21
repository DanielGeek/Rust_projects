# Semantic Search with Vector DBs: LLMs + Qdrant DB in Rust

## Prerequisites

- Rust
- Qdrant
- LLM

## Links

- [Qdrant](https://qdrant.tech/)
- [LLM](https://ollama.com/library)

## Installation

```bash
# Clone the repository
$ git clone https://github.com/your-username/LLMQdrantVectorDB.git

# Change directory
$ cd LLMQdrantVectorDB

# Build the project
$ cargo build

# Run the project
$ cargo run
```

## Helpful Commands

```bash
# Check the version of Rust
$ rustc --version

# Check the version of Qdrant
$ qdrant --version

# Check Ollama LLMs models
$ ollama ls

# Check the running LLMs models
$ ollama ps

# Run LLM
$ ollama run llama3.2:1b

# Get embedding
curl http://localhost:11434/api/embed -d  '{"model": "llama3.2:1b", "input": "Why is the sky blue?"}'

# Run Qdrant in a docker container
docker run -p 6333:6333 -p 6334:6334 \
    -e QDRANT__SERVICE__GRPC_PORT="6334" \
    qdrant/qdrant

# Check the running containers
docker ps

# Stop the running containers
docker stop $(docker ps -q)

# Remove the running containers
docker rm $(docker ps -a -q)

# Add dependencies
cargo add tokio --features tokio/full reqwest --features reqwest/json serde --features serde/derive serde_json qdrant-client
```
