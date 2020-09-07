import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';

import { createClient } from './client';
import { LanguageClient } from 'vscode-languageclient';
import { abort } from 'process';

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
	client = createClient("native_idl_server");
	context.subscriptions.push(client.start());
	await client.onReady();
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
