import colorBenchmark from "./benchmarks/colors.js";

const benchmarks = [
	colorBenchmark,
	// Add more benchmarks here
];

console.info("Running benchmarks...");

for (const benchmark of benchmarks) {
	await benchmark.run();
	console.table(benchmark.table());
}
