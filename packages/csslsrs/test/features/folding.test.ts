import { describe, it } from "mocha";
import { expect } from "chai";
import { get_folding_ranges } from "../../dist/index.js";
import { TextDocument } from "vscode-languageserver-textdocument";
import type { FoldingRange } from "vscode-languageserver-types";

describe("Folding", () => {
  it("Can return folding ranges", async () => {
    const myDocument = TextDocument.create("file:///test.css", "css", 0, "body {\n    margin: 0;\n    padding: 0;\n}\n");
    const foldingRanges = await get_folding_ranges(myDocument);

    expect(foldingRanges).to.deep.equal([
      {
        endLine: 3,
        startLine: 0,
      },
    ] satisfies FoldingRange[]);
  });
});
