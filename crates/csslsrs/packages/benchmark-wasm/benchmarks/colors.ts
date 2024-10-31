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

export function registerColorBenchmarks(
	bench: Bench,
	onlyCSSLSRS: boolean
): Bench {
	bench.add("CSSLSRS(WASM) - Document colors", async () => {
		await get_document_colors(textDocument);
	});

	if (onlyCSSLSRS) return bench;

	bench.add("vscode-css-languageservice - Document colors", () => {
		const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
		vscodeLanguageService.findDocumentColors(textDocument, stylesheet);
	});

	return bench;
}

export default registerColorBenchmarks(bench, false);
