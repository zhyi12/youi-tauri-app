<script lang="ts">

	import Konva from 'konva';
	import { createEventDispatcher, getContext, onMount } from 'svelte';
	import context from './context';
	import { appendKonvaElement } from './util';

	export let text: string = null;
	export let textDecoration: string = null;
	export let stroke: string = null;
    export let strokeWidth:number = null;

	export let x: number = null;
	export let y: number = null;
	export let width: number = null;
	export let height: number = null;
	export let visible: boolean = null;
	export let listening: boolean = null;
	export let id: string = null;
	export let name: string = null;
	export let padding: number = 2;
	export let opacity: number = null;
	export let scale: Konva.Vector2d = { x: null, y: null };
	export let scaleX: number = null;
	export let scaleY: number = null;
	export let rotation: number = null;
	export let offset: Konva.Vector2d = { x: null, y: null };
	export let offsetX: number = null;
	export let offsetY: number = null;
    export let align: string = null;
    export let verticalAlign:string = null;

	export let draggable: boolean = null;
	export let dragDistance: number = null;
	export let dragBoundFunc: (pos: Konva.Vector2d) => Konva.Vector2d = null;

    export let fontStyle:string = null;
    export let fontFamily:string = null;

	const dispatch = createEventDispatcher();
	const parent: () => Konva.Container = getContext(context.parent);

	let element:Konva.Text = null;

	onMount(()=>{
		const parentElement = parent();
		const config = {x,y,width,height,padding,offset,offsetX,offsetY,rotation,text,textDecoration,stroke,strokeWidth,align,verticalAlign,
            fontStyle,fontFamily};
		element = appendKonvaElement(parentElement,'Text',config);
		dispatch('added', { element, parentElement });
		return ()=>{
			element.destroy();
			element = null;
		}
	});

	$: element && element.text(text);
	$: element && element.x(x);
	$: element && element.y(y);
	$: element && element.width(width);
	$: element && element.height(height);
	$: element && element.align(align);
	$: element && element.fontStyle(fontStyle);
	$: element && element.fontFamily(fontFamily);

</script>