FROM rust:latest

# Install useful dependencies for tun/tap
RUN apt-get update && apt-get install -y \
    iproute2 \
    iputils-ping \
    net-tools \
    libssl-dev \
    pkg-config \
    tshark \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all project files including run_osx_version.sh
COPY . .

# Make run_osx_version.sh executable
RUN chmod +x run_osx_version.sh

# Optional: script ready to run as entrypoint
ENTRYPOINT ["./run_osx_version.sh"]
