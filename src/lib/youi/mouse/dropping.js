const DROPPING_EVENT_NAME = 'dropping';
const DROPPING_CLASS = 'dropping';

/**
 *
 * @param node
 * @param options
 */
export function dropping(node, options) {
    if(options.droppable){
        // 监听自定义事件
        node.addEventListener(DROPPING_EVENT_NAME, function(event) {
            if(options.acceptDropping){
                const accept = options.acceptDropping(event);
                if(accept){
                    event.target.classList.add(DROPPING_CLASS);
                }
            }
        });

        node.addEventListener('releaseDropping', function(event) {
            event.target.classList.remove(DROPPING_CLASS,`${DROPPING_CLASS}-before`,`${DROPPING_CLASS}-after`);
        });
    }

    return {
        update: newOptions => {

        },
        destroy: () => {

        }
    };
}