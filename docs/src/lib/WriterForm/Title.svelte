<span>
  <slot class="title" />
</span>

<style>
  span > :global(h2),
  span > :global(h3),
  span > :global(h4) {
    display: inline-block;
    margin: 0;
  }

  span > :global(h2) {
    font-size: 25px;
  }

  span > :global(h3) {
    font-size: 22px;
  }

  span > :global(h4) {
    font-size: 20px;
  }
</style>
