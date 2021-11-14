<script lang="ts">
  import github from '../../assets/github.png?url';
</script>

<header>
  <h1>Playground</h1>
  <a href="https://github.com/keiya01/og_image_writer">
    <img src={github} alt="GitHub" height={120} width={120} />
  </a>
</header>

<style>
  header {
    align-items: center;
    display: flex;
    grid-template-rows: 1fr 60px;
    width: 100%;
    padding: 5px 20px;
    box-sizing: border-box;
  }

  h1 {
    color: var(--main-color);
    margin: 0;
    flex: 1;
  }

  img {
    width: 40px;
    height: 40px;
    border-radius: 50%;
  }
</style>
