import { defineConfig } from "vitest/config";
import { BaseSequencer, WorkspaceSpec } from "vitest/node";

class Sequencer extends BaseSequencer {
	async sort(files: WorkspaceSpec[]): Promise<WorkspaceSpec[]> {
		const init = files.find((f) => f.moduleId.endsWith("init.test.ts"));
		const shutdown = files.find((f) => f.moduleId.endsWith("shutdown.test.ts"));

		const tests = files.filter(
			(f) =>
				!f.moduleId.endsWith("init.test.ts") &&
				!f.moduleId.endsWith("shutdown.test.ts")
		);

		return [init, ...tests, shutdown].filter(Boolean) as WorkspaceSpec[];
	}
}

export default defineConfig({
	test: {
		dir: "tests",
		watch: false,
		isolate: false,
		fileParallelism: false,
		setupFiles: ["./tests/setup.ts"],
		sequence: {
			sequencer: Sequencer,
		},
	},
});
