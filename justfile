alias b := build
alias bw := build-wasm
alias t := test
alias bm := benchmark
alias i := install

default: build

default_mode := "debug"

install:
    pnpm install

build mode=default_mode:
	just fetch-data
	echo "Building to native target..."
	cargo build {{ if mode == "release" {"--release"} else {""} }}

build-wasm mode=default_mode:
	just fetch-data
	echo "Building to WASM target..."
	cargo build --package csslsrs --target wasm32-unknown-unknown {{ if mode == "release" {"--release"} else if mode == "benchmark" {"--profile benchmark"} else {""} }} --features wasm
	wasm-bindgen ./target/wasm32-unknown-unknown/{{mode}}/csslsrs.wasm --out-dir ./packages/csslsrs/src/generated --target=experimental-nodejs-module {{ if mode == "release" { "" } else { "--keep-debug" } }}
	{{ if mode == "release" { "wasm-opt -O4 ./packages/csslsrs/src/generated/csslsrs_bg.wasm -o ./packages/csslsrs/src/generated/csslsrs_bg.wasm" } else { "" } }}
	pnpm -C ./packages/csslsrs run build

fetch-data:
	echo "Fetching CSS properties data..."
	mkdir -p ./crates/csslsrs/data
	curl -L -o ./crates/csslsrs/data/css-schema.json https://raw.githubusercontent.com/microsoft/vscode-custom-data/main/web-data/css/css-schema.json

test:
	echo "Running Rust tests..."
	cargo test
	echo "Running JS tests..."
	pnpm -C ./packages/csslsrs run test --run
	pnpm -C ./packages/language-server-tests run test --run

benchmark:
	echo "Running Native benchmarks..."
	cargo bench
	echo "Running WASM benchmarks..."
	pnpm -C ./packages/benchmark-wasm run benchmark --run
