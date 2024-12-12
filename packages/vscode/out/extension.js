"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const path = __importStar(require("path"));
const vscode = __importStar(require("vscode"));
const lsp = __importStar(require("vscode-languageclient/node"));
let client;
/**
 * Turn on WEBlsp's vscode extension 🚀
 */
async function activate(context) {
    const serverExecutable = getServerExecutablePath(context);
    const serverOptions = {
        command: serverExecutable,
        args: [],
        options: {
            // vscode.workspace.rootPath is deprecated, so we'll just run on the first workspace folder
            cwd: (vscode.workspace.workspaceFolders &&
                vscode.workspace.workspaceFolders[0].uri.fsPath) ||
                process.cwd(),
        },
    };
    const clientOptions = {
        // TODO: We should add the support of HTML later
        documentSelector: [{ scheme: "file", language: "css" }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher("**/*.css"),
        },
    };
    client = new lsp.LanguageClient("cssLanguageServer", "CSS Language Server", serverOptions, clientOptions);
    await client.start();
}
/**
 * Cut off WEBlsp's vscode extension 😢
 */
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
/**
 * Get the Rust WEBlsp binary path from the configuration.
 */
function getServerExecutablePath(context) {
    const config = vscode.workspace.getConfiguration("cssLanguageServer");
    let serverPath = config.get("path");
    if (serverPath) {
        if (!path.isAbsolute(serverPath)) {
            if (vscode.workspace.workspaceFolders &&
                vscode.workspace.workspaceFolders.length > 0) {
                const rootPath = vscode.workspace.workspaceFolders[0].uri.fsPath;
                serverPath = path.join(rootPath, serverPath);
            }
        }
    }
    else {
        serverPath = vscode.Uri.joinPath(context.extensionUri, "../../target/debug/weblsp").fsPath;
    }
    return serverPath;
}
//# sourceMappingURL=extension.js.map