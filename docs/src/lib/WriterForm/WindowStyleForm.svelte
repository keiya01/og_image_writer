<script context="module">
  export const WINDOW_STYLE_FORM_ID = "window-style-form";
</script>

<script lang="ts">
  import { AlignItems, FlexDirection, JustifyContent } from "og_image_writer";
  import type { WindowStyleObj } from "../../renderer/style";
  import FormLabel from "./FormLabel.svelte";
  import ChildList from "./ChildList.svelte";
  import FormSection from "./FormSection.svelte";
  import Title from "./Title.svelte";

  export let style: WindowStyleObj;

  let flexDirection: "Column" | "Row" = "Column";
  let alignItems: "Start" | "Center" | "End" = "Start";
  let justifyContent: "Start" | "Center" | "End" = "Start";

  $: {
    style.flexDirection = FlexDirection[flexDirection];
    style.alignItems = AlignItems[alignItems];
    style.justifyContent = JustifyContent[justifyContent];
  }
</script>

<FormSection id={WINDOW_STYLE_FORM_ID}>
  <details open>
    <summary>
      <Title>
        <h2>Window Style Properties</h2>
      </Title>
      <p>
        Window Style indicates layout for top level window image. All element is
        drawn on this window.
      </p>
    </summary>

    <ChildList>
      <li>
        <FormLabel>
          height
          <input slot="input" type="number" bind:value={style.height} />
        </FormLabel>
      </li>

      <li>
        <FormLabel>
          width
          <input slot="input" type="number" bind:value={style.width} />
        </FormLabel>
      </li>

      <li>
        <fieldset>
          <legend>background_color</legend>
          <ul>
            <li>
              <FormLabel>
                r
                <input
                  slot="input"
                  type="number"
                  bind:value={style.backgroundColor.r}
                />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                g
                <input
                  slot="input"
                  type="number"
                  bind:value={style.backgroundColor.g}
                />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                b
                <input
                  slot="input"
                  type="number"
                  bind:value={style.backgroundColor.b}
                />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                a
                <input
                  slot="input"
                  type="number"
                  bind:value={style.backgroundColor.a}
                />
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>

      <li>
        <fieldset>
          <legend>flex_direction</legend>
          <ul>
            <li>
              <FormLabel>
                <input
                  slot="input"
                  type="radio"
                  name="flex_direction"
                  bind:group={flexDirection}
                  value="Column"
                  checked={flexDirection === "Column"}
                />
                Column
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                <input
                  slot="input"
                  type="radio"
                  name="flex_direction"
                  bind:group={flexDirection}
                  value="Row"
                  checked={flexDirection === "Row"}
                />
                Row
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>

      <li>
        <fieldset>
          <legend>align_items</legend>
          <ul>
            <li>
              <FormLabel>
                <input
                  slot="input"
                  type="radio"
                  name="align_items"
                  bind:group={alignItems}
                  value="Start"
                  checked={alignItems === "Start"}
                />
                Start
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                <input
                  slot="input"
                  type="radio"
                  name="align_items"
                  bind:group={alignItems}
                  value="Center"
                  checked={alignItems === "Center"}
                />
                Center
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                <input
                  slot="input"
                  type="radio"
                  name="align_items"
                  bind:group={alignItems}
                  value="End"
                  checked={alignItems === "End"}
                />
                End
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>

      <li>
        <fieldset>
          <legend>justify_content</legend>
          <ul>
            <li>
              <FormLabel>
                <input
                  slot="input"
                  type="radio"
                  name="justify_content"
                  bind:group={justifyContent}
                  value="Start"
                  checked={justifyContent === "Start"}
                />
                Start
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                <input
                  slot="input"
                  type="radio"
                  name="justify_content"
                  bind:group={justifyContent}
                  value="Center"
                  checked={justifyContent === "Center"}
                />
                Center
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                <input
                  slot="input"
                  type="radio"
                  name="justify_content"
                  bind:group={justifyContent}
                  value="End"
                  checked={justifyContent === "End"}
                />
                End
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>
    </ChildList>
  </details>
</FormSection>

<style>
  legend {
    font-size: 16px;
  }
</style>
