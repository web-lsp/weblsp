# language-server-tests-benchmarks

The tests and benchmarks in this folder aims to test and benchmark the language server in a close-to-reality scenario, using the same JavaScript client powered by `vscode-jsonrpc` used in VS Code.

## Why are the benchmarks so slow?

If you're used to the numbers from the benchmarks of the language services, the numbers here might be surprisingly slow. The reason for that is that the benchmark also includes the large overhead caused by the client-server communication. Even if the language server can sometimes answer in 50-100μs, just sending and waiting for the response can take 95-99% of the time the benchmark measures, leading to times closer to 1-2ms.

Unfortunately, due to the multiple processes involved it's not possible to get accurate flamegraphs from CodSpeed for these benchmarks at the time of writing. Locally, [flamegraph](https://github.com/flamegraph-rs/flamegraph) can be used, but it requires a bit of setup, especially on non-Linux systems.
