/**
 * 获取浏览器的range对象
 */
export function findNativeRange() {
    let nativeRange;
    const sel = window.getSelection();
    if (sel.rangeCount > 0) {
        nativeRange = sel.getRangeAt(0);
    }
    return nativeRange;
}