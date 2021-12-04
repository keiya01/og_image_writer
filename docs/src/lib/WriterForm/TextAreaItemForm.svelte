<script lang="ts">
  import { getDefaultStyleObj } from "../../renderer/style";
  import type { TextAreaItem } from "../../renderer/types";
  import FormLabel from "./FormLabel.svelte";
  import InlineFileInput from "./InlineFileInput.svelte";
  import ChildList from "./ChildList.svelte";
  import StyleForm from "./StyleForm.svelte";
  import Title from "./Title.svelte";

  export let item: TextAreaItem;
  let files: FileList;
  let style = getDefaultStyleObj();
  let hasStyle = false;

  $: {
    if (files?.length) {
      files[0].arrayBuffer().then((buf) => {
        item.font = new Uint8Array(buf);
      });
    }
    item.style = hasStyle ? style : undefined;
  }
</script>

<section>
  <details open>
    <summary>
      <Title>
        <h4>Text</h4>
      </Title>
    </summary>

    <ChildList>
      <li>
        <FormLabel>
          text
          <input slot="input" type="text" bind:value={item.text} />
        </FormLabel>
      </li>

      <li>
        <InlineFileInput bind:files>
          Select a child font file(.ttf)
        </InlineFileInput>
      </li>

      <li>
        <fieldset>
          <legend>style</legend>
          <p>
            <FormLabel>
              <input slot="input" type="checkbox" bind:checked={hasStyle} />
              Does element have style?
            </FormLabel>
          </p>
          {#if hasStyle}
            <StyleForm bind:style />
          {/if}
        </fieldset>
      </li>
    </ChildList>
  </details>
</section>

<style>
  legend {
    font-size: 16px;
  }
</style>
