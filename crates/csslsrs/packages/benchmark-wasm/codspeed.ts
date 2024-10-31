import { withCodSpeed } from "@codspeed/tinybench-plugin";
import { benchmarks } from "./index.js";
import { Bench } from "tinybench";
import { registerColorBenchmarks } from "./benchmarks/colors.js";
import { registerFoldingRangesBenchmarks } from "./benchmarks/folding_ranges.js";

const codSpeedBenchmark = withCodSpeed(
	new Bench({ name: "Benchmark-WASM", time: 100 })
);

registerColorBenchmarks(codSpeedBenchmark, true);
registerFoldingRangesBenchmarks(codSpeedBenchmark, true);

await codSpeedBenchmark.run();
