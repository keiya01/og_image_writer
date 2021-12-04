<script lang="ts" context="module">

  let timeoutId: NodeJS.Timeout;
</script>

<script lang="ts">
  import type { Writer } from '../../renderer/types';
  import { renderImg } from '../../renderer/render';
  import { drawImg } from '../../renderer/draw';
  import { GITHUB_URL } from '../../constants/url';

  export let writer: Writer;
  export let height = 0;
  export let width = 0;

  let rendering = false;
  let error: { title: string, body: string } | null = null;

  let canvas: HTMLCanvasElement;

  const createError = (e: Error) => {
    const title = `[Web Document Runtime Error]: ${e.name} occurred.`
    const body = `
\`\`\`
name: ${e.name}
message: ${e.message}
${e.stack}
\`\`\``;
    return {
      title,
      body,
    };
  }

  $: {
    if(timeoutId) {
      clearTimeout(timeoutId);
    }

    const w = writer.style.width || width;
    const h = writer.style.height || height;
    if(canvas && w && h) {
      rendering = true;
      timeoutId = setTimeout(() => {
        try {
          const imgWriter = drawImg(writer, w, h);
          renderImg(canvas, imgWriter, w, h);
        } catch(e) {
          error = createError(e);
          console.error(e);
        } finally {
          rendering = false;
        }
      }, 500);
    }
  }
</script>

<canvas bind:this={canvas} width={width} height={height} />

<div class="state-container">
  {#if rendering}
    <span class="loading">rendering...</span>
  {:else if !!error}
    <span class="error-message" role="alert">
      Wasm runtime error occurred.<br/>
      If you have a time, please <a href={`${GITHUB_URL}/issues/new?title=${encodeURIComponent(error.title)}&body=${encodeURIComponent(error.body)}`}>send a issue with this url</a>.
    </span>
  {/if}
</div>

<style>
  .state-container {
    margin: 10px 0 0;
  }

  .loading {
    font-size: 1rem;
    color: var(--text-color-high);
  }

  .error-message {
    color: var(--error-color);
    font-size: 1rem;
  }
</style>
