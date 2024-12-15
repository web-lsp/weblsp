import { afterAll, bench, describe } from "vitest";
import { startLanguageServer } from "../../server";
import { fileURLToPath } from "url";

const filePath = fileURLToPath(
	new URL("../../../fixture/colors_benchmark.css", import.meta.url)
);

const weblsp = await startLanguageServer(undefined, "weblsp");
const weblspUri = (await weblsp.openTextDocument(filePath, "css")).uri;
const weblspColors = await weblsp.sendDocumentColorRequest(weblspUri);

const vscodeLsp = await startLanguageServer(undefined, "vscode-css");
const vscodeLspUri = (await vscodeLsp.openTextDocument(filePath, "css")).uri;
const vscodeColors = await vscodeLsp.sendDocumentColorRequest(vscodeLspUri);

describe("Document Colors", async () => {
	bench("weblsp - Document Colors", async () => {
		await weblsp.sendDocumentColorRequest(weblspUri);
	});

	if (!process.env.CODSPEED) {
		bench("vscode-css-languageserver - Document Colors", async () => {
			await vscodeLsp.sendDocumentColorRequest(vscodeLspUri);
		});
	}
});

describe("Color Presentations", async () => {
	bench("weblsp - Color Presentation", async () => {
		await weblsp.sendColorPresentationRequest(
			weblspUri,
			weblspColors[0].color,
			weblspColors[0].range
		);
	});

	if (!process.env.CODSPEED) {
		bench("vscode-css-languageserver - Color Presentation", async () => {
			await vscodeLsp.sendColorPresentationRequest(
				vscodeLspUri,
				vscodeColors[0].color,
				vscodeColors[0].range
			);
		});
	}

	afterAll(async () => {
		await weblsp.shutdown();
		await vscodeLsp.shutdown();
		await weblsp.exit();
		await vscodeLsp.exit();
	});
});
