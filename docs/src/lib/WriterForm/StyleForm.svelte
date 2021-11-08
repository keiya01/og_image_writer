<script context="module">
  let nameIndex = 0;
</script>

<script lang="ts">
import { Position, TextAlign, WordBreak } from "og_image_writer";

import type { StyleObj } from "../../renderer/style";
import FormLabel from "./FormLabel.svelte";
import ChildList from "./ChildList.svelte";
import Title from "./Title.svelte";

  let wordBreak: "Normal" | "BreakAll" = "Normal";
  let textAlign: "Start" | "Center" | "End" = "Start";
  let position: "Static" | "Absolute" = "Static";

  export let style: StyleObj;

  nameIndex += 1;

  $: {
    style.wordBreak = WordBreak[wordBreak];
    style.textAlign = TextAlign[textAlign];
    style.position = Position[position];
  }
</script>

<section>
  <details>
    <summary>
      <Title>
        <h3>Style Properties</h3>
      </Title>
    </summary>

    <ChildList>
      <li>
        <fieldset>
          <legend>border_radius</legend>
          <ul>
            <li>
              <FormLabel>
                top_left
                <input slot="input" type="number" bind:value={style.borderRadius.topLeft} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                top_right
                <input slot="input" type="number" bind:value={style.borderRadius.topRight} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                bottom_left
                <input slot="input" type="number" bind:value={style.borderRadius.bottomLeft} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                bottom_right
                <input slot="input" type="number" bind:value={style.borderRadius.bottomRight} />
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>

      <li>
        <fieldset>
          <legend>color</legend>
          <ul>
            <li>
              <FormLabel>
                r
                <input slot="input" type="number" bind:value={style.color.r} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                g
                <input slot="input" type="number" bind:value={style.color.g} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                b
                <input slot="input" type="number" bind:value={style.color.b} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                a
                <input slot="input" type="number" bind:value={style.color.a} />
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>

      <li>
        <FormLabel>
          font_size
          <input slot="input" type="number" bind:value={style.fontSize} />
        </FormLabel>
      </li>

      <li>
        <FormLabel>
          line_height
          <input slot="input" type="number" bind:value={style.lineHeight} />
        </FormLabel>
      </li>

      <li>
        <fieldset>
          <legend>margin</legend>
          <ul>
            <li>
              <FormLabel>
                top
                <input slot="input" type="number" bind:value={style.margin.top} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                right
                <input slot="input" type="number" bind:value={style.margin.right} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                bottom
                <input slot="input" type="number" bind:value={style.margin.bottom} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                left
                <input slot="input" type="number" bind:value={style.margin.left} />
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>

      <li>
        <FormLabel>
          max_height
          <input slot="input" type="number" bind:value={style.maxHeight} />
        </FormLabel>
      </li>

      <li>
        <FormLabel>
          max_width
          <input slot="input" type="number" bind:value={style.maxWidth} />
        </FormLabel>
      </li>

      <li>
        <fieldset>
          <legend>position</legend>
          <ul>
            <li>
              <FormLabel>
                Static
                <input slot="input" type="radio" name={`style-position-${nameIndex}`} bind:group={position} value="Static" checked={position === "Static"} />
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                Absolute
                <input slot="input" type="radio" name={`style-position-${nameIndex}`} bind:group={position} value="Absolute" checked={position === "Absolute"} />
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>

      <li>
        <FormLabel>
          top
          <input slot="input" type="number" bind:value={style.top} />
        </FormLabel>
      </li>

      <li>
        <FormLabel>
          right
          <input slot="input" type="number" bind:value={style.right} />
        </FormLabel>
      </li>

      <li>
        <FormLabel>
          bottom
          <input slot="input" type="number" bind:value={style.bottom} />
        </FormLabel>
      </li>

      <li>
        <FormLabel>
          left
          <input slot="input" type="number" bind:value={style.left} />
        </FormLabel>
      </li>

      <li>
        <fieldset>
          <legend>text_align</legend>
          <ul>
            <li>
              <FormLabel>
                <input slot="input" type="radio" name={`style-text-align-${nameIndex}`} bind:group={textAlign} value="Start" checked={textAlign === "Start"} />
                Start
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                <input slot="input" type="radio" name={`style-text-align-${nameIndex}`} bind:group={textAlign} value="Center" checked={textAlign === "Center"} />
                Center
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                <input slot="input" type="radio" name={`style-text-align-${nameIndex}`} bind:group={textAlign} value="End" checked={textAlign === "End"} />
                End
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>

      <li>
        <FormLabel>
          text_overflow
          <input slot="input" type="text" bind:value={style.textOverflow} />
        </FormLabel>
      </li>

      <li>
        <fieldset>
          <legend>word_break</legend>
          <ul>
            <li>
              <FormLabel>
                <input slot="input" type="radio" name={`style-word-break-${nameIndex}`} bind:group={wordBreak} value="Normal" checked={wordBreak === "Normal"} />
                Normal
              </FormLabel>
            </li>
            <li>
              <FormLabel>
                <input slot="input" type="radio" name={`style-word-break-${nameIndex}`} bind:group={wordBreak} value="BreakAll" checked={wordBreak === "BreakAll"} />
                BreakAll
              </FormLabel>
            </li>
          </ul>
        </fieldset>
      </li>
    </ChildList>
  </details>
</section>
