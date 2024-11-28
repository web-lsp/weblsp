import * as path from "path"
import * as vscode from "vscode"
import * as lsp from "vscode-languageclient/node"

let client: lsp.LanguageClient

/**
 * Turn on WEBlsp's vscode extension 🚀
 */
export async function activate(context: vscode.ExtensionContext) {
  const serverExecutable = getServerExecutablePath(context)

  const serverOptions: lsp.ServerOptions = {
    command: serverExecutable,
    args: [],
    options: {
      // vscode.workspace.rootPath is deprecated, so we'll just run on the first workspace folder
      cwd:
        (vscode.workspace.workspaceFolders &&
          vscode.workspace.workspaceFolders[0].uri.fsPath) ||
        process.cwd(),
    },
  }

  const clientOptions: lsp.LanguageClientOptions = {
    // TODO: We should add the support of HTML later
    documentSelector: [{ scheme: "file", language: "css" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.css"),
    },
  }

  client = new lsp.LanguageClient(
    "cssLanguageServer",
    "CSS Language Server",
    serverOptions,
    clientOptions
  )

  await client.start()
}

/**
 * Cut off WEBlsp's vscode extension 😢
 */
export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined
  }
  return client.stop()
}

/**
 * Get the Rust WEBlsp binary path from the configuration.
 */
function getServerExecutablePath(context: vscode.ExtensionContext): string {
  const config = vscode.workspace.getConfiguration("cssLanguageServer")
  let serverPath = config.get<string>("path")

  if (serverPath) {
    if (!path.isAbsolute(serverPath)) {
      if (
        vscode.workspace.workspaceFolders &&
        vscode.workspace.workspaceFolders.length > 0
      ) {
        const rootPath = vscode.workspace.workspaceFolders[0].uri.fsPath
        serverPath = path.join(rootPath, serverPath)
      }
    }
  } else {
    serverPath = vscode.Uri.joinPath(
      context.extensionUri,
      "../../target/debug/weblsp"
    ).fsPath
  }

  return serverPath
}
