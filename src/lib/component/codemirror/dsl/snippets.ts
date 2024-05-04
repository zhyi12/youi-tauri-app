import {snippetCompletion as snip} from "@codemirror/autocomplete"
import type {Completion} from "@codemirror/autocomplete"

/// A collection of JavaScript-related
/// [snippets](#autocomplete.snippet).
export const snippets: readonly Completion[] = [
  snip("fn ${name}(${params}) {\n\t${}\n}", {
    label: "fn",
    detail: "函数",
    type: "keyword"
  }),
  snip("for (let ${x} in ${array}) {\n\t${}\n}", {
    label: "for",
    detail: "in",
    type: "keyword"
  }),
  snip("for x in 0..10 {\n\t${}\n}", {
    label: "for",
    detail: "in range",
    type: "keyword"
  }),
  snip("for x in range(0..10,2) {\n\t${}\n}", {
    label: "for",
    detail: "in range step",
    type: "keyword"
  }),
  snip("if (${}) {\n\t${}\n}", {
    label: "if",
    detail: "block",
    type: "keyword"
  }),
  snip("if (${}) {\n\t${}\n} else {\n\t${}\n}", {
    label: "if",
    detail: "/ else block",
    type: "keyword"
  }),
  snip("${read_csv}(\"${{dataDir}}/data.csv\")",{
    label: "read_csv",
    detail: "读csv数据",
    type: "function"
  })
]


//{label: "read_csv", type: "function",apply: "read_csv(\"${dataDir}/t.csv\")"},
//             {label: "write_csv", type: "function",apply: "write_csv(\"\")"},
//             {label: "read_sql", type: "function",apply: "read_sql(\"${dbConnect}\",\"select * from \")"},
//             {label: "write_sql", type: "function",apply: "write_sql(\"${dbConnect}\")"},
//             {label: "filter", type: "function",apply: "filter(col(\"\"))"},
//             {label: "filter_by_expression", type: "function",apply: "filter_by_expression(\"\")"},
//             {label: "select", type: "function",apply: "select([col(\"\")])"},
//             {label: "with_columns", type: "function",apply: "with_columns([col(\"\")])"},
//             {label: "explode", type: "function",apply: "explode([col(\"\")])"},
//             {label: "union", type: "function",apply: "union()"},
//             {label: "left_join", type: "function",apply: "left_join(,\"\",\"\")"},
//             {label: "unique", type: "function",apply: "unique(\"\")"},
//             {label: "limit", type: "function",apply: "limit(10)"},
//             {label: "sort_by_exprs", type: "function",apply: "sort_by_exprs([col(\"\")],\"false\")"},
//             {label: "agg", type: "function",apply: "agg(\"\",[col(\"\").sum().alias(\"col_sum\")])"},
//             {label: "when", type: "function",apply: "when(col(\"\").eq(lit(\"\")).then(lit(1)).otherwise(lit(0)))"},
//             {label: "cast_str", type: "function",apply: "cast_str()"},
//             {label: "col", type: "function",apply: "col(\"\")"},
//             {label: "alias", type: "function",apply: "alias(\"\")"},
//             {label: "cols", type: "function",apply: "cols([])"},
//             {label: "sum", type: "function",apply: "sum()"},
//             {label: "count", type: "function",apply: "count()"},
//             {label: "exclude", type: "function",apply: "exclude(\"\")"},
//             {label: "str_contains", type: "function",apply: "str_contains(\"\")"},
//             {label: "str_replace", type: "function",apply: "str_replace(\"\",\"\")"},
//             {label: "str_length", type: "function",apply: "str_length(\"\",\"\")"},
//             {label: "str_split", type: "function",apply: "str_split(\"\")"},
//             {label: "str_slice", type: "function",apply: "str_slice(0,2)"},