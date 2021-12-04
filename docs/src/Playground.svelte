<script lang="ts">
  import { overrideWindowStyle } from "./constants/styles";
  import Canvas from "./lib/Canvas";
  import Introduction from "./lib/Introduction/Introduction.svelte";
  import WriterForm from "./lib/WriterForm/WriterForm.svelte";
  import { createWriter } from "./renderer/draw";

  let writer = createWriter(overrideWindowStyle);
</script>

<div class="introduction">
  <Introduction />
</div>
<div class="canvas">
  <div class="canvas-inner">
    <Canvas {writer} />
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
    width: calc(100% + 40px);
    margin-top: 40px;
    margin-left: -20px;
  }

  .canvas-inner {
    display: inline-block;
    padding: 0 20px;
    box-sizing: border-box;
  }

  .introduction {
    margin: 40px 0 0;
  }

  .writer-form {
    box-sizing: border-box;
    margin-top: 40px;
    overflow-x: auto;
    width: calc(100% + 40px);
    margin-left: -20px;
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
