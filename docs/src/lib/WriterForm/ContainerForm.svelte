<script lang="ts">
import type { Container } from "../../renderer/types";
import ChildList from "./ChildList.svelte";
import StyleForm from "./StyleForm.svelte";
import Title from "./Title.svelte";

import WriterForm from "./WriterForm.svelte";

export let element: Container & { id: number };

</script>

<section>
  <details open>
    <summary>
      <Title>
        <h2>Container</h2>
      </Title>
    </summary>

    <ChildList>
      <li>
        <StyleForm bind:style={element.style} />
      </li>
      <li>
        <fieldset>
          <legend>Writer</legend>
          <WriterForm bind:writer={element.writer} />
        </fieldset>
      </li>
    </ChildList>
  </details>
</section>
