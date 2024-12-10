import { describe, it, expect, beforeAll } from "vitest";
import { TextDocument } from "vscode-languageserver-textdocument";
import { LanguageService } from "../../../csslsrs/dist/index";

describe("Hover", () => {
	let ls: LanguageService;
	let document: TextDocument;

	beforeAll(() => {
		ls = new LanguageService({
			include_base_css_custom_data: true,
		});

		document = TextDocument.create(
			"file:///test.css",
			"css",
			0,
			`body {
  background-color: #fff;
    }`
		);

		ls.upsertDocument(document);
	});

	it("Can return hover", () => {
		const hover = ls.getHover(document.uri, { line: 0, character: 3 });

		expect(hover).to.deep.equal({
			contents: {
				kind: "markdown",
				value:
					"**body**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 0, 1)\n\n",
			},
			range: {
				start: {
					line: 0,
					character: 0,
				},
				end: {
					line: 0,
					character: 4,
				},
			},
		});
	});
});
