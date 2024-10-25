alias b := build

default: build

mode := "debug"

build:
		echo "Building to native target..."
		cargo build {{ if mode == "release" {"--release"} else {""} }}

build-wasm:
		echo "Building to WASM target..."
		cargo build --target wasm32-unknown-unknown {{ if mode == "release" {"--release"} else {""} }} --features wasm
		wasm-bindgen ./target/wasm32-unknown-unknown/{{mode}}/csslsrs.wasm --out-dir ./packages/csslsrs/src/wasm --target=experimental-nodejs-module
		pnpm -C ./packages/csslsrs install
		pnpm -C ./packages/csslsrs run build
