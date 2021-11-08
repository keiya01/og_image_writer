<script lang="ts">
  import type { ElementFormData } from "./WriterForm.svelte";
import ContainerForm from "./ContainerForm.svelte";
import TextAreaForm from "./TextAreaForm.svelte";
import ImgForm from "./ImgForm.svelte";
import FormSection from "./FormSection.svelte";

  export let element: ElementFormData[number];
</script>

<FormSection>
  {#if element.type === "textarea"}
    <TextAreaForm bind:element />
  {:else if element.type === "img"}
    <ImgForm bind:element />
  {:else if element.type === "container"}
    <ContainerForm bind:element />
  {:else}
    <p>nothing</p>
  {/if}
</FormSection>
