
const parser = new DOMParser();

/**
 *
 * @param svg
 */
export function svgToData(name:string,svg:string) {

    const doc = parser.parseFromString(svg,"text/xml");

    if(doc){
        const svgRoot  = doc.documentElement;
        if(svgRoot.nodeName === 'svg'){
            //
            const viewBox = svgRoot.getAttribute("viewBox");
            const boxes = viewBox.split(' ');
            const iconData = {width:parseInt(boxes[2]),height:parseInt(boxes[3]),paths:[]};
            svgRoot.childNodes.forEach(node=>{
                if(node.nodeName === 'path'){
                    iconData.paths.push({
                        fill:node.getAttribute('fill')||'',
                        d:node.getAttribute('d'),
                    });
                }
            })
            return {[name]:iconData};
        }
    }
}