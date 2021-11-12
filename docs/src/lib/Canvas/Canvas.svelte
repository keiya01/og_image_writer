<script lang="ts">
  import Content from './Content.svelte';
  import { initOnlyOnce } from '../../helpers/wasmHelper';
  import type { Writer } from '../../renderer/types';

  export let writer: Writer;

  let width = 500;
  let height = 300;
</script>

{#await initOnlyOnce()}
  <div class="loader" style={`width:${width}px;height:${height}px`}>loading wasm modules...</div>
{:then _}
  <Content writer={writer} width={width} height={height} />
{/await}

<style>
  .loader {
    font-size: 16px;
  }
</style>
