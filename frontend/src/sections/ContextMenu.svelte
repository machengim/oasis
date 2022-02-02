<script lang="ts">
  import { t } from "svelte-i18n";
  import { onMount } from "svelte";
  import { userStore } from "../utils/store";
  import type {
    ContextMenuAction,
    IFile,
    IMousePosition,
  } from "../utils/types";

  export let onContextAction: (action: ContextMenuAction) => void;
  export let pos: IMousePosition;
  export let contextFile: IFile;
  const user = $userStore;
  let menu: HTMLDivElement;

  $: if (pos && menu) {
    setPosition();
  }

  onMount(() => {
    setPosition();
  });

  const setPosition = () => {
    if (menu) {
      menu.style.position = "absolute";
      menu.style.top = pos.y + "px";
      menu.style.left = pos.x + "px";
    }
  };
</script>

{#if user.permission > 0}
  <div
    class="z-20 w-32 py-1 bg-white border rounded-sm shadow-sm "
    bind:this={menu}
    on:click|stopPropagation
  >
    <div
      class="px-2 py-1 text-black hover:bg-gray-400 cursor-pointer"
      on:click={() => onContextAction("visibility")}
    >
      {contextFile?.least_permission > 0
        ? $t("component.context_menu.show")
        : $t("component.context_menu.hide")}
    </div>
    <div
      class="px-2 py-1 text-black hover:bg-gray-400 cursor-pointer"
      on:click={() => onContextAction("rename")}
    >
      {$t("component.context_menu.rename")}
    </div>

    <div
      class="px-2 py-1 text-black hover:bg-gray-400 cursor-pointer"
      on:click={() => onContextAction("copy")}
    >
      {$t("component.context_menu.copy_to")}
    </div>
    <div
      class="px-2 py-1 text-black hover:bg-gray-400 cursor-pointer"
      on:click={() => onContextAction("move")}
    >
      {$t("component.context_menu.move_to")}
    </div>
    <div
      class="px-2 py-1 text-red-500 hover:bg-gray-400 cursor-pointer"
      on:click={() => onContextAction("delete")}
    >
      {$t("component.context_menu.delete")}
    </div>

    <hr />
    <div
      class="px-2 py-1 text-black hover:bg-gray-400 cursor-pointer"
      on:click={() => onContextAction("close")}
    >
      {$t("component.context_menu.close")}
    </div>
  </div>
{/if}
