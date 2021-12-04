<script lang="ts">
  export let files: FileList;
  export let multiple = false;

  let filename: string;
  $: filename = files?.length ? files[0].name : "No file selected.";
</script>

<label tabindex="0">
  <div class="button">
    <slot></slot>
  </div>
  {filename}
  <input type="file" bind:files multiple={multiple} />
</label>

<style>
  label {
    display: inline-block;
    font-size: 15px;
  }

  .button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    height: 40px;
    padding: 0 10px;
    font-weight: bold;
    font-size: 15px;
    background: var(--main-color);
    color: var(--base-color);
    border: none;
    border-radius: 5px;
    box-sizing: border-box;
  }

  .button:focus {
    outline: 3px solid var(--focus-color);
  }

  input {
    display: none;
  }
</style>
