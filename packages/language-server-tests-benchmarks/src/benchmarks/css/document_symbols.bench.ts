import { afterAll, bench, describe } from "vitest";
import { startLanguageServer } from "../../server";
import { fileURLToPath } from "url";

const filePath = fileURLToPath(
	new URL("../../../fixture/document_symbols_benchmark.css", import.meta.url)
);

const weblsp = await startLanguageServer(undefined, "weblsp");
const weblspUri = (await weblsp.openTextDocument(filePath, "css")).uri;

const vscodeLsp = await startLanguageServer(undefined, "vscode-css");
const vscodeLspUri = (await vscodeLsp.openTextDocument(filePath, "css")).uri;

describe("Document Symbols", async () => {
	bench("weblsp - Document Symbols", async () => {
		await weblsp.sendDocumentSymbolRequest(weblspUri);
	});

	if (!process.env.CODSPEED) {
		bench("vscode-css-languageserver - Document Symbols", async () => {
			await vscodeLsp.sendDocumentSymbolRequest(vscodeLspUri);
		});
	}

	afterAll(async () => {
		await weblsp.shutdown();
		await vscodeLsp.shutdown();
		await weblsp.exit();
		await vscodeLsp.exit();
	});
});
