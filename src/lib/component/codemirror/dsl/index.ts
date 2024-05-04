import {parser} from "@lezer/rust"
import {continuedIndent, indentNodeProp, foldNodeProp, foldInside, LRLanguage, LanguageSupport} from "@codemirror/language"
import {dontComplete, localCompletionSource} from "./complete";
import {completeFromList, ifNotIn} from "@codemirror/autocomplete";
import {snippets} from "./snippets";

export * from "./hoverTooltip";

/// A syntax provider based on the [Lezer Rust
/// parser](https://github.com/lezer-parser/rust), extended with
/// highlighting and indentation information.
export const dslLanguage = LRLanguage.define({
  name: "dsl",
  parser: parser.configure({
    props: [
      indentNodeProp.add({
        IfExpression: continuedIndent({except: /^\s*({|else\b)/}),
        "String BlockComment": () => null,
        "AttributeItem": cx => cx.continue(),
        "Statement MatchArm": continuedIndent()
      }),
      foldNodeProp.add(type => {
        if (/(Block|edTokens|List)$/.test(type.name)) return foldInside
        if (type.name == "BlockComment") return tree => ({from: tree.from + 2, to: tree.to - 2})
        return undefined
      })
    ]
  }),
  languageData: {
    commentTokens: {line: "//", block: {open: "/*", close: "*/"}},
    indentOnInput: /^\s*(?:\{|\})$/,
    closeBrackets: {stringPrefixes: ["b", "r", "br"]}
  }
})

let kwCompletion = (name: string) => ({label: name, type: "keyword"});
const keywords = "break case const continue false finally in let new return static super switch this throw true yield".split(" ").map(kwCompletion)

/// dsl language support
export function dsl() {

  let completions = snippets.concat(keywords);
  return new LanguageSupport(dslLanguage,[
    dslLanguage.data.of({
        autocomplete: ifNotIn(dontComplete, completeFromList(completions))
    }),
    dslLanguage.data.of({
        autocomplete: localCompletionSource
    })
  ])
}
