<script lang="ts">
import Header from './lib/Header';
import Playground from './Playground.svelte';
</script>

<Header />

<main>
  <Playground />
</main>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    font-family: arial, sans-serif;
  }

  :global(main) {
    margin: 20px 20px 120px;
  }

  :global(:root) {
    --main-color: #963f09;
    --base-color: #fff;
    --focus-color: #0d6eff;
    --code-background-color: #ddd;
  }

  :global(a:focus), :global(button:focus), :global(input:focus), :global(select:focus) {
    outline: 3px solid var(--focus-color);
  }
</style>
