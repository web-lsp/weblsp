import { afterAll, bench, describe } from "vitest";
import { startLanguageServer } from "../../server";
import { fileURLToPath } from "url";

const filePath = fileURLToPath(
	new URL("../../../fixture/hover_benchmark.css", import.meta.url)
);

const weblsp = await startLanguageServer(undefined, "weblsp");
const weblspUri = (await weblsp.openTextDocument(filePath, "css")).uri;

const vscodeLsp = await startLanguageServer(undefined, "vscode-css");
const vscodeLspUri = (await vscodeLsp.openTextDocument(filePath, "css")).uri;

describe("Hover", async () => {
	bench("weblsp - Hover", async () => {
		await weblsp.sendHoverRequest(weblspUri, {
			line: 1,
			character: 6,
		});
	});

	if (!process.env.CODSPEED) {
		bench("vscode-css-languageserver - Hover", async () => {
			await vscodeLsp.sendHoverRequest(vscodeLspUri, {
				line: 1,
				character: 6,
			});
		});
	}

	afterAll(async () => {
		await weblsp.shutdown();
		await vscodeLsp.shutdown();
		await weblsp.exit();
		await vscodeLsp.exit();
	});
});
