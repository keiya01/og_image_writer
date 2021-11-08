<script lang="ts">
import type { ImgObj } from "../../renderer/types";
import FormLabel from "./FormLabel.svelte";
import InlineFileInput from "./InlineFileInput.svelte";
import ChildList from "./ChildList.svelte";
import StyleForm from "./StyleForm.svelte";
import Title from "./Title.svelte";

export let element: ImgObj & { id: number };

let files: FileList;

$: {
  if(files?.length) {
    files[0].arrayBuffer().then((buf) => {
      element.data = new Uint8Array(buf);
    });
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
            Select a image file
          </InlineFileInput>
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
