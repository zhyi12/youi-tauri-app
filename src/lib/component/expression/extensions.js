
import {MatchDecorator,Decoration,WidgetType,ViewPlugin,EditorView} from '@codemirror/view';

export const createColPlaceholder = (render)=>{

    ///\[\[(\w+)\]\]/g,
    const placeholderMatcher = new MatchDecorator({
        regexp: /col\("(\w+)"\)/g,
        decoration: match => Decoration.replace({
            widget: new PlaceholderWidget(match[1],render),
        })
    });

    return ViewPlugin.fromClass(class {

        constructor(view) {
            this.placeholders = placeholderMatcher.createDeco(view);
        }
        update(update) {
            this.placeholders = placeholderMatcher.updateDeco(update, this.placeholders);
        }
    }, {
        decorations: instance => instance.placeholders,
        provide: plugin => EditorView.atomicRanges.of(view => {
            return _optionalChain([view, 'access', _ => _.plugin, 'call', _2 => _2(plugin), 'optionalAccess', _3 => _3.placeholders]) || Decoration.none
        })
    })
};

class PlaceholderWidget extends WidgetType {
    constructor( name,render) {
        super();
        this.name = name;
        this.render = render;
    }
    eq(other) {
        return this.name == other.name
    }
    toDOM() {
        let elt = document.createElement("span");
        elt.style.cssText = `
              color:green;
              border-radius: 4px;
              padding: 0 1px 0 0;`;

        elt.textContent = `col("${this.render?this.render(this.name):this.name}")`;
        return elt
    }
    ignoreEvent() {
        return false
    }
}

function _optionalChain(ops) { let lastAccessLHS = undefined; let value = ops[0]; let i = 1; while (i < ops.length) { const op = ops[i]; const fn = ops[i + 1]; i += 2; if ((op === 'optionalAccess' || op === 'optionalCall') && value == null) { return undefined; } if (op === 'access' || op === 'optionalAccess') { lastAccessLHS = value; value = fn(value); } else if (op === 'call' || op === 'optionalCall') { value = fn((...args) => value.call(lastAccessLHS, ...args)); lastAccessLHS = undefined; } } return value; }