import { describe, it, expect } from "vitest";
import { LanguageService } from "../../dist/index.js";
import { TextDocument } from "vscode-languageserver-textdocument";
import type { FoldingRange } from "vscode-languageserver-types";

describe("Folding", () => {
	it("Can return folding ranges", () => {
		const myDocument = TextDocument.create(
			"file:///test.css",
			"css",
			0,
			"body {\n    margin: 0;\n    padding: 0;\n}\n"
		);
		// @ts-ignore
		const ls = new LanguageService({
			include_base_css_custom_data: true,
		});
		ls.upsertDocument(myDocument);

		const foldingRanges = ls.getFoldingRanges(myDocument.uri);

		expect(foldingRanges).to.deep.equal([
			{
				endLine: 3,
				startLine: 0,
			},
		] satisfies FoldingRange[]);
	});
});
