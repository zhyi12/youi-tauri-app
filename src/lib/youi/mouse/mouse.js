

export let dragContext = undefined;

export function mouse(node, options) {

    if(!options)return;

    let mouseHandled = false;
    let started = false;
    let _mouseMoved = false;
    let _mouseStarted = false;
    let _mouseDownEvent = undefined;

    let ignoreMissingWhich;
    let _mouseDelayTimer;

    let mouseDelayMet = undefined;
    /**
     *
     * @param event
     */
    async function _mousedown(event){
        // don't let more than one widget handle mouseStart and skip right click
        if ( mouseHandled || event.button ===2) {
            return;
        }

        _mouseMoved = false;

        // We may have missed mouseup (out of window)
        ( _mouseStarted && _mouseUp( event ) );

        _mouseDownEvent = event;

        const btnIsLeft = ( event.which === 1 );

        // event.target.nodeName works around a bug in IE 8 with
        // disabled inputs (#7620)
        const elIsCancel = ( typeof options.cancel === "string" && event.target.nodeName ?
             event.target.closest( options.cancel )!=null : false );
        if ( !btnIsLeft || elIsCancel  ) {//|| !_mouseCapture( event )
            return true;
        }

        mouseDelayMet = !options.delay;

        if ( !mouseDelayMet ) {
            _mouseDelayTimer = setTimeout( function() {
                mouseDelayMet = true;
            }, options.delay );
        }

        if ( _mouseDistanceMet( event ) && _mouseDelayMet( event ) ) {
            _mouseStarted = ( _mouseStart( event ) !== false );
            if ( !_mouseStarted ) {
                event.preventDefault();
                return true;
            }
        }

        // Click event may never have fired (Gecko & Opera)
        // if ( true === $.data( event.target, this.widgetName + ".preventClickEvent" ) ) {
        //     $.removeData( event.target, this.widgetName + ".preventClickEvent" );
        // }

        // These delegates are required to keep context

        document.addEventListener('mousemove',_mouseMoveDelegate);
        document.addEventListener('mouseup',_mouseUpDelegate);

        event.preventDefault();

        mouseHandled = true;
        return true;
    }

    function _mouseMoveDelegate( event ) {
        _mouseMove( event );
    }

    function  _mouseUpDelegate( event ) {
        return _mouseUp( event );
    }

    function _mouseMove(event){
        // Only check for mouseups outside the document if you've moved inside the document
        // at least once. This prevents the firing of mouseup in the case of IE<9, which will
        // fire a mousemove event if content is placed under the cursor. See #7778
        // Support: IE <9
        if ( _mouseMoved ) {

            // IE mouseup check - mouseup happened when mouse was out of window
            // if ( $.ui.ie && ( !document.documentMode || document.documentMode < 9 ) &&
            //     !event.button ) {
            //     return this._mouseUp( event );
            //
            //     // Iframe mouseup check - mouseup occurred in another document
            // } else if ( !event.which ) {

            // Support: Safari <=8 - 9
            // Safari sets which to 0 if you press any of the following keys
            // during a drag (#14461)
            // if ( event.altKey || event.ctrlKey ||
            //     event.metaKey || event.shiftKey ) {
            //     ignoreMissingWhich = true;
            // } else if ( !ignoreMissingWhich ) {
            //     return _mouseUp( event );
            // }
            // }
        }

        if ( event.which || event.button ) {
            _mouseMoved = true;
        }

        if ( _mouseStarted ) {
            _mouseDrag( event );
            return event.preventDefault();
        }

        if ( _mouseDistanceMet( event ) && _mouseDelayMet( event ) ) {
            _mouseStarted =
                ( _mouseStart(event ) !== false );
            ( _mouseStarted ? _mouseDrag( event ) : _mouseUp( event ) );
        }

        return !_mouseStarted;
    }

    function _mouseDelayMet(event){
        return mouseDelayMet;
    }

    function _mouseUp(event){
        document.removeEventListener('mousemove',_mouseMoveDelegate);
        document.removeEventListener('mouseup',_mouseUpDelegate);

        if ( _mouseStarted ) {
            //
            _mouseStarted = false;
            //if ( event.target === this._mouseDownEvent.target ) {
            //$.data( event.target, this.widgetName + ".preventClickEvent", true );
            //}
            _mouseStop( event );
        }else{
            _normalMouseUp(event);
        }

        if ( _mouseDelayTimer ) {
            clearTimeout( _mouseDelayTimer );
            _mouseDelayTimer = undefined;
        }

        ignoreMissingWhich = false;
        mouseHandled = false;
        event.preventDefault();
    }

    function _mouseStart(event){
        return options.mouseStart && options.mouseStart(event);
    }

    function _mouseDrag( event ){
        options.mouseDrag && options.mouseDrag(event);
    }

    function _normalMouseUp(event){
        options.normalMouseUp && options.normalMouseUp(event);
    }

    function _mouseStop(event){
        let dragData = {};

        if(options.mouseStop){
            dragData = options.mouseStop(event);
        }
        dragContext = dragData;
        //trigger(event.target,'dropStop',{});
    }

    function _click(event){
        // event.target.removeEventListener('mousedown',_mousedown);
        // event.target.removeEventListener('mousedown',_mouseup);
        event.stopPropagation();
    }

    function _mouseDistanceMet( event ) {
        return ( Math.max(
                Math.abs( _mouseDownEvent.pageX - event.pageX ),
                Math.abs( _mouseDownEvent.pageY - event.pageY )
            ) >= 1
        );
    }

    function _drop(e){
        if(options.doDrop){
            options.doDrop(e);
        }
    }

    function _mouseInit(node,options) {
        node.addEventListener('mousedown',_mousedown,options||{});
        node.addEventListener('click',_click);

        if(options.droppable){
            node.addEventListener('drop',_drop);
        }
        started = false;
    }

    /**
     *
     */
    _mouseInit(node,options);

    return {
        update: newOptions => {
            console.log('use mouse')
            // validateOptions(newOptions);
            // pointerZone.update(newOptions);
            // keyboardZone.update(newOptions);
        },
        destroy: () => {
            // pointerZone.destroy();
            // keyboardZone.destroy();
        }
    };
}
