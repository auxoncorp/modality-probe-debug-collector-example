#!/usr/bin/env bash

set -e

modality-probe \
    log \
    --graph \
    --component-path example-component \
    --report trace_log.jsonl

exit 0
