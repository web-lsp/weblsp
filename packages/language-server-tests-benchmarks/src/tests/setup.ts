import { startLanguageServer } from "../server";

declare global {
	var languageServer: import("../server").LanguageServerHandle;
}

if (!globalThis.languageServer) {
	globalThis.languageServer = await startLanguageServer();
}
