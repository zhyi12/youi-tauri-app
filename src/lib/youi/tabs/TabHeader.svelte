<script>
  import { setContext } from 'svelte';
  import classnames from "../util/classname";

  let className = '';
  export { className as class };

  setContext('tabs', true);

  $: classes = classnames("flex tabs",className);

</script>

<div class={classes}>
  <slot />
</div>
