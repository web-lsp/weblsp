import { describe, it } from "mocha";
import { expect } from "chai";
import { TextDocument } from "vscode-languageserver-textdocument";
import { LanguageService } from "../../dist";

describe("Colors", () => {
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
			"body {\n    color: red;\n    background-color: #fff;\n}\n"
		);

		ls.upsertDocument(document);
	});

	it("Can return document colors", () => {
		const colors = ls.getDocumentColors(document.uri);

		expect(colors).to.deep.equal([
			{
				color: {
					alpha: 1,
					blue: 0,
					green: 0,
					red: 1,
				},
				range: {
					end: {
						character: 14,
						line: 1,
					},
					start: {
						character: 11,
						line: 1,
					},
				},
			},
			{
				color: {
					alpha: 1,
					blue: 1,
					green: 1,
					red: 1,
				},
				range: {
					end: {
						character: 26,
						line: 2,
					},
					start: {
						character: 22,
						line: 2,
					},
				},
			},
		]);
	});

	it("Can return color presentations", () => {
		const colors = ls.getDocumentColors(document.uri);
		const firstcolor = colors[0];

		const colorPresentations = ls.getColorPresentations(firstcolor);

		expect(colorPresentations).not.to.be.empty;
	});
});
