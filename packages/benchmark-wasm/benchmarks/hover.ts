import { Bench } from "tinybench"
import { get_hover } from "../../csslsrs/dist/generated/csslsrs"
import { getCSSLanguageService } from "vscode-css-languageservice"
import { TextDocument } from "vscode-languageserver-textdocument"
import cssCustomData from "../../../data/css-schema.json"

const bench = new Bench({ name: "Hover", time: 100 })

const vscodeLanguageService = getCSSLanguageService()
const content = `
body {
  background-color: #fff;
}

a {
  color: red;
}

h1.foo {
  color: rgba(0, 0, 0, 0.5);
}

h1 > span {
  color: linear-gradient(to right, red, #fff);
}
`

const textDocument = TextDocument.create("file:///test.css", "css", 0, content)

export function registerHoverBenchmarks(
  bench: Bench,
  onlyCSSLSRS: boolean
): Bench {
  bench.add("CSSLSRS(WASM) - Hover", async () => {
    await get_hover(textDocument, { line: 4, character: 3 }, cssCustomData)
  })

  if (onlyCSSLSRS) return bench

  bench.add("vscode-css-languageservice - Hover", () => {
    const stylesheet = vscodeLanguageService.parseStylesheet(textDocument)
    vscodeLanguageService.doHover(
      textDocument,
      { line: 4, character: 3 },
      stylesheet
    )
  })

  return bench
}

export default registerHoverBenchmarks(bench, false)
