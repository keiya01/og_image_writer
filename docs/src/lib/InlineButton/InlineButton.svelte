<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const clickDispatcher = createEventDispatcher();

  let handleOnClick = () => {
    clickDispatcher("click");
  }
</script>

<button type="button" on:click={handleOnClick}><slot></slot></button>

<style>
  button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    height: 40px;
    padding: 0 10px;
    font-weight: bold;
    font-size: 16px;
    background: var(--main-color);
    color: var(--base-color);
    border: none;
    border-radius: 5px;
    box-sizing: border-box;
  }
</style>
