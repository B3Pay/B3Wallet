#!/bin/bash
if [[ $* == *--enable-bitcoin* ]]; then
    echo "Starting DFX with Bitcoin"
    dfx start --enable-bitcoin --clean
else
    echo "Starting DFX without Bitcoin"
    dfx start --clean
fi