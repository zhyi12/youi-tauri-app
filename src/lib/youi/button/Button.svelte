<script lang="ts">
  import classnames from '../util/utils';

  let className = '';
  /*  样式 */
  export { className as class };
  export let active = false;
  export let block = false;
  export let children = undefined;
  export let close = false;
  export let color = 'secondary';
  export let disabled = false;
  export let href = '';
  export let inner = undefined;
  export let outline = false;
  export let size = null;
  export let style = '';
  export let value = '';
  export let title = '';

  $: ariaLabel = $$props['aria-label'];

  $: classes = classnames(
    className,
    close ? 'btn-close' : 'btn',
    close || `btn${outline ? '-outline' : ''}-${color}`,
    size ? `btn-${size}` : false,
    block ? 'd-block w-100' : false,
    { active }
  );

  $: defaultAriaLabel = close ? 'Close' : null;
</script>

{#if href}
  <a
    {...$$restProps}
    class={classes}
    {disabled}
    bind:this={inner}
    on:click
    {href}
    aria-label={ariaLabel || defaultAriaLabel}
    {style}
    {title}
  >
    {#if children}
      {children}
    {:else}
      <slot />
    {/if}
  </a>
{:else}
  <button
    {...$$restProps}
    class={classes}
    {disabled}
    bind:this={inner}
    on:click
    {value}
    aria-label={ariaLabel || defaultAriaLabel}
    {style}
    {title}
  >
    <slot>
      {#if children}
        {children}
      {:else}
        <slot />
      {/if}
    </slot>
  </button>
{/if}
