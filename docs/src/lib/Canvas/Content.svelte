<script lang="ts" context="module">

  let timeoutId: NodeJS.Timeout;
</script>

<script lang="ts">
  import type { Writer } from '../../renderer/types';
  import { renderImg } from '../../renderer/render';
  import { drawImg } from '../../renderer/draw';

  export let writer: Writer;
  export let height = 0;
  export let width = 0;

  let canvas: HTMLCanvasElement;

  $: {
    if(timeoutId) {
      clearTimeout(timeoutId);
    }

    const w = writer.style.width || width;
    const h = writer.style.height || height;
    if(canvas && w && h) {
      timeoutId = setTimeout(() => {
        const imgWriter = drawImg(writer, w, h);
        renderImg(canvas, imgWriter, w, h);
      }, 500);
    }
  }
</script>

<canvas bind:this={canvas} width={width} height={height} />
