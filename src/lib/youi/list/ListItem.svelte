<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<script lang="ts">
  import classnames from "../util/classname";

  let className = '';
  export { className as class };

  $: classes = classnames("list-item", className);
</script>

<li
  class={classes}
  {...$$restProps}
  on:click
  on:mouseover
  on:mouseenter
  on:mouseleave
>
  <slot />
</li>
