import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		dir: "tests",
		// Both settings are needed so that tests can share the same language server
		isolate: false,
		fileParallelism: false,
		setupFiles: ["./tests/setup.ts"],
	},
});
