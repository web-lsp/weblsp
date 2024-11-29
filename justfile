alias b := build

default: build

default_mode := "debug"

build mode=default_mode:
	just fetch-data
	echo "Building to native target..."
	cargo build {{ if mode == "release" {"--release"} else {""} }}

build-wasm mode=default_mode:
	just fetch-data
	echo "Building to WASM target..."
	cargo build --target wasm32-unknown-unknown {{ if mode == "release" {"--release"} else {""} }} --features wasm
	wasm-bindgen ./target/wasm32-unknown-unknown/{{mode}}/csslsrs.wasm --out-dir ./packages/csslsrs/src/generated --target=experimental-nodejs-module
	wasm-opt -O4 ./packages/csslsrs/src/generated/csslsrs_bg.wasm -o ./packages/csslsrs/src/generated/csslsrs_bg.wasm
	pnpm -C ./packages/csslsrs install
	pnpm -C ./packages/csslsrs run build

fetch-data:
	echo "Fetching CSS properties data..."
	mkdir -p ./crates/csslsrs/data
	curl -L -o ./crates/csslsrs/data/css-schema.json https://raw.githubusercontent.com/microsoft/vscode-custom-data/main/web-data/css/css-schema.json

test:
	echo "Running Rust tests..."
	cargo test
	echo "Running JS tests..."
	pnpm -C ./packages/csslsrs run test

benchmark:
	echo "Running Native benchmarks..."
	cargo bench
	echo "Running WASM benchmarks..."
	just build-wasm release
	pnpm -C ./packages/benchmark-wasm run benchmark
