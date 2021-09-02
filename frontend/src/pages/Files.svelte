<script lang="ts">
  import { useLocation } from "svelte-navigator";
  import { sectionStore } from "../utils/store";
  import DirList from "./DirList.svelte";
  import FileView from "./FileView.svelte";

  const location = useLocation();
  let dirs: Array<string> = [];
  let filename: string;

  sectionStore.set("files");
  $: if ($location) parseLocation();

  const parseLocation = () => {
    filename = null;
    getFilename();
    getDir();
  };

  const getFilename = () => {
    let query = $location.search;
    if (!query) return;

    let splits = query.split("?view=").filter((s) => s.length > 0);
    if (splits.length !== 1) return;

    filename = splits[0];
  };

  const getDir = () => {
    let param = $location.pathname;
    if (!param) return;

    dirs = param
      .split("/")
      .filter((s) => s.length > 0)
      .slice(1);
  };
</script>

{#if filename}
  <FileView {dirs} {filename} />
{:else}
  <DirList {dirs} />
{/if}
