<script lang="ts">

	export let x = 0;
	export let y = 0;
	export let width = 0;
	export let height = 0;
	export let strokeWidth = 1;
	export let stroke = 'rgb(26, 115, 232)';
	export let strokeStyle = "solid";

	const toPixel = (num)=>num+'px';

</script>

<div class='box-fill'
		 style:left={toPixel(x)} style:top={toPixel(y)} style:width={toPixel(width)} style:height={toPixel(height)}></div>

<div class='box-line box-top'
		 style:left={toPixel(x)} style:top={toPixel(y)} style:width={toPixel(width)} style:height={toPixel(strokeWidth)}
		 style:border-color={stroke} style:border-top-width={toPixel(strokeWidth)} style:border-style={strokeStyle}
></div>

<div class='box-line box-right'
		 style:left={toPixel(x+width)} style:top={toPixel(y)} style:width={toPixel(strokeWidth)} style:height={toPixel(height)}
		 style:border-color={stroke} style:border-right-width={toPixel(strokeWidth)} style:border-style={strokeStyle}
></div>

<div class='box-line box-bottom'
		 style:left={toPixel(x)} style:top={toPixel(y+height)} style:width={toPixel(width+strokeWidth)} style:height={toPixel(strokeWidth)}
		 style:border-color={stroke} style:border-bottom-width={toPixel(strokeWidth)} style:border-style={strokeStyle}
></div>

<div class='box-line box-left'
		 style:left={toPixel(x)} style:top={toPixel(y)} style:width={toPixel(strokeWidth)} style:height={toPixel(height)}
		 style:border-color={stroke} style:border-left-width={toPixel(strokeWidth)} style:border-style={strokeStyle}
></div>

<style>
	.box-line{
      border-width: 0;
      position: absolute;
      pointer-events: none;
	}

	.box-fill{
      position: absolute;
      background-color: rgba(14, 101, 235, 0.1);
      opacity: 1;
      user-select: none;
      pointer-events: none;
	}
</style>
