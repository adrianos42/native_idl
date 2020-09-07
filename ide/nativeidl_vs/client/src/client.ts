import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind,
	Executable
} from 'vscode-languageclient';


import { workspace, ExtensionContext } from 'vscode';

export function createClient(serverPath: string): LanguageClient {
	const run: Executable = { command: serverPath,  };

	const serverOptions: ServerOptions = {
		run,
		debug: run,
	};

	let clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: 'file', language: 'idl' }],
		synchronize: {
			fileEvents: workspace.createFileSystemWatcher('**/.idl')
		}
	};

	const client = new LanguageClient(
		'idl_language_server',
		'Native Idl Language Server',
		serverOptions,
		clientOptions
	);


	return client;
}