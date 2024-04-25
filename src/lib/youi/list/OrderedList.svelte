<script>
  /** Set to `true` to use the nested variant */
  export let nested = false;

  /** Set to `true` to use native list styles */
  export let native = false;

  /** Set to `true` to use Carbon's expressive typesetting */
  export let expressive = false;
</script>

<ol
  class:list-ordered="{!native}"
  class:list-ordered-native="{native}"
  class:list-nested="{nested}"
  class:list-expressive="{expressive}"
  {...$$restProps}
  on:click
  on:mouseover
  on:mouseenter
  on:mouseleave
  on:focus
>
  <slot />
</ol>
