import { afterAll, bench, describe } from "vitest";
import { startLanguageServer } from "../../server";
import { fileURLToPath } from "url";

const filePath = fileURLToPath(
	new URL("../../../fixture/folding_benchmark.css", import.meta.url)
);

const weblsp = await startLanguageServer(undefined, "weblsp");
const weblspUri = (await weblsp.openTextDocument(filePath, "css")).uri;

const vscodeLsp = await startLanguageServer(undefined, "vscode-css");
const vscodeLspUri = (await vscodeLsp.openTextDocument(filePath, "css")).uri;

describe("Folding Ranges", async () => {
	bench("weblsp - Folding Ranges", async () => {
		await weblsp.sendFoldingRangesRequest(weblspUri);
	});

	if (!process.env.CODSPEED) {
		bench("vscode-css-languageserver - Folding Ranges", async () => {
			await vscodeLsp.sendFoldingRangesRequest(vscodeLspUri);
		});
	}

	afterAll(async () => {
		await weblsp.shutdown();
		await vscodeLsp.shutdown();
		await weblsp.exit();
		await vscodeLsp.exit();
	});
});
