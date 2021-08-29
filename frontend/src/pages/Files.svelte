<script lang="ts">
  import { useLocation } from "svelte-navigator";
  import FilePreview from "./FilePreview.svelte";
  import FileList from "./FileList.svelte";

  const location = useLocation();
  let dir_paths: Array<string> = [];
  let filename: string;

  $: if ($location) parseLocation();

  const parseLocation = () => {
    getFilename();
    getDir();
  };

  const getFilename = () => {
    let query = $location.search;
    if (!query) return;

    let splits = query.split("?preview=").filter((s) => s.length > 0);
    if (splits.length !== 1) return;

    filename = splits[0];
  };

  const getDir = () => {
    let param = $location.pathname;
    if (!param) return;

    let splits = param.split("/").filter((s) => s.length > 0 && s !== "files");
    if (splits.length === 0) return;

    dir_paths = splits;
    // dirsStore.set(dir_paths);
  };
</script>

{#if filename}
  <FilePreview />
{:else}
  <FileList {dir_paths} />
{/if}
