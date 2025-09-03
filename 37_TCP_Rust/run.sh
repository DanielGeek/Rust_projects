#!/bin/bash
set -e

# TUN Configuration
TUN_IF="tun0"
IP_ADDR="192.168.0.1/24"

# Create tun0 if it doesn't exist
if ! ip link show $TUN_IF >/dev/null 2>&1; then
    echo "Creating interface $TUN_IF..."
    ip tuntap add dev $TUN_IF mode tun
fi

# Assign IP and activate interface
echo "Configuring IP $IP_ADDR in $TUN_IF..."
ip addr add $IP_ADDR dev $TUN_IF || echo "IP already assigned"
ip link set up dev $TUN_IF

# Run Rust binary
echo "Running TCP_Rust..."
cargo run --release
