<script lang="ts">
  import Canvas from './lib/Canvas'
  import WriterForm from './lib/WriterForm/WriterForm.svelte';
  import { createWriter } from './renderer/draw';

  let writer = createWriter();
</script>

<div class="canvas-container">
  <Canvas writer={writer} />
</div>
<div class="writer-form">
  <div class="writer-form-inner">
    <WriterForm bind:writer />
  </div>
</div>

<style>
  .canvas-container {
    margin: 0 20px;
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
