import colorBenchmark from "./benchmarks/colors.js";
import foldingRangesBenchmark from "./benchmarks/folding_ranges.js";

const benchmarks = [
	colorBenchmark,
	foldingRangesBenchmark,
	// Add more benchmarks here
];

console.info("Running benchmarks...");

for (const benchmark of benchmarks) {
	await benchmark.run();
	console.table(benchmark.table());
}
