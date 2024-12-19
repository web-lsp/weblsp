import { describe, expect, it } from "vitest";

describe("CSS - Document Symbols", () => {
	it("should return symbols for a document", async () => {
		const doc = await languageServer.openFakeDocument(
			`h1 { color: red; }`,
			"css"
		);

		const symbols = await languageServer.sendDocumentSymbolRequest(
			doc.uri
		);

		expect(symbols).toBeDefined();
	});
});
