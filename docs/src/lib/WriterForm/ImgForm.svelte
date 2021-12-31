<script lang="ts">
  import type { ImgObj } from "../../renderer/types";
  import FormLabel from "./FormLabel.svelte";
  import InlineFileInput from "./InlineFileInput.svelte";
  import ChildList from "./ChildList.svelte";
  import StyleForm from "./StyleForm.svelte";
  import Title from "./Title.svelte";
  import ErrorMessage from "./ErrorMessage.svelte";
  import Code from "../Code/Code.svelte";
  import { SUPPORTED_IMAGE_FORMAT } from "../../constants/image";
  import { ImageInputFormat } from "og_image_writer";

  export let element: ImgObj & { id: number };

  const toFormat = (ext: string) => {
    switch (ext) {
      case "png": {
        return ImageInputFormat.Png;
      }
      case "jpg" || "jpeg": {
        return ImageInputFormat.Jpeg;
      }
      default: {
        return null;
      }
    }
  };

  let files: FileList;
  let errors = {
    file: false,
  };

  $: {
    if (files?.length) {
      const file = files[0];
      let ext = file.name.split(".").pop().toLowerCase();
      if (!SUPPORTED_IMAGE_FORMAT.includes(ext.toLowerCase())) {
        errors.file = true;
      } else {
        errors.file = false;
        file.arrayBuffer().then((buf) => {
          element.data = new Uint8Array(buf);
          const format = toFormat(ext);
          if (format !== null) {
            element.format = format;
          } else {
            errors.file = true;
          }
        });
      }
    }
  }
</script>

<section>
  <details open>
    <summary>
      <Title>
        <h2>Img</h2>
      </Title>
      <p>
        img is an element for indicating image data. You need to set image data
        for drawing on window.
      </p>
    </summary>

    <ChildList>
      <li>
        <InlineFileInput bind:files
          >Select a image file(png or jpeg)</InlineFileInput
        >
        {#if errors.file}
          <div class="error-message">
            <ErrorMessage
              >Currently, only images in <Code>.png</Code> or <Code>.jpeg</Code>
              format are supported.</ErrorMessage
            >
          </div>
        {/if}
      </li>

      <li>
        <FormLabel>
          height
          <input slot="input" type="number" bind:value={element.height} />
        </FormLabel>
      </li>

      <li>
        <FormLabel>
          width
          <input slot="input" type="number" bind:value={element.width} />
        </FormLabel>
      </li>

      <li>
        <StyleForm bind:style={element.style} />
      </li>
    </ChildList>
  </details>
</section>

<style>
  .error-message {
    margin-top: 5px;
  }
</style>
