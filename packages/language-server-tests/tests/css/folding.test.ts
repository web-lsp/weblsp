import { describe, expect, it } from "vitest";

describe("CSS - Folding Ranges", () => {
	it("should return something for a folding ranges request", async () => {
		const doc = await languageServer.openFakeDocument(
			`h1 { color: red; }`,
			"css"
		);

		const foldingRanges = await languageServer.sendFoldingRangesRequest(
			doc.uri
		);

		expect(foldingRanges).toBeDefined();
	});
});
