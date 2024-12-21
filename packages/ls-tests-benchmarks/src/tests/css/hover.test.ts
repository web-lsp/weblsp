import { describe, expect, it } from "vitest";
import { Position } from "vscode-languageserver-protocol";

describe("CSS - Hover", () => {
	it("should return something for a hover request", async () => {
		const doc = await languageServer.openFakeDocument(
			`h1 { color: red; }`,
			"css"
		);

		const hover = await languageServer.sendHoverRequest(
			doc.uri,
			Position.create(0, 5)
		);

		expect(hover).toBeDefined();
	});
});
