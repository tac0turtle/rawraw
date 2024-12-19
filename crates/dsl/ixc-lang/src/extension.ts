/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import { trace } from "console";
import {
    languages,
    workspace,
    EventEmitter,
    ExtensionContext,
    window,
    InlayHintsProvider,
    TextDocument,
    CancellationToken,
    Range,
    InlayHint,
    TextDocumentChangeEvent,
    ProviderResult,
    commands,
    WorkspaceEdit,
    TextEdit,
    Selection,
    Uri,
} from "vscode";

import * as vscode from 'vscode';

import {
    Disposable,
    Executable,
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

// type a = Parameters<>;

export async function activate(context: ExtensionContext) {
    console.log("Starting IXC Language Server");

    let disposable = vscode.commands.registerCommand('ixc-lang.helloWorld', () => {
        // The code you place here will be executed every time your command is executed

        // Display a message box to the user
        vscode.window.showInformationMessage('Hello World!');
    });

    context.subscriptions.push(disposable);

    const traceOutputChannel = window.createOutputChannel("IXC Language Server");
	traceOutputChannel.append("Hello world!");
    //const command = process.env.SERVER_PATH || "ixc_lang";
	const command = "/Users/arc/ixc/target/debug/ixc_lang";
    const run: Executable = {
        command,
        options: {
            env: {
                ...process.env,
                // eslint-disable-next-line @typescript-eslint/naming-convention
                RUST_LOG: "debug",
            },
        },
    };
    const serverOptions: ServerOptions = {
        run,
        debug: run,
    };
    // If the extension is launched in debug mode then the debug server options are used
    // Otherwise the run options are used
    // Options to control the language client
    let clientOptions: LanguageClientOptions = {
        // Register the server for plain text documents
        documentSelector: [{scheme: "file", language: "ixc"}],
        synchronize: {
            // Notify the server about file changes to '.clientrc files contained in the workspace
            fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
        },
        traceOutputChannel,
    };

    // Create the language client and start the client.
    client = new LanguageClient("ixc-language-server", "IXC Language Server", serverOptions, clientOptions);
    // activateInlayHints(context);
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

export function activateInlayHints(ctx: ExtensionContext) {
    const maybeUpdater = {
        hintsProvider: null as Disposable | null,
        updateHintsEventEmitter: new EventEmitter<void>(),

        async onConfigChange() {
            this.dispose();

            const event = this.updateHintsEventEmitter.event;
            // this.hintsProvider = languages.registerInlayHintsProvider(
            //   { scheme: "file", language: "nrs" },
            //   // new (class implements InlayHintsProvider {
            //   //   onDidChangeInlayHints = event;
            //   //   resolveInlayHint(hint: InlayHint, token: CancellationToken): ProviderResult<InlayHint> {
            //   //     const ret = {
            //   //       label: hint.label,
            //   //       ...hint,
            //   //     };
            //   //     return ret;
            //   //   }
            //   //   async provideInlayHints(
            //   //     document: TextDocument,
            //   //     range: Range,
            //   //     token: CancellationToken
            //   //   ): Promise<InlayHint[]> {
            //   //     const hints = (await client
            //   //       .sendRequest("custom/inlay_hint", { path: document.uri.toString() })
            //   //       .catch(err => null)) as [number, number, string][];
            //   //     if (hints == null) {
            //   //       return [];
            //   //     } else {
            //   //       return hints.map(item => {
            //   //         const [start, end, label] = item;
            //   //         let startPosition = document.positionAt(start);
            //   //         let endPosition = document.positionAt(end);
            //   //         return {
            //   //           position: endPosition,
            //   //           paddingLeft: true,
            //   //           label: [
            //   //             {
            //   //               value: `${label}`,
            //   //               // location: {
            //   //               //   uri: document.uri,
            //   //               //   range: new Range(1, 0, 1, 0)
            //   //               // }
            //   //               command: {
            //   //                 title: "hello world",
            //   //                 command: "helloworld.helloWorld",
            //   //                 arguments: [document.uri],
            //   //               },
            //   //             },
            //   //           ],
            //   //         };
            //   //       });
            //   //     }
            //   //   }
            //   // })()
            // );
        },

        onDidChangeTextDocument({contentChanges, document}: TextDocumentChangeEvent) {
            // debugger
            // this.updateHintsEventEmitter.fire();
        },

        dispose() {
            this.hintsProvider?.dispose();
            this.hintsProvider = null;
            this.updateHintsEventEmitter.dispose();
        },
    };

    workspace.onDidChangeConfiguration(maybeUpdater.onConfigChange, maybeUpdater, ctx.subscriptions);
    workspace.onDidChangeTextDocument(maybeUpdater.onDidChangeTextDocument, maybeUpdater, ctx.subscriptions);

    maybeUpdater.onConfigChange().catch(console.error);
}
  