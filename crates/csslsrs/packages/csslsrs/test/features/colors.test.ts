import { describe, it } from "mocha";
import { expect } from "chai";
import { TextDocument } from "vscode-languageserver-textdocument";
import {
	get_document_colors,
	get_color_presentations,
} from "../../dist/generated/csslsrs.js";

describe("Colors", () => {
	it("Can return document colors", async () => {
		const myDocument = TextDocument.create(
			"file:///test.css",
			"css",
			0,
			"body {\n    color: red;\n    background-color: #fff;\n}\n"
		);
		const colors = await get_document_colors(myDocument);

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

	it("Can return color presentations", async () => {
		const myDocument = TextDocument.create(
			"file:///test.css",
			"css",
			0,
			"body {\n    color: red;\n    background-color: #fff;\n}\n"
		);
		const colors = await get_document_colors(myDocument);
		const color = colors[0];
		const colorPresentations = await get_color_presentations(
			color,
			color.range
		);

		expect(colorPresentations).not.to.be.empty;
	});
});
