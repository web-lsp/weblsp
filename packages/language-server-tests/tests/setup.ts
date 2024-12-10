import { startLanguageServer } from "./server";

declare global {
	var languageServer: Awaited<import("./server").LanguageServerHandle>;
}

if (!globalThis.languageServer) {
	globalThis.languageServer = await startLanguageServer();
}
