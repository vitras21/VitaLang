import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    // Word pattern includes optional currency prefixes so filtering works for keywords and variables,
    // while word-based suggestions remain scoped by our configurationDefaults.
    vscode.languages.setLanguageConfiguration('vitalang', {
        wordPattern: /[$£]?[A-Za-z_][\w]*/
    });

    // Register for the vitalang language; trigger on common identifier starters so suggestions appear as you type.
    const provider = vscode.languages.registerCompletionItemProvider(
        { language: 'vitalang', scheme: 'file' },
        {
            provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
                const keywords = [
                    "I would love to own a plot of land in the 1800s called",           // TokenType::Define
                    "and lease it to",                                                  // TokenType::Assign
                    "sweet but stout",                                                  // TokenType::ElseIf
                    "American",                                                         // TokenType::ImportAll
                    "owners",                                                           // TokenType::EndOfAssign
                    "scammy",                                                           // TokenType::Import
                    "sweet",                                                            // TokenType::If
                    "stout",                                                            // TokenType::Else
                    "lolsie",                                                           // TokenType::For
                    "yarp'",                                                            // TokenType::While
                    "jump off the bandwagon",                                           // TokenType::Break
                    "get back to work boy",                                             // TokenType::Continue
                    "anywho",                                                           // TokenType::Yield
                    "sir, would there happen to be any extension work?",                // TokenType::Try
                    "yay, homework!",                                                   // TokenType::Catch
                    "europe ->",                                                        // TokenType::Comment
                    "asia ->",                                                          // TokenType::BlockCommentStart
                    "<- asia"                                                           // TokenType::BlockCommentEnd
                ];

                // Collect variable-like identifiers present in the document.
                const text = document.getText();
                const variableMatches =
                    text.match(/(?:^|[^A-Za-z0-9_])([$£][A-Za-z_]\w*)/g)?.map(m => m.trim().replace(/^[^$£]*/, '')) || [];

                const makeVarItem = (token: string) => {
                    const base = token.slice(1); // show without prefix
                    const item = new vscode.CompletionItem(base, vscode.CompletionItemKind.Variable);
                    // Replace only the typed prefix to avoid duplication.
                    const lineText = document.lineAt(position.line).text;
                    const linePrefix = lineText.slice(0, position.character);
                    const currencyIndex = Math.max(linePrefix.lastIndexOf('$'), linePrefix.lastIndexOf('£'));
                    let range: vscode.Range | undefined;
                    if (currencyIndex !== -1 && /^[A-Za-z0-9_]*$/.test(linePrefix.slice(currencyIndex + 1))) {
                        range = new vscode.Range(
                            new vscode.Position(position.line, currencyIndex),
                            position
                        );
                    }
                    if (!range) {
                        const prefixMatch = linePrefix.match(/[$£]?[A-Za-z_]\w*$/);
                        range = prefixMatch
                            ? new vscode.Range(
                                  new vscode.Position(position.line, position.character - prefixMatch[0].length),
                                  position
                              )
                            : document.getWordRangeAtPosition(position, /[$£]?[A-Za-z_]\w*/);
                    }
                    item.textEdit = range
                        ? new vscode.TextEdit(range, token)
                        : new vscode.TextEdit(new vscode.Range(position, position), token);
                    // Filter text includes the prefix so typing "$f" still matches.
                    item.filterText = token;
                    item.sortText = base;
                    return item;
                };

                const dollarItems = Array.from(new Set(variableMatches.filter(v => v.startsWith('$')))).map(makeVarItem);
                const poundItems = Array.from(new Set(variableMatches.filter(v => v.startsWith('£')))).map(makeVarItem);

                // Determine if the cursor is in a variable context (immediately after $/£).
                const line = document.lineAt(position.line).text;
                const charBefore = position.character > 0 ? line[position.character - 1] : '';
                if (charBefore === '$') {
                    return new vscode.CompletionList(dollarItems, false);
                }
                if (charBefore === '£') {
                    return new vscode.CompletionList(poundItems, false);
                }

                // Otherwise show keyword suggestions plus variables.
                const keywordItems = keywords.map(
                    word => new vscode.CompletionItem(word, vscode.CompletionItemKind.Keyword)
                );
                return new vscode.CompletionList([...keywordItems, ...dollarItems, ...poundItems], false);
            }
        },
        // trigger on letters and common variable prefix characters so suggestions show as you type
        ...'abcdefghijklmnopqrstuvwxyz',
        ...'ABCDEFGHIJKLMNOPQRSTUVWXYZ',
        '$',
        '£',
        '.'
    );

    context.subscriptions.push(provider);

    // Auto-trigger suggestions when typing plain identifiers so keyword completions appear without manual Ctrl+Space.
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            if (event.document.languageId !== 'vitalang') {
                return;
            }
            const change = event.contentChanges[0];
            if (!change) {
                return;
            }
            // Only fire on single-character inserts (not replacements) to avoid re-triggering after a completion accept.
            if (change.text.length === 1 && change.rangeLength === 0 && /[A-Za-z]/.test(change.text)) {
                void vscode.commands.executeCommand('editor.action.triggerSuggest');
            }
        })
    );
}
