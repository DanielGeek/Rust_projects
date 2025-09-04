#!/bin/bash
cargo b --release
setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/TCP_Rust
$CARGO_TARGET_DIR/release/TCP_Rust &
pid=$!
ip addr add 192.168.0.1/24 dev tun0
ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid
