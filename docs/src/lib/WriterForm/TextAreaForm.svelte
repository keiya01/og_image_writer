<script lang="ts">
  import { createTextAreaItem } from "../../renderer/draw";
  import type { TextAreaItem, TextAreaObj } from "../../renderer/types";
  import InlineButton from "../InlineButton";
  import InlineFileInput from "./InlineFileInput.svelte";
  import ChildList from "./ChildList.svelte";
  import StyleForm from "./StyleForm.svelte";
  import TextAreaItemForm from "./TextAreaItemForm.svelte";
  import Title from "./Title.svelte";

  export let element: TextAreaObj & { id: number };

  let area: TextAreaItem[] = [];
  let files: FileList;

  $: {
    if (files?.length) {
      files[0].arrayBuffer().then((buf) => {
        element.font = new Uint8Array(buf);
      });
    }
    element.area = area;
  }

  const handleAddArea = () => {
    area = [...area, createTextAreaItem()];
  };
</script>

<section>
  <details open>
    <summary>
      <Title>
        <h2>TextArea</h2>
      </Title>
      <p>
        textarea is an element for indicating text. You need to set font data
        for drawing on window.
      </p>
    </summary>
    <ChildList>
      <li>
        <InlineFileInput bind:files>
          Select a parent font file(.ttf or .otf)
        </InlineFileInput>
      </li>

      <li>
        <StyleForm bind:style={element.style} />
      </li>

      <li>
        <section>
          <details open>
            <summary>
              <Title>
                <h3>Children</h3>
              </Title>
            </summary>

            <div class="buttonContainer">
              <InlineButton on:click={handleAddArea}
                >Add textarea item</InlineButton
              >
            </div>

            <ChildList hasListStyle={false}>
              {#each area as item}
                <li class="textarea-item-list-item">
                  <TextAreaItemForm bind:item />
                </li>
              {/each}
            </ChildList>
          </details>
        </section>
      </li>
    </ChildList>
  </details>
</section>

<style>
  .buttonContainer {
    margin-top: 15px;
  }

  .textarea-item-list-item {
    position: relative;
    margin-top: 20px;
  }

  .textarea-item-list-item::before {
    position: absolute;
    top: -10px;
    left: -35px;
    border-bottom: 5px solid #777;
    border-left: 5px solid #777;
    border-bottom-left-radius: 10px;
    content: "";
    display: block;
    height: 20px;
    width: 25px;
  }
</style>
