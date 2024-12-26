#!/usr/bin/env bash

IXC_CONFIG=$(pwd)/examples/client_binding.toml cargo run --example client_binding "0x39" "0xabcdef1234567890"