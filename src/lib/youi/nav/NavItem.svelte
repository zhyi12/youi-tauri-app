<script>
  import classnames from '../util/utils';

  let className = '';
  export { className as class };
  export let active = false;

  $: classes = classnames(className, 'nav-item', active ? 'active' : false);
</script>

<li {...$$restProps} class={classes}>
  <slot />
</li>
