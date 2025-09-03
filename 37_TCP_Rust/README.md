# This is a TCP server and client implementation in Rust

## Helpfull commands

```bash
cargo new --bin TCP_Rust
```

```bash
cargo b --release
```

## Build the image

```bash
docker-compose build
```

## Run the container

```bash
docker-compose up
```

## If you want to go in and test things manually

```bash
docker-compose run --rm tcp_rust bash
```

```bash
ifconfig | grep inet
```

```bash
ipconfig getifaddr en0
```

```bash
networksetup -getinfo Wi-Fi
```

```bash
sudo ifconfig utun0 inet 192.168.0.1 192.168.0.1 netmask 255.255.255.0 up
```
