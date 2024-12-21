import { describe, expect, it } from "vitest";

describe("CSS - Colors", () => {
	it("should return something for a document colors request", async () => {
		const doc = await languageServer.openFakeDocument(
			`h1 { color: red; }`,
			"css"
		);

		const colors = await languageServer.sendDocumentColorRequest(doc.uri);

		expect(colors).toBeDefined();

		const colorPresentations =
			await languageServer.sendColorPresentationRequest(
				doc.uri,
				colors[0].color,
				colors[0].range
			);

		expect(colorPresentations).toBeDefined();
	});
});
