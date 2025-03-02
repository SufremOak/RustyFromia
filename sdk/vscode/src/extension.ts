import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    const objectiveRProvider = vscode.languages.registerDocumentSemanticTokensProvider(
        { language: 'objective-r' },
        {
            provideDocumentSemanticTokens(document: vscode.TextDocument): vscode.SemanticTokens | Thenable<vscode.SemanticTokens> {
                // Logic for semantic tokens can be implemented here
                return new vscode.SemanticTokens(new Uint32Array([]), new Uint32Array([]));
            }
        },
        legend
    );

    context.subscriptions.push(objectiveRProvider);
}

export function deactivate() {}