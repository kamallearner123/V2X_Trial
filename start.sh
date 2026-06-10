#!/bin/bash

# Build everything
echo "Building C ASN.1 Library..."
cd asn1_lib
make
cd ..

echo "Building Network Layer..."
cd network_layer
make
cd ..

echo "Building Security Service..."
cd security_service
cargo build
cd ..

echo "Building CAN Interface..."
cd can_interface
cargo build
cd ..

echo "Building V2X Stack..."
cd v2x_stack
cargo build
cd ..

# Export LD_LIBRARY_PATH so v2x_stack can find libasn1.so
export LD_LIBRARY_PATH="$(pwd)/asn1_lib:$LD_LIBRARY_PATH"

# Run processes
echo "Starting Security Service..."
./security_service/target/debug/security_service &
SEC_PID=$!
sleep 1

echo "Starting V2X Stack..."
./v2x_stack/target/debug/v2x_stack &
V2X_PID=$!
sleep 1

echo "Starting CAN Interface..."
./can_interface/target/debug/can_interface &
CAN_PID=$!
sleep 1

echo "Starting Network Layer..."
./network_layer/network_dummy &
NET_PID=$!

echo "All processes started. Press Ctrl+C to stop..."

cleanup() {
    echo "Stopping all processes..."
    kill $SEC_PID $V2X_PID $CAN_PID $NET_PID 2>/dev/null
    exit 0
}

trap cleanup SIGINT SIGTERM

# Wait indefinitely
wait
