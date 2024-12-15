// Adapted from https://github.com/volarjs/volar.js/blob/bdbd555a7fed6e084a454d64ba0b98aac1d85241/packages/test-utils/index.ts to remove Volar-specific code and adapt it for a non-Node language server
// See https://github.com/volarjs/volar.js/blob/bdbd555a7fed6e084a454d64ba0b98aac1d85241/LICENSE#L1 for license information

import * as cp from "node:child_process";
import * as fs from "node:fs";
import * as _ from "vscode-languageserver-protocol/node";
import { TextDocument } from "vscode-languageserver-textdocument";
import { URI } from "vscode-uri";
import * as assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { randomBytes } from "node:crypto";

let pathToBinary: string;
if (process.env.BENCHMARK === "true" || process.env.RELEASE === "true") {
	pathToBinary = fileURLToPath(
		new URL("../../../target/release/weblsp", import.meta.url)
	);
} else {
	pathToBinary = fileURLToPath(
		new URL("../../../target/debug/weblsp", import.meta.url)
	);
}

export const fixtureDir = URI.file(
	fileURLToPath(new URL("./fixture", import.meta.url))
).toString();

export type LanguageServerHandle = Awaited<
	ReturnType<typeof startLanguageServer>
>;

export async function startLanguageServer(
	cwd?: string | undefined,
	which: "weblsp" | "vscode-css" = "weblsp"
) {
	if (which === "weblsp")
		console.info(`Starting language server at ${pathToBinary}`);

	const childProcess =
		which === "weblsp"
			? cp.spawn(pathToBinary, [], {
					env: process.env,
					cwd,
					stdio: "pipe",
			  })
			: cp.fork(
					"node_modules/vscode-langservers-extracted/bin/vscode-css-language-server",
					["--stdio", `--clientProcessId=${process.pid.toString()}`],
					{
						execArgv: ["--nolazy"],
						env: process.env,
						cwd,
						stdio: "pipe",
					}
			  );

	if (!childProcess.stdout || !childProcess.stdin) {
		throw new Error("Bad stdio configuration, should be pipe");
	}

	childProcess.stderr?.on("data", (data) => {
		if (process.env.DEBUG) {
			console.error(data.toString());
		}
	});

	const connection = _.createProtocolConnection(
		childProcess.stdout,
		childProcess.stdin
	);

	connection.listen();

	connection.onClose((e) => console.log("Closed", e));

	connection.onUnhandledNotification((e) =>
		console.log("Unhandled notification", e)
	);

	connection.onError((e) => console.log("Error:", e));

	connection.onRequest(_.InitializeRequest.method, (params) => {
		console.log("InitializeRequest", params);
	});

	connection.onNotification(_.LogMessageNotification.type, (e) => {
		if (e.type === _.MessageType.Error || e.type === _.MessageType.Warning) {
			console.error(e.message);
		} else {
			console.log(e.message);
		}
	});

	connection.onDispose(() => {
		connection.end();
	});

	connection.onRequest(_.ConfigurationRequest.type, ({ items }) => {
		return items.map((item) => {
			if (item.section) {
				return getConfiguration(item.section);
			}
		});
	});

	const documentVersions = new Map<string, number>();
	const openedDocuments = new Map<string, TextDocument>();
	const settings: any = {};

	let untitledCounter = 0;
	let running = false;

	function getConfiguration(section: string) {
		if (section in settings) {
			return settings[section];
		}
		let result: any;
		for (const settingKey in settings) {
			if (settingKey.startsWith(`${section}.`)) {
				const value = settings[settingKey];
				const props = settingKey.substring(section.length + 1).split(".");
				result ??= {};
				let current = result;
				while (props.length > 1) {
					const prop = props.shift()!;
					if (typeof current[prop] !== "object") {
						current[prop] = {};
					}
					current = current[prop];
				}
				current[props.shift()!] = value;
			}
		}
		return result;
	}

	async function initialize(
		rootUri: string | _.WorkspaceFolder[],
		initializationOptions: _._InitializeParams["initializationOptions"],
		capabilities: _.ClientCapabilities = {},
		locale?: string
	) {
		const result = await connection.sendRequest(_.InitializeRequest.type, {
			processId: childProcess.pid ?? null,
			rootUri: typeof rootUri === "string" ? rootUri : null,
			workspaceFolders: Array.isArray(rootUri) ? rootUri : null,
			initializationOptions,
			capabilities,
			locale,
		} satisfies _.InitializeParams);
		await connection.sendNotification(
			_.InitializedNotification.type,
			{} satisfies _.InitializedParams
		);
		running = true;
		return result;
	}

	const initResult = await initialize(
		fixtureDir,
		{
			// No init options
		},
		{
			textDocument: {
				definition: {
					linkSupport: true,
				},
			},
			workspace: {
				// Needed for tests that use didChangeWatchedFiles
				didChangeWatchedFiles: {},
				configuration: true,
			},
		}
	);

	// VS Code's CSS language server crashes if this is not set
	if (which === "vscode-css") {
		Object.assign(settings, { "css.lint.validProperties": [] });
		await connection.sendNotification(
			_.DidChangeConfigurationNotification.type,
			{ settings } satisfies _.DidChangeConfigurationParams
		);
	}

	return {
		process: childProcess,
		connection,
		initializeResult: initResult,
		async shutdown() {
			running = false;
			await connection.sendRequest(_.ShutdownRequest.type);
			openedDocuments.clear();
		},
		async exit() {
			await connection.sendNotification(_.ExitNotification.type);
		},
		async openTextDocument(fileName: string, languageId: string) {
			const uri = URI.file(fileName).toString();
			if (!openedDocuments.has(uri)) {
				const document = TextDocument.create(
					uri,
					languageId,
					(documentVersions.get(uri) ?? 0) + 1,
					fs.readFileSync(fileName, "utf-8")
				);
				documentVersions.set(uri, document.version);
				openedDocuments.set(uri, document);
				await connection.sendNotification(
					_.DidOpenTextDocumentNotification.type,
					{
						textDocument: {
							uri,
							languageId,
							version: document.version,
							text: document.getText(),
						},
					} satisfies _.DidOpenTextDocumentParams
				);
			}
			return openedDocuments.get(uri)!;
		},
		async openUntitledDocument(languageId: string, content: string) {
			const uri = URI.from({
				scheme: "untitled",
				path: `Untitled-${untitledCounter++}`,
			}).toString();
			const document = TextDocument.create(
				uri,
				languageId,
				(documentVersions.get(uri) ?? 0) + 1,
				content
			);
			documentVersions.set(uri, document.version);
			openedDocuments.set(uri, document);
			await connection.sendNotification(
				_.DidOpenTextDocumentNotification.type,
				{
					textDocument: {
						uri,
						languageId,
						version: document.version,
						text: document.getText(),
					},
				} satisfies _.DidOpenTextDocumentParams
			);
			return document;
		},
		async openFakeDocument(content: string, languageId: string) {
			const hash = randomBytes(10).toString("hex");
			const uri = URI.file(`does-not-exists-${hash}`).toString();
			const textDocument = await this.openInMemoryDocument(
				uri,
				languageId,
				content
			);

			return textDocument;
		},
		async openInMemoryDocument(
			uri: string,
			languageId: string,
			content: string
		) {
			const oldDocument = openedDocuments.get(uri);
			if (oldDocument) {
				await this.closeTextDocument(uri);
			}
			const document = TextDocument.create(
				uri,
				languageId,
				(documentVersions.get(uri) ?? 0) + 1,
				content
			);
			documentVersions.set(uri, document.version);
			openedDocuments.set(uri, document);
			await connection.sendNotification(
				_.DidOpenTextDocumentNotification.type,
				{
					textDocument: {
						uri,
						languageId,
						version: document.version,
						text: document.getText(),
					},
				} satisfies _.DidOpenTextDocumentParams
			);
			return document;
		},
		closeTextDocument(uri: string) {
			assert(openedDocuments.has(uri));
			openedDocuments.delete(uri);
			return connection.sendNotification(
				_.DidCloseTextDocumentNotification.type,
				{
					textDocument: { uri },
				} satisfies _.DidCloseTextDocumentParams
			);
		},
		async updateTextDocument(uri: string, edits: _.TextEdit[]) {
			let document = openedDocuments.get(uri);
			assert(document);
			const newText = TextDocument.applyEdits(document, edits);
			document = TextDocument.create(
				uri,
				document.languageId,
				document.version + 1,
				newText
			);
			documentVersions.set(uri, document.version);
			openedDocuments.set(uri, document);
			await connection.sendNotification(
				_.DidChangeTextDocumentNotification.type,
				{
					textDocument: {
						uri: document.uri,
						version: document.version,
					},
					contentChanges: [{ text: document.getText() }],
				} satisfies _.DidChangeTextDocumentParams
			);
			return document;
		},
		async updateConfiguration(newSettings: any) {
			Object.assign(settings, newSettings);
			if (running) {
				await connection.sendNotification(
					_.DidChangeConfigurationNotification.type,
					{ settings } satisfies _.DidChangeConfigurationParams
				);
			}
		},
		didChangeWatchedFiles(changes: _.FileEvent[]) {
			return connection.sendNotification(
				_.DidChangeWatchedFilesNotification.type,
				{ changes } satisfies _.DidChangeWatchedFilesParams
			);
		},
		async sendCompletionRequest(uri: string, position: _.Position) {
			const result = await connection.sendRequest(_.CompletionRequest.type, {
				textDocument: { uri },
				position,
			} satisfies _.CompletionParams);
			// @volar/language-server only returns CompletionList
			assert(!Array.isArray(result));
			return result;
		},
		sendCompletionResolveRequest(item: _.CompletionItem) {
			return connection.sendRequest(
				_.CompletionResolveRequest.type,
				item satisfies _.CompletionItem
			);
		},
		sendDocumentDiagnosticRequest(uri: string) {
			return connection.sendRequest(_.DocumentDiagnosticRequest.type, {
				textDocument: { uri },
			} satisfies _.DocumentDiagnosticParams);
		},
		sendHoverRequest(uri: string, position: _.Position) {
			return connection.sendRequest(_.HoverRequest.type, {
				textDocument: { uri },
				position,
			} satisfies _.HoverParams);
		},
		sendDocumentFormattingRequest(uri: string, options: _.FormattingOptions) {
			return connection.sendRequest(_.DocumentFormattingRequest.type, {
				textDocument: { uri },
				options,
			} satisfies _.DocumentFormattingParams);
		},
		sendDocumentRangeFormattingRequestRequest(
			uri: string,
			range: _.Range,
			options: _.FormattingOptions
		) {
			return connection.sendRequest(_.DocumentRangeFormattingRequest.type, {
				textDocument: { uri },
				range,
				options,
			} satisfies _.DocumentRangeFormattingParams);
		},
		sendRenameRequest(uri: string, position: _.Position, newName: string) {
			return connection.sendRequest(_.RenameRequest.type, {
				textDocument: { uri },
				position,
				newName,
			} satisfies _.RenameParams);
		},
		sendPrepareRenameRequest(uri: string, position: _.Position) {
			return connection.sendRequest(_.PrepareRenameRequest.type, {
				textDocument: { uri },
				position,
			} satisfies _.PrepareRenameParams);
		},
		sendFoldingRangesRequest(uri: string) {
			return connection.sendRequest(_.FoldingRangeRequest.type, {
				textDocument: { uri },
			} satisfies _.FoldingRangeParams);
		},
		sendDocumentSymbolRequest(uri: string) {
			return connection.sendRequest(_.DocumentSymbolRequest.type, {
				textDocument: { uri },
			} satisfies _.DocumentSymbolParams);
		},
		sendDocumentColorRequest(uri: string) {
			return connection.sendRequest(_.DocumentColorRequest.type, {
				textDocument: { uri },
			} satisfies _.DocumentColorParams);
		},
		sendDefinitionRequest(uri: string, position: _.Position) {
			return connection.sendRequest(_.DefinitionRequest.type, {
				textDocument: { uri },
				position,
			} satisfies _.DefinitionParams);
		},
		sendTypeDefinitionRequest(uri: string, position: _.Position) {
			return connection.sendRequest(_.TypeDefinitionRequest.type, {
				textDocument: { uri },
				position,
			} satisfies _.TypeDefinitionParams);
		},
		sendReferencesRequest(
			uri: string,
			position: _.Position,
			context: _.ReferenceContext
		) {
			return connection.sendRequest(_.ReferencesRequest.type, {
				textDocument: { uri },
				position,
				context,
			} satisfies _.ReferenceParams);
		},
		sendSignatureHelpRequest(uri: string, position: _.Position) {
			return connection.sendRequest(_.SignatureHelpRequest.type, {
				textDocument: { uri },
				position,
			} satisfies _.SignatureHelpParams);
		},
		sendSelectionRangesRequest(uri: string, positions: _.Position[]) {
			return connection.sendRequest(_.SelectionRangeRequest.type, {
				textDocument: { uri },
				positions,
			} satisfies _.SelectionRangeParams);
		},
		sendCodeActionsRequest(
			uri: string,
			range: _.Range,
			context: _.CodeActionContext
		) {
			return connection.sendRequest(_.CodeActionRequest.type, {
				textDocument: { uri },
				range,
				context,
			} satisfies _.CodeActionParams);
		},
		sendCodeActionResolveRequest(codeAction: _.CodeAction) {
			return connection.sendRequest(
				_.CodeActionResolveRequest.type,
				codeAction satisfies _.CodeAction
			);
		},
		sendExecuteCommandRequest(command: string, args?: any[]) {
			return connection.sendRequest(_.ExecuteCommandRequest.type, {
				command,
				arguments: args,
			} satisfies _.ExecuteCommandParams);
		},
		sendSemanticTokensRequest(uri: string) {
			return connection.sendRequest(_.SemanticTokensRequest.type, {
				textDocument: { uri },
			} satisfies _.SemanticTokensParams);
		},
		sendSemanticTokensRangeRequest(uri: string, range: _.Range) {
			return connection.sendRequest(_.SemanticTokensRangeRequest.type, {
				textDocument: { uri },
				range,
			} satisfies _.SemanticTokensRangeParams);
		},
		sendColorPresentationRequest(uri: string, color: _.Color, range: _.Range) {
			return connection.sendRequest(_.ColorPresentationRequest.type, {
				textDocument: { uri },
				color,
				range,
			} satisfies _.ColorPresentationParams);
		},
		sendDocumentLinkRequest(uri: string) {
			return connection.sendRequest(_.DocumentLinkRequest.type, {
				textDocument: { uri },
			} satisfies _.DocumentLinkParams);
		},
		sendDocumentLinkResolveRequest(link: _.DocumentLink) {
			return connection.sendRequest(
				_.DocumentLinkResolveRequest.type,
				link satisfies _.DocumentLink
			);
		},
		sendInlayHintRequest(uri: string, range: _.Range) {
			return connection.sendRequest(_.InlayHintRequest.type, {
				textDocument: { uri },
				range,
			} satisfies _.InlayHintParams);
		},
		sendInlayHintResolveRequest(hint: _.InlayHint) {
			return connection.sendRequest(
				_.InlayHintResolveRequest.type,
				hint satisfies _.InlayHint
			);
		},
	};
}
