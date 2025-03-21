"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const path = require("path");
const vscode = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    // Adjust the path to point to your compiled Rust LSP executable.
    const serverExecutable = context.asAbsolutePath(path.join('server', 'rustlite_lsp'));
    // Server options to launch the language server.
    let serverOptions = {
        command: serverExecutable,
        args: [],
        transport: node_1.TransportKind.stdio
    };
    // Client options to register for documents of language 'rustlite'.
    let clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'rustlite' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/.clientrc')
        }
    };
    // Create and start the language client.
    client = new node_1.LanguageClient('rustliteLanguageServer', 'RustLite Language Server', serverOptions, clientOptions);
    client.start();
}
exports.activate = activate;
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map
