<script context="module">
  export const ELEMENT_SELECT_FORM_ID = "element-select-form";
</script>

<script lang="ts">
  import type { Element } from "../../renderer/types";
  import { createEventDispatcher } from "svelte";
  import InlineButton from "../InlineButton";

  export let type: Element["type"];

  const clickDispatcher = createEventDispatcher();

  let handleOnClick = () => {
    clickDispatcher("click");
  };
</script>

<div class="container" id={ELEMENT_SELECT_FORM_ID}>
  <select bind:value={type} name="element-type">
    <option value="">-- Select a drawing element --</option>
    <option value="textarea">textarea</option>
    <option value="img">img</option>
    <option value="container">container</option>
  </select>
  <InlineButton on:click={handleOnClick}>Add the selected element</InlineButton>
</div>

<style>
  .container {
    margin-top: 20px;
  }

  select {
    height: 40px;
    background: var(--base-color);
    border: 2px solid #ccc;
    border-radius: 5px;
    padding: 5px 10px;
    font-weight: bold;
    box-sizing: border-box;
    font-size: 16px;
  }
</style>
