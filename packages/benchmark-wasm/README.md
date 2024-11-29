# benchmark-wasm

This package contains benchmarks for the WASM output of CSSLSRS, comparing it with other popular language services for CSS in JavaScript.

## Adding a new benchmark

To add a new benchmark, create a new file in the `benchmarks` directory. The file should create a new `Bench` instance from `tinybench`, a function to register the different tasks to run (typically, one from each services being tested) and default export the result of said function, taking in the `Bench` instance as an argument.

Additionally, to ensure the benchmark can be used by CodSpeed, add a parameter to only register the CSSLSRS task.

Example:

```ts
import { Bench } from "tinybench";

const bench = new Bench({ name: "Feature Name", time: 100 });

export function registerFeatureBenchmarks(bench: Bench, csslsrsOnly: boolean) {
    bench.add("CSSLSRS(WASM) - Feature Name", () => {
        // Run the CSSLSRS benchmark...
    });

    if (csslsrsOnly) {
        return;
    }

    bench.add("Other Service - Feature Name", () => {
        // Run the other service benchmark...
    });

    return bench;
}

export default registerFeatureBenchmarks(bench);
```

Then, add the benchmark to `index.ts` and `codspeed.ts`. The former simply imports the default export of the new benchmark file, while the latter imports the new benchmark and registers it to a single benchmark with the `registerFeatureBenchmarks` function, passing in `true` as the second argument.

## Running the benchmarks

To run the benchmarks, use the following command:

```sh
pnpm run benchmark
```

This will run the benchmarks and output the results to the console.
