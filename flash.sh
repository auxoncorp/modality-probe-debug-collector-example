#!/usr/bin/env bash

set -e

openocd \
    -f openocd.cfg \
    -c init \
    -c "reset halt" \
    -c "flash write_image erase target/thumbv7em-none-eabihf/debug/example-project" \
    -c "reset run" \
    -c "shutdown"

exit 0
