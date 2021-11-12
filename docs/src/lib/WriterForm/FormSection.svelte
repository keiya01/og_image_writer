<script lang="ts">
  export let id: string | undefined = undefined;
</script>

<section id={id}>
  <slot></slot>
</section>

<style>
  section {
    background: #eee;
    border-radius: 5px;
    padding: 15px;
    box-sizing: border-box;
    display: inline-block;
    min-width: 100%;
  }
</style>
