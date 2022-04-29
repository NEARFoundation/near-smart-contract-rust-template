#!/usr/bin/env bash

WASM_PATH="$(find ./target/wasm32-unknown-unknown/release/ -maxdepth 1 -name "*.wasm")"

near dev-deploy \
  --wasmFile $WASM_PATH \
  "$@"

near call "$(<./neardev/dev-account)" new "$(node ./init-args.js)" \
  --accountId "$(<./neardev/dev-account)"
