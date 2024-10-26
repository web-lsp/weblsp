alias b := build

default: build

default_mode := "debug"

build mode=default_mode:
		echo "Building to native target..."
		cargo build {{ if mode == "release" {"--release"} else {""} }}

build-wasm mode=default_mode:
		echo "Building to WASM target..."
		cargo build --target wasm32-unknown-unknown {{ if mode == "release" {"--release"} else {""} }}
		wasm-bindgen ./target/wasm32-unknown-unknown/{{mode}}/csslsrs.wasm --out-dir ./packages/csslsrs/src/generated --target=experimental-nodejs-module
		pnpm -C ./packages/csslsrs install
		pnpm -C ./packages/csslsrs run build

test:
        echo "Running Rust tests..."
        cargo test
        echo "Running JS tests..."
        pnpm -C ./packages/csslsrs run test
