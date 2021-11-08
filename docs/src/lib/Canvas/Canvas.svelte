<script lang="ts">
  import Content from './Content.svelte';
  import type { Writer } from '../../renderer/types';

  export let writer: Writer;
</script>

<Content writer={writer}/>
