<script lang="ts">
  export let hasListStyle = true;
</script>

<ul style={!hasListStyle && "list-style: none;"} role="list">
  <slot></slot>
</ul>

<style>
  ul > :global(li) {
    margin-top: 15px;
  }
</style>
