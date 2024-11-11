#!/bin/bash

# Build the program
cargo build-bpf || exit

# If successful, deploy to local test validator
if [ $? -eq 0 ]; then
    solana program deploy \
        target/deploy/partpay.so \
        --program-id program-keypair.json
fi