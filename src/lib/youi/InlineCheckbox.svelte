<script>
  /** Specify whether the checkbox is checked */
  export let checked = false;

  /** Specify whether the checkbox is indeterminate */
  export let indeterminate = false;

  /** Set an id for the input label */
  export let id = "ccs-" + Math.random().toString(36);

  /** Obtain a reference to the input HTML element */
  export let ref = null;
</script>

<input
  bind:this="{ref}"
  type="checkbox"
  class:bx--checkbox="{true}"
  checked="{indeterminate ? false : checked}"
  indeterminate="{indeterminate}"
  id="{id}"
  {...$$restProps}
  aria-label="{undefined}"
  aria-checked="{indeterminate ? 'mixed' : checked}"
  on:change
/>
