#!/usr/bin/env bash

set -e

modality-probe-debug-collector \
    --attach stm32 \
    --elf target/thumbv7em-none-eabihf/debug/example-project \
    --reset 100ms \
    --interval 100ms \
    --output trace_log.jsonl \
    PROBE_BUFFER

exit 0
