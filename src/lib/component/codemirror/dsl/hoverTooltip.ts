import {hoverTooltip} from "@codemirror/view"
import {snippets} from "./snippets";

const funcNames = snippets.filter((s)=>s.type === 'function').map(s=>s.label);

/**
 * dsl函数 tooltips
 */
export const dslHover = hoverTooltip((view, pos, side) => {
    let {from, to, text} = view.state.doc.lineAt(pos)
    let start = pos, end = pos
    while (start > from && /\w/.test(text[start - from - 1])) start--
    while (end < to && /\w/.test(text[end - from])) end++

    const funcName = text.slice(start - from, end - from);

    if (start == pos && side < 0 || end == pos && side > 0 || !funcNames.includes(funcName))
        return null
    return {
        pos: start,
        end,
        above: true,
        create(_view) {
            let dom = document.createElement("div")
            dom.classList.add('p-1');
            dom.textContent = funcName;
            return {dom}
        }
    }
})