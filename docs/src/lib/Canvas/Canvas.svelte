<script lang="ts">
  import Content from "./Content.svelte";
  import { initOnlyOnce } from "../../helpers/wasmHelper";
  import type { Writer } from "../../renderer/types";

  export let writer: Writer;

  let width = writer.style.width;
  let height = writer.style.height;
</script>

{#await initOnlyOnce()}
  <div class="loader" style={`width:${width}px; height:${height}px`}>
    loading wasm modules...
  </div>
{:then _}
  <Content {writer} {width} {height} />
{/await}

<style>
  .loader {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
  }
</style>
