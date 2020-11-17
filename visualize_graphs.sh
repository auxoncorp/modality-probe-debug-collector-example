#!/usr/bin/env bash

set -e

modality-probe \
    visualize \
    cyclic \
    --component-path example-component \
    --report trace_log.jsonl \
    > cyclic_graph.dot

modality-probe \
    visualize \
    acyclic \
    --component-path example-component \
    --report trace_log.jsonl \
    > acyclic_graph.dot

exit 0
