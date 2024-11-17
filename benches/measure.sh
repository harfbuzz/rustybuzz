#!/usr/bin/env bash
set -euo pipefail

export HARFBUZZ_SYS_NO_PKG_CONFIG=""

# Using jq here would be best, however it's not installed by default on many systems, and this isn't a critical script.
TEST_EXECUTABLE=$(
    cargo +nightly bench \
    --message-format=json \
    --no-default-features --no-run | \
        sed -n 's/^{"reason":"compiler-artifact".*"executable":"\([^"]*\)".*}$/\1/p'
)

exec perf record --call-graph dwarf -- "$TEST_EXECUTABLE" --bench --profile-time 5 "$@"
