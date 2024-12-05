alias b := build

default: build

default_mode := "debug"

build mode=default_mode:
	echo "Building to native target..."
	cargo build {{ if mode == "release" {"--release"} else {""} }}

build-wasm mode=default_mode:
	echo "Building to WASM target..."
	cargo build --target wasm32-unknown-unknown {{ if mode == "release" {"--release"} else if mode == "benchmark" {"--profile benchmark"} else {"-C opt-level=s"} }} --features wasm
	wasm-bindgen ./target/wasm32-unknown-unknown/{{mode}}/csslsrs.wasm --out-dir ./packages/csslsrs/src/generated --target=experimental-nodejs-module {{ if mode == "release" { "" } else { "--keep-debug --debug" } }}
	{{ if mode == "release" { "wasm-opt -O4 ./packages/csslsrs/src/generated/csslsrs_bg.wasm -o ./packages/csslsrs/src/generated/csslsrs_bg.wasm" } else { "" } }}
	pnpm -C ./packages/csslsrs install
	pnpm -C ./packages/csslsrs run build

fetch-data:
	echo "Fetching CSS properties data..."
	mkdir -p ./crates/codegen/res
	curl -L -o ./crates/codegen/res/css-schema.json https://raw.githubusercontent.com/microsoft/vscode-custom-data/main/web-data/css/css-schema.json

codegen:
	just fetch-data
	echo "Generating Rust code..."
	cargo run -p codegen

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
