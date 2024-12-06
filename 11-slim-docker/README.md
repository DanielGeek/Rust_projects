# Building Slim Docker Image | Rust | Step by Step Guide

## help commands
### example heavy image
`sudo docker build . -t heavy:latest`
`docker images --filter "reference=heavy:*"`

### example light image
`sudo docker build . -t light:latest`
`docker images --filter "reference=light:*"`
`docker run -d -p 3000:3000 light:latest`

#### Test it with GET request
`http://localhost:3000/`
