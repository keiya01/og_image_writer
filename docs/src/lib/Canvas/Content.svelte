<script lang="ts" context="module">
  let timeoutId: NodeJS.Timeout;
</script>

<script lang="ts">
  import type { Writer } from '../../renderer/types';
  import { renderImg } from '../../renderer/render';
  import { drawImg } from '../../renderer/draw';

  export let writer: Writer;

  let canvas: HTMLCanvasElement;

  $: {
    if(timeoutId) {
      clearTimeout(timeoutId);
    }

    const w = writer.style.width;
    const h = writer.style.height;
    if(canvas && w && h) {
      timeoutId = setTimeout(() => {
        const imgWriter = drawImg(writer);
        renderImg(canvas, imgWriter, w, h);
      }, 500);
    }
  }
</script>

<canvas bind:this={canvas} width={500} height={300} />
