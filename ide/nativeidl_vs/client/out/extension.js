"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const client_1 = require("./client");
let client;
async function activate(context) {
    client = client_1.createClient("native_idl_server");
    context.subscriptions.push(client.start());
    await client.onReady();
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