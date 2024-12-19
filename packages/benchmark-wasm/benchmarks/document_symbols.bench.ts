import { LanguageService } from "csslsrs";
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

h1.foo {
  color: rgba(0, 0, 0, 0.5);
}

h1 > span {
  color: linear-gradient(to right, red, #fff);
}
`;

const textDocument = TextDocument.create("file:///test.css", "css", 0, content);
const ls = new LanguageService({
	include_base_css_custom_data: true,
});
ls.upsertDocument(textDocument);

describe("Document Symbols", async () => {
	bench("CSSLSRS(WASM) - Document Symbols", () => {
		ls.getDocumentSymbols(textDocument.uri);
	});
	if (!process.env.CODSPEED) {
		bench("vscode-css-languageservice - Document Symbols", () => {
			const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
			vscodeLanguageService.findDocumentSymbols2(
				textDocument,
				stylesheet
			);
		});
	}
});
