#!/bin/bash
# Script to run the application with the correct library path

# Set the library path to prioritize system libraries
export LD_LIBRARY_PATH="/usr/lib:$LD_LIBRARY_PATH"

# Run with library debugging
if [ -f "target/debug/projectm-rs" ]; then
    echo "Running debug version..."
    ldd target/debug/projectm-rs
    LD_DEBUG=libs LD_LIBRARY_PATH="/usr/lib" ./target/debug/projectm-rs "$@"
elif [ -f "target/release/projectm-rs" ]; then
    echo "Running release version..."
    ldd target/release/projectm-rs
    LD_DEBUG=libs LD_LIBRARY_PATH="/usr/lib" ./target/release/projectm-rs "$@"
else
    echo "Application not found. Please build it first with 'cargo build'"
    exit 1
fi