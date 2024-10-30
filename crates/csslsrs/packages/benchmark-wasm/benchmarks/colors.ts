import { Bench } from "tinybench";
import { get_document_colors } from "csslsrs";
import { getCSSLanguageService } from "vscode-css-languageservice";
import { TextDocument } from "vscode-languageserver-textdocument";

const bench = new Bench({ name: "Colors", time: 100 });

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

bench
	.add("CSSLSRS - WASM", async () => {
		await get_document_colors(textDocument);
	})
	.add("vscode-css-languageservice", () => {
		const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
		vscodeLanguageService.findDocumentColors(textDocument, stylesheet);
	});

export default bench;
