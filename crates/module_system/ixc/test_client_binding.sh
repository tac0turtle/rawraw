#!/usr/bin/env bash

IXC_CONFIG=$(pwd)/examples/client_binding.toml cargo run --example client_binding "2" "39" "abcdef1234567890"