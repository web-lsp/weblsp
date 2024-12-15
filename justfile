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
	echo "Building to native target..."
	cargo build {{ if mode == "release" {"--release"} else {""} }}

build-wasm mode=default_mode:
	echo "Building to WASM target..."
	cargo build --package csslsrs --target wasm32-unknown-unknown {{ if mode == "release" {"--release"} else if mode == "benchmark" {"--profile benchmark"} else {""} }} --features wasm
	wasm-bindgen ./target/wasm32-unknown-unknown/{{mode}}/csslsrs.wasm --out-dir ./packages/csslsrs/src/generated --target=experimental-nodejs-module {{ if mode == "release" { "" } else { "--keep-debug" } }}
	{{ if mode == "release" { "wasm-opt -O4 ./packages/csslsrs/src/generated/csslsrs_bg.wasm -o ./packages/csslsrs/src/generated/csslsrs_bg.wasm" } else { "" } }}
	pnpm -C ./packages/csslsrs run build

test:
	echo "Running Rust tests..."
	cargo test
	echo "Running JS tests..."
	pnpm -r run test --run

benchmark:
	echo "Running Native benchmarks..."
	cargo bench
	echo "Running WASM benchmarks..."
	pnpm -C ./packages/benchmark-wasm run benchmark --run
	echo "Running Language Server benchmarks..."
	pnpm -C ./packages/language-server-tests-benchmarks run benchmark --run
