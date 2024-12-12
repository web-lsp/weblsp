import { describe, expect, it } from "vitest";
import { ServerCapabilities } from "vscode-languageserver-protocol/node";

describe("Language server initilization", () => {
	it("Can shutdown server", async () => {
		await languageServer.shutdown();
		await languageServer.exit();

		await new Promise((resolve) => {
			languageServer.process.on("exit", resolve);
		});

		expect(languageServer.process.exitCode).toBe(0);
	});
});
