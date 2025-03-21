const path = require('path');
const vscode = require('vscode');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');

let client;

function activate(context) {
  // Adjust the path to point to your compiled Rust LSP executable.
  const serverExecutable = context.asAbsolutePath(path.join('server', 'rustlite_lsp'));

  // Server options to launch the language server.
  const serverOptions = {
    command: serverExecutable,
    args: [],
    transport: TransportKind.stdio
  };

  // Client options to register for documents of language 'rustlite'.
  const clientOptions = {
    documentSelector: [{ scheme: 'file', language: 'rustlite' }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/.clientrc')
    }
  };

  // Create and start the language client.
  client = new LanguageClient(
    'rustliteLanguageServer',
    'RustLite Language Server',
    serverOptions,
    clientOptions
  );

  client.start();
}

function deactivate() {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

module.exports = {
  activate,
  deactivate
};
