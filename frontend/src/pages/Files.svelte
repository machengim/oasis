<script lang="ts">
  import { onDestroy } from "svelte";
  import { useLocation, useNavigate } from "svelte-navigator";
  import {
    dirsStore,
    resetTitle,
    sectionStore,
    userStore,
  } from "../utils/store";
  import { compareArray } from "../utils/util";
  import DirList from "./DirList.svelte";
  import FileView from "./FileView.svelte";

  const navigate = useNavigate();
  const location = useLocation();
  let dirs: Array<string> = [];
  let filename: string;
  let user = $userStore;

  const unsubscribeUser = userStore.subscribe((u) => (user = u));

  $: if (!user) {
    navigate("/login");
  }

  onDestroy(() => {
    unsubscribeUser();
  });

  sectionStore.set("files");
  resetTitle();

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

    filename = decodeURIComponent(splits[0]);
  };

  const getDir = () => {
    let param = $location.pathname;
    if (!param) return;

    dirs = param
      .split("/")
      .filter((s) => s.length > 0)
      .map((s) => decodeURIComponent(s))
      .slice(1);

    if (!compareArray(dirs, $dirsStore)) {
      dirsStore.set(dirs);
    }
  };
</script>

{#if filename}
  <FileView {filename} />
{:else}
  <DirList />
{/if}
