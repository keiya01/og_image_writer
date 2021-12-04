<script context="module" lang="ts">
  export type ElementFormData = (Element & { id: number })[];
</script>

<script lang="ts">
  import type { Element, FontContextObj, Writer } from "../../renderer/types";
  import {
    createContainer,
    createImg,
    createTextArea,
  } from "../../renderer/draw";
  import WindowStyleForm from "./WindowStyleForm.svelte";
  import ElementForm from "./ElementForm.svelte";
  import ElementSelect from "./ElementSelect.svelte";
  import FormSection from "./FormSection.svelte";
  import Title from "./Title.svelte";
  import InlineFileInput from "./InlineFileInput.svelte";
  import Spacer from "../Spacer";

  export let writer: Writer;

  let data: ElementFormData = [];

  let type: Element["type"];
  let currentId = 0;

  $: {
    writer.data = data.map((elm) => {
      const { id: _, ...rest } = elm;
      return rest;
    });
  }

  let files: FileList;

  $: {
    // TODO: validate font file
    if (files?.length) {
      const context: FontContextObj["context"] = [];
      Array.from(files).map((file) => {
        file.arrayBuffer().then((buf) => {
          context.push(new Uint8Array(buf));
        });
      });
      writer.fontContext.context = context;
    }
  }

  const handleAddElement = () => {
    if (!type) {
      return;
    }
    switch (type) {
      case "img": {
        data = [
          ...data,
          {
            id: currentId,
            ...createImg(),
          },
        ];
        break;
      }
      case "textarea": {
        data = [
          ...data,
          {
            id: currentId,
            ...createTextArea(),
          },
        ];
        break;
      }
      case "container": {
        data = [
          ...data,
          {
            id: currentId,
            ...createContainer(),
          },
        ];
      }
    }
    currentId += 1;
  };
</script>

<div>
  <form>
    <FormSection>
      <svelte:fragment>
        <Title>
          <h2>FontContext</h2>
        </Title>
        <p>
          You can set global fallback font.<br />
          Replaced with FontContext when chars is not found in parent font or child
          font.
        </p>
        <Spacer marginTop={20} />
        <InlineFileInput bind:files multiple
          >Select some global font file(.ttf).</InlineFileInput
        >
      </svelte:fragment>
    </FormSection>

    <Spacer marginTop={20} />

    <WindowStyleForm bind:style={writer.style} />

    <ElementSelect bind:type on:click={handleAddElement} />

    <ul class="element-form-list" role="list">
      {#each data as element (element.id)}
        <li class="element-form-item">
          <ElementForm bind:element />
        </li>
      {/each}
    </ul>
  </form>
</div>

<style>
  form {
    display: flex;
    flex-direction: column;
  }

  .element-form-list {
    list-style: none;
  }

  .element-form-item {
    position: relative;
    margin-top: 20px;
  }

  .element-form-item::before {
    position: absolute;
    top: 10px;
    left: -35px;
    border-bottom: 5px solid #777;
    border-left: 5px solid #777;
    border-bottom-left-radius: 10px;
    content: "";
    display: block;
    height: 20px;
    width: 25px;
  }

  form {
    margin-top: 10px;
  }
</style>
