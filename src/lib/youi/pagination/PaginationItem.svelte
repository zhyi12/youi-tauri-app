<script>
  import classnames from '../util/utils';

  let className = '';
  export { className as class };
  export let active = false;
  export let disabled = false;

  $: classes = classnames(className, 'page-item', {
    active,
    disabled
  });
</script>

<li {...$$restProps} class={classes}>
  <slot />
</li>
