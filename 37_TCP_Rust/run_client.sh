#!/bin/bash
set -e

TUN_IF="tun0"
IP_ADDR="192.168.0.2/24"

# Create tun0
if ! ip link show $TUN_IF >/dev/null 2>&1; then
    echo "Creating $TUN_IF..."
    ip tuntap add dev $TUN_IF mode tun
fi

# Assign IP
ip addr add $IP_ADDR dev $TUN_IF || echo "IP already assigned"
ip link set up dev $TUN_IF

echo "Client ready on $IP_ADDR, trying to connect to server..."

# Try ping
ping -c 3 -I $TUN_IF 192.168.0.1 || echo "Ping failed"

# Open TCP connection to server:80
echo "hello from client" | nc 192.168.0.1 80
