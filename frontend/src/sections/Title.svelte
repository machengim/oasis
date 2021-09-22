<script lang="ts">
  import { Link } from "svelte-navigator";
  import { t, isLoading as isLoadingI18N } from "svelte-i18n";

  export let sitename: string;
  export let section: string;
</script>

{#if $isLoadingI18N}
  <div>Waiting</div>
{:else}
  <div class="text-xl">
    <Link to="/">{sitename}</Link>
    {#if section}
      &gt;
      <Link to={"/" + section}>{$t("section." + section)}</Link>
    {/if}
  </div>
{/if}
