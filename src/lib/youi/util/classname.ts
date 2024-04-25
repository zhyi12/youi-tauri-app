
function toClassName(value:string|number|any) {
    let result = '';

    if (typeof value === 'string' || typeof value === 'number') {
        result += value;
    } else if (typeof value === 'object') {
        if (Array.isArray(value)) {
            result = value.map(toClassName).filter(Boolean).join(' ');
        } else {
            for (const key in value) {
                if (value[key]) {
                    result && (result += ' ');
                    result += key;
                }
            }
        }
    }

    return result;
}

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export default function classnames(...args) {
    return args.map(toClassName).filter(Boolean).join(' ');
}