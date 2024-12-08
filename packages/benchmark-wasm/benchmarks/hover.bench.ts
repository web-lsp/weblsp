import { LanguageService } from "../../csslsrs/dist/index.js";
import { getCSSLanguageService } from "vscode-css-languageservice";
import { TextDocument } from "vscode-languageserver-textdocument";
import { bench, describe } from "vitest";

describe("Hover", async () => {
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

	const textDocument = TextDocument.create(
		"file:///test.css",
		"css",
		0,
		content
	);
	const ls = new LanguageService({
		include_base_css_custom_data: true,
	});
	await ls.upsert_document(textDocument);

	bench("CSSLSRS(WASM) - Hover", async () => {
		await ls.get_hover(textDocument, { line: 4, character: 3 });
	});
	if (!process.env.CODSPEED) {
		bench("vscode-css-languageservice - Hover", () => {
			const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
			vscodeLanguageService.doHover(
				textDocument,
				{ line: 4, character: 3 },
				stylesheet
			);
		});
	}
});
