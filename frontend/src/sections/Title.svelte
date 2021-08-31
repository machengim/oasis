<script lang="ts">
  import { Link } from "svelte-navigator";
  import { captilizeFirst } from "../utils/util";

  export let section: string;
</script>

<div class="text-xl">
  <Link to="/">Oasis</Link>
  {#if section}
    &gt;
    <Link to={"/" + section}>{captilizeFirst(section)}</Link>
  {/if}
</div>
