import { describe, expect, it } from "vitest";
import { ServerCapabilities } from "vscode-languageserver-protocol/node";

describe("Language server initilization", () => {
	it("Can init server", async () => {
		expect(languageServer).toBeDefined();
	});

	it("Has proper capabilities", async () => {
		const capabilities: ServerCapabilities = {
			colorProvider: true,
			foldingRangeProvider: true,
			hoverProvider: true,
			textDocumentSync: 1,
		};

		expect(languageServer.initializeResult.capabilities).to.deep.equal(
			capabilities
		);
	});
});
