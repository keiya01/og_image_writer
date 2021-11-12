<script lang="ts">
  import { overrideWindowStyle } from './constants/styles';
  import Canvas from './lib/Canvas'
  import WriterForm from './lib/WriterForm/WriterForm.svelte';
  import { createWriter } from './renderer/draw';

  let writer = createWriter(overrideWindowStyle);
</script>

<div class="canvas">
  <div class="canvas-inner">
    <Canvas writer={writer} />
  </div>
</div>
<div class="writer-form">
  <div class="writer-form-inner">
    <WriterForm bind:writer />
  </div>
</div>

<style>
  .canvas {
    box-sizing: border-box;
    overflow-x: auto;
    white-space: nowrap;
  }

  .canvas-inner {
    display: inline-block;
    padding: 0 20px;
    box-sizing: border-box;
  }

  .writer-form {
    box-sizing: border-box;
    margin-top: 10px;
    overflow-x: auto;
    white-space: nowrap;
  }

  .writer-form-inner {
    display: inline-block;
    min-width: 780px;
    width: 100%;
    padding: 0 20px;
    box-sizing: border-box;
  }
</style>
