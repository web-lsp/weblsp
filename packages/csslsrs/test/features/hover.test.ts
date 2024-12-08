import { describe, it } from "mocha";
import { expect } from "chai";
import { TextDocument } from "vscode-languageserver-textdocument";
import { LanguageService } from "../../../csslsrs/dist/index";

describe("Hover", () => {
	let ls: LanguageService;
	let document: TextDocument;

	before(() => {
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

		ls.upsert_document(document);
	});

	it("Can return hover", async () => {
		const hover = await ls.get_hover(document.uri, { line: 0, character: 3 });

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
