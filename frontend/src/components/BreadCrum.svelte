<script lang="ts">
  import { Link } from "svelte-navigator";
  import { sectionStore } from "../utils/store";
  import { captilizeFirst } from "../utils/util";

  export let dirs: Array<string>;
  export let filename = "";
  export let className = "";

  const elementStyle =
    "flex flex-row max-w-full text-lg leading-tight break-word overflow-x-hidden " +
    className;

  const buildLink = (index: number) => {
    if (index >= dirs.length) {
      return null;
    }

    let targetLink = "/" + $sectionStore;
    for (let i = 0; i <= index; i++) {
      targetLink += "/" + dirs[i];
    }

    return targetLink;
  };
</script>

<div class={elementStyle}>
  <Link
    to={"/" + $sectionStore}
    class="hover:bg-blue-400 hover:text-white px-1 rounded whitespace-nowrap overflow-ellipsis"
    >{captilizeFirst($sectionStore)}</Link
  >
  {#each dirs as dir, i}
    <span>/</span>
    <Link
      to={buildLink(i)}
      class="hover:bg-blue-400 hover:text-white px-1 rounded whitespace-nowrap  overflow-ellipsis"
      >{dir}</Link
    >
  {/each}
  {#if filename}
    <span>/</span>
    <span
      class="px-1 rounded text-gray-500 whitespace-nowrap  overflow-ellipsis"
      >{filename}</span
    >
  {/if}
</div>
