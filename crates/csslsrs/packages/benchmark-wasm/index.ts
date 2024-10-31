import colorBenchmark from "./benchmarks/colors.js";
import foldingRangesBenchmark from "./benchmarks/folding_ranges.js";

export const benchmarks = [
	colorBenchmark,
	foldingRangesBenchmark,
	// Add more benchmarks here
];

console.info("\u001b[1mRunning benchmarks...\u001b[0m");

for (const benchmark of benchmarks) {
	await benchmark.run();
	console.info(`\u001b[1m${benchmark.name}\u001b[0m`);
	console.table(benchmark.table());
}
