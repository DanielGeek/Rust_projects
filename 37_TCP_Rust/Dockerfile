FROM rust:latest

# Instalar dependencias útiles para tun/tap
RUN apt-get update && apt-get install -y \
    iproute2 \
    iputils-ping \
    net-tools \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
