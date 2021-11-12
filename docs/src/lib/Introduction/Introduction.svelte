<script lang="ts">
  import { GITHUB_URL, MDN_CSS } from '../../constants/url';
</script>

<section>
  <h2>What is this?</h2>
  <p>This is documentation for <a href={GITHUB_URL}>og_image_writer</a> library written in Rust.</p>
  <p>You can create image dynamically by using <a href={MDN_CSS}>CSS</a> like API.</p>
</section>

<section>
  <h2>How to use the playgroud?</h2>
  <p>You can draw element and styling on image at the top of page by entering some infomation at the below input field.</p>
  <p>All style properties behave like <a href={MDN_CSS}>CSS</a>.</p>
  <h3>Drawing Steps</h3>
  <ol>
    <li>Setting up <b>Window Style Properties</b> fields.</li>
    <ul>
      <li><b>Window Style</b> specify style for top level window. All element is drawn on this window.</li>
    </ul>
    <li>Select a drawing element from select form then click the adding button.</li>
    <ul>
      <li><code>textarea</code> is an element for indicating text. You need to set font data for drawing on window.</li>
      <li><code>img</code> is an element for indicating image data. You need to set image data for drawing on window.</li>
      <li><code>container</code> is an element for indicating child window.</li>
    </ul>
    <li>Setting up element style properties.</li>
  </ol>
</section>

<style>
  section {
    margin: 40px 0 0;
  }

  section:first-of-type {
    margin: 0;
  }

  h2 {
    display: inline-block;
    margin: 0;
    font-size: calc(25rem / 16);
  }

  p {
    font-size: calc(17rem / 16);
    line-height: 1.8;
    margin: 20px 0 0;
  }

  h3 {
    font-size: calc(20rem / 16);
    margin: 30px 0 0;
  }

  li {
    font-size: calc(17rem / 16);
    line-height: 1.8;
  }

  ol > li {
    margin-top: 20px;
  }

  ul > li {
    margin-top: 10px;
  }
</style>
