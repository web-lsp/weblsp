import { get_color_presentations, get_document_colors } from "csslsrs";
import { getCSSLanguageService } from "vscode-css-languageservice";
import { TextDocument } from "vscode-languageserver-textdocument";
import { bench, describe } from "vitest";

const vscodeLanguageService = getCSSLanguageService();
const content = `
body {
	background-color: #fff;
}

a {
	color: red;
}

h1 {
	color: rgba(0, 0, 0, 0.5);
}

h2 {
	color: linear-gradient(to right, red, #fff);
}
`;

const textDocument = TextDocument.create("file:///test.css", "css", 0, content);
const color = (await get_document_colors(textDocument))[0];

describe("Document colors", async () => {
	bench("CSSLSRS(WASM) - Document colors", async () => {
		await get_document_colors(textDocument);
	});
	if (!process.env.CODSPEED) {
		bench("vscode-css-languageservice - Document colors", () => {
			const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
			vscodeLanguageService.findDocumentColors(textDocument, stylesheet);
		});
	}
});

describe("Color Presentations", async () => {
	bench("CSSLSRS(WASM) - Color Presentations", () => {
		get_color_presentations(color, color.range);
	});

	if (!process.env.CODSPEED) {
		bench("vscode-css-languageservice - Color Presentations", () => {
			const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
			vscodeLanguageService.getColorPresentations(
				textDocument,
				stylesheet,
				color.color,
				color.range
			);
		});
	}
});
