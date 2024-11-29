import { describe, it } from "mocha"
import { expect } from "chai"
import { TextDocument } from "vscode-languageserver-textdocument"
import { get_hover } from "../../dist/index.js"
import cssCustomData from "../../../../data/css-schema.json"

describe("Hover", () => {
  it("Can return hover", async () => {
    const myDocument = TextDocument.create(
      "file:///test.css",
      "css",
      0,
      `body {
  background-color: #fff;
    }`
    )
    const hover = await get_hover(
      myDocument,
      { line: 0, character: 3 },
      cssCustomData
    )

    expect(hover).to.deep.equal({
      contents: {
        kind: "markdown",
        value:
          "**body**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 0, 1)\n\n",
      },
      range: {
        start: {
          line: 0,
          character: 0,
        },
        end: {
          line: 0,
          character: 4,
        },
      },
    })
  })
})
