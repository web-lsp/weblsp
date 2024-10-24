alias b := build

default: build

mode := "debug"

build:
		echo "Building..."
		cargo build {{ if mode == "release" {"--release"} else {""} }}
		just _create_wasm


_create_wasm:
		echo "Creating wasm..."
		wasm-bindgen ./target/wasm32-unknown-unknown/{{mode}}/csslsrs.wasm --out-dir ./packages/csslsrs/src/wasm
