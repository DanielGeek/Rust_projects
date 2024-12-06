FROM rust:1.70.0-alpine3.18 AS builder
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/slim-docker
COPY . .
RUN cargo build --release

FROM alpine:3.18.0
COPY --from=builder /usr/src/slim-docker/target/release/slim-docker /
CMD ["./slim-docker"]

# example heavy image
# FROM rust:latest

# WORKDIR usr/src/app

# COPY . .

# RUN cargo build --release

# CMD cargo run --quit
