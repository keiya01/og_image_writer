<label>
  <slot />
  <slot name="input" />
</label>

<style>
  label {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    font-size: 16px;
    font-weight: bold;
  }

  label > :global(input:not(input[type="radio"])) {
    margin-left: 7px;
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
