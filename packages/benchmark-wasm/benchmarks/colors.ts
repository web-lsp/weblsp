import { Bench } from "tinybench";
import { get_color_presentations, get_document_colors } from "csslsrs";
import { getCSSLanguageService, Range } from "vscode-css-languageservice";
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
const color = (await get_document_colors(textDocument))[0];

export function registerColorBenchmarks(
	bench: Bench,
	onlyCSSLSRS: boolean
): Bench {
	bench
		.add("CSSLSRS(WASM) - Document colors", async () => {
			await get_document_colors(textDocument);
		})
		.add("CSSLSRS(WASM) - Color Presentations", async () => {
			get_color_presentations(color, color.range);
		});

	if (onlyCSSLSRS) return bench;

	bench
		.add("vscode-css-languageservice - Document colors", () => {
			const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
			vscodeLanguageService.findDocumentColors(textDocument, stylesheet);
		})
		.add("vscode-css-languageservice - Color Presentations", () => {
			const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
			vscodeLanguageService.getColorPresentations(
				textDocument,
				stylesheet,
				color.color,
				color.range
			);
		});

	return bench;
}

export default registerColorBenchmarks(bench, false);
