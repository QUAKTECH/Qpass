#!/bin/bash

echo "Building project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

if [ ! -f target/release/qpass ]; then
    echo "Error: target/release/qpass not found. Exiting."
    exit 1
fi

echo "Copying qpass to /usr/bin..."
sudo cp target/release/qpass /usr/bin/
if [ $? -ne 0 ]; then
    echo "Failed to copy qpass to /usr/bin/. Exiting."
    exit 1
fi

echo "qpass has been installed.🔥"   