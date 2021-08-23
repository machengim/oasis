<script lang="ts">
  import { onMount } from "svelte";

  export let mouseEvent: MouseEvent;
  export let onAction: (action: "rename" | "delete" | "unselect") => void;
  let item: HTMLElement;
  let isFirstRun = true;

  onMount(() => {
    initMenu();
    isFirstRun = false;
  });

  $: if (mouseEvent && !isFirstRun) {
    initMenu();
  }

  const stopEvent = (e: Event) => {
    e.stopPropagation();
  };

  const initMenu = () => {
    let x = mouseEvent.clientX;
    let y = mouseEvent.clientY;

    item.style.top = `${y}px`;
    item.style.left = `${x}px`;
  };
</script>

<div
  id="context_menu"
  bind:this={item}
  on:click={stopEvent}
  class="fixed z-40 w-40 bg-gray-500 rounded text-white text-lg py-2 px-2 cursor-pointer shadow"
>
  <div class="px-2 hover:bg-blue-400" on:click={() => onAction("rename")}>
    Rename
  </div>
  <div class="px-2 hover:bg-blue-400" on:click={() => onAction("delete")}>
    Delete
  </div>
  <div class="px-2 hover:bg-blue-400" on:click={() => onAction("unselect")}>
    Unselect
  </div>
</div>
