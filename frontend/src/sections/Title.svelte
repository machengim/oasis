<script lang="ts">
  import { Link } from "svelte-navigator";
  import { t, isLoading as isLoadingI18N } from "svelte-i18n";
  import { captilizeFirst } from "../utils/util";

  export let section: string;
</script>

{#if $isLoadingI18N}
  <div>Waiting</div>
{:else}
  <div class="text-xl">
    <Link to="/">Oasis</Link>
    {#if section}
      &gt;
      <Link to={"/" + section}>{$t("section." + section)}</Link>
    {/if}
  </div>
{/if}
