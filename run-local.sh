#!/bin/bash
set -e

# build frontend assets and put them in a place the Rocket server
# expects


echo "building ui"
pushd site-ui
cargo web build --target=wasm32-unknown-unknown
popd
echo "ui build complete"

cp site-ui/target/wasm32-unknown-unknown/release/ui.js site-server/static/ui.js
cp site-ui/target/wasm32-unknown-unknown/release/ui.wasm site-server/static/ui.wasm
cp site-ui/static/styles.css site-server/static/styles.css

(
  echo "running server"
  cd site-server
  cargo run --release
)