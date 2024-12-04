import { get_folding_ranges } from "csslsrs";
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

//#region Outer Region
h3 {
	color: hsl(120, 100%, 50%);
}
//#region Inner Region
h4 {
	color: hwb(120, 0%, 0%);
}
//#endregion
//#endregion

@media (max-width: 600px) {
	body {
		background-color: #000;
	}
}
`;

const textDocument = TextDocument.create("file:///test.css", "css", 0, content);

describe("Folding Ranges", async () => {
	bench("CSSLSRS(WASM) - Folding Ranges", async () => {
		await get_folding_ranges(textDocument);
	});
	if (!process.env.CODSPEED) {
		bench("vscode-css-languageservice - Folding Ranges", () => {
			const stylesheet = vscodeLanguageService.parseStylesheet(textDocument);
			vscodeLanguageService.getFoldingRanges(textDocument, stylesheet);
		});
	}
});
