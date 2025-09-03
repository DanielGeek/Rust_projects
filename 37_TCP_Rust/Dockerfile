FROM rust:latest

# Install useful dependencies for tun/tap
RUN apt-get update && apt-get install -y \
    iproute2 \
    iputils-ping \
    net-tools \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all project files including run.sh
COPY . .

# Make run.sh executable
RUN chmod +x run.sh

# Optional: script ready to run as entrypoint
ENTRYPOINT ["./run.sh"]
