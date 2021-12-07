<script lang="ts">
  import { GITHUB_URL, MDN_CSS } from "../../constants/url";
  import Code from "../Code/Code.svelte";
</script>

<section>
  <h2>What is this?</h2>
  <p>
    This is documentation for <a href={GITHUB_URL}>og_image_writer</a> library
    written in Rust. <br />
    You can create a image dynamically by using <a href={MDN_CSS}>CSS</a> like API.
  </p>
</section>

<section>
  <h2>How to use the playgroud?</h2>
  <p>
    You can draw an element and styling on window image by entering some styling
    value at the below input field.<br />
    All style properties behave like <a href={MDN_CSS}>CSS</a>.
  </p>
  <h3>Drawing Steps</h3>
  <ol>
    <li>Set up <b>Window Style Properties</b> fields.</li>
    <ul>
      <li>
        <b>Window Style</b> indicates layout for top level window image. All element
        is drawn on this window.
      </li>
    </ul>
    <li>
      Select a drawing element from select form then click the adding button.
    </li>
    <ul>
      <li>
        <Code>Textarea</Code> is an element for indicating text. You need to set
        font data to <Code>parent font</Code> or <Code>global font context</Code
        > for drawing on the window.
      </li>
      <ul>
        <li>
          <Code>Global font context</Code> is global fallback font family.
        </li>
        <li>
          <Code>Prent font</Code> is font family for <Code>textarea</Code>.
        </li>
        <li>
          <Code>Child font</Code> is font family for <Code>textarea items</Code
          >.
        </li>
      </ul>
      <li>
        <Code>Img</Code> is an element for indicating image data. You need to set
        image data for drawing on the window.
      </li>
      <li>
        <Code>Container</Code> is an element for indicating the child window.
      </li>
    </ul>
    <li>Set up element's style Properties.</li>
    <ul>
      <li><b>Element's style</b> indicates layout for each element.</li>
    </ul>
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
