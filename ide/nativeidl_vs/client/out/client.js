"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.createClient = void 0;
const vscode_languageclient_1 = require("vscode-languageclient");
const vscode_1 = require("vscode");
function createClient(serverPath) {
    const run = { command: serverPath, };
    const serverOptions = {
        run,
        debug: run,
    };
    let clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'idl' }],
        synchronize: {
            fileEvents: vscode_1.workspace.createFileSystemWatcher('**/.idl')
        }
    };
    const client = new vscode_languageclient_1.LanguageClient('idl_language_server', 'Native Idl Language Server', serverOptions, clientOptions);
    return client;
}
exports.createClient = createClient;
//# sourceMappingURL=client.js.map