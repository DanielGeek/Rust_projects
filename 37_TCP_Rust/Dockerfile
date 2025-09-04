FROM rust:latest

# Install useful dependencies for tun/tap
RUN apt-get update && apt-get install -y \
    iproute2 \
    iputils-ping \
    net-tools \
    libssl-dev \
    pkg-config \
    tshark \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all project files including run_server.sh and run_client.sh
COPY . .

# Make run_server.sh executable
RUN chmod +x run_server.sh

# Optional: script ready to run as entrypoint
ENTRYPOINT ["./run_server.sh"]
CMD ["./run_client.sh"]
