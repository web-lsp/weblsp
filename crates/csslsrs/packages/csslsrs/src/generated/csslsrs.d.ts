/* tslint:disable */
/* eslint-disable */
export async function get_document_colors(source: import("vscode-languageserver-textdocument").TextDocument): Promise<import("vscode-languageserver-types").ColorInformation[]>;

export async function get_color_presentations(source: import("vscode-languageserver-textdocument").TextDocument, color: import("vscode-languageserver-types").ColorInformation, range: import("vscode-languageserver-types").Range): Promise<import("vscode-languageserver-types").ColorPresentation[]>;


export async function get_folding_ranges(source: import("vscode-languageserver-textdocument").TextDocument): Promise<import("vscode-languageserver-types").FoldingRange[]>;


