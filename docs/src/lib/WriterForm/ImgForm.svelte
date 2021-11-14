<script lang="ts">
import type { ImgObj } from "../../renderer/types";
import FormLabel from "./FormLabel.svelte";
import InlineFileInput from "./InlineFileInput.svelte";
import ChildList from "./ChildList.svelte";
import StyleForm from "./StyleForm.svelte";
import Title from "./Title.svelte";
import ErrorMessage from "./ErrorMessage.svelte";
import Code from "../Code/Code.svelte";

export let element: ImgObj & { id: number };

let files: FileList;
let errors = {
  file: false,
}

$: {
  if(files?.length) {
    const file = files[0];
    let ext = file.name.split(".").pop();
    if(!["png", "PNG"].includes(ext)) {
      errors.file = true;
    } else {
      errors.file = false;
      file.arrayBuffer().then((buf) => {
        element.data = new Uint8Array(buf);
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
    </summary>

    <ChildList>
      <li>
        <InlineFileInput bind:files>
          Select a PNG image file
        </InlineFileInput>
        {#if errors.file}
          <div class="error-message">
            <ErrorMessage>Currently, only images in <Code>.png</Code> format are supported.</ErrorMessage>
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
          <input slot="input" type="number" bind:value={element.width}/>
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
