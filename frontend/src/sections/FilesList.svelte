<script lang="ts">
  // Name it `FilesList` instead of `FileList` to avoid conflicts with the ts built-in type.
  import { t } from "svelte-i18n";
  import type { IFile, IFileOrder } from "../utils/types";
  import { EIconType } from "../utils/enums";
  import Icon from "../components/Icon.svelte";
  import FileIcon from "../components/FileIcon.svelte";
  import { formatSize } from "../utils/util";

  export let page: "dir" | "search" = "dir";
  export let dirs: Array<string> = null;
  export let files: Array<IFile>;
  export let order: IFileOrder;
  export let selectFile: (file: IFile) => void;
  export let changeOrder = (key: string) => {};
  export let backToParentDir = () => {};
  export let openContextMenu = (e: Event, file: IFile) => {};

  const onOpenContextMenu = (e: Event, file: IFile) => {
    if (page !== "dir") return;

    openContextMenu(e, file);
  };
</script>

<div class="grid grid-cols-5 border-b border-gray-200 py-2 font-bold">
  <div class="col-span-3 px-2 flex flex-row items-center">
    <span
      class="cursor-pointer hover:text-gray-400"
      on:click={() => changeOrder("name")}
      >{$t("component.dir_list.filename")}</span
    >
    {#if order.key === "name" && order.asc}
      <Icon type={EIconType.up} size="tiny" className="ml-2" />
    {:else if order.key === "name"}
      <Icon type={EIconType.down} size="tiny" className="ml-2" />
    {/if}
  </div>
  <div class="px-2 flex flex-row items-center">
    <span
      class="cursor-pointer hover:text-gray-400"
      on:click={() => changeOrder("type")}>{$t("component.dir_list.type")}</span
    >
    {#if order.key === "type" && order.asc}
      <Icon type={EIconType.up} size="tiny" className="ml-2" />
    {:else if order.key === "type"}
      <Icon type={EIconType.down} size="tiny" className="ml-2" />
    {/if}
  </div>
  <div class="px-2  flex flex-row items-center">
    <span
      class="cursor-pointer hover:text-gray-400"
      on:click={() => changeOrder("size")}
      >{$t("component.dir_list.size")}
    </span>
    {#if order.key === "size" && order.asc}
      <Icon type={EIconType.up} size="tiny" className="ml-2" />
    {:else if order.key === "size"}
      <Icon type={EIconType.down} size="tiny" className="ml-2" />
    {/if}
  </div>
</div>
{#if page === "dir" && dirs && dirs.length > 0}
  <div
    class="grid grid-cols-5 border-b border-gray-200 py-2 hover:bg-gray-200 cursor-pointer"
    on:click={backToParentDir}
  >
    <div class="col-span-3 px-2">..</div>
    <div class="px-2" />
    <div class="px-2" />
    <div class="px-2" />
  </div>
{/if}
{#each files as file}
  <div
    class="grid grid-cols-5 border-b border-gray-200 py-2 hover:bg-gray-200 cursor-pointer"
    on:contextmenu|preventDefault={(e) => onOpenContextMenu(e, file)}
    on:click={() => selectFile(file)}
  >
    <div class="col-span-3 px-2 flex flex-row items-center">
      <FileIcon {file} />
      <span class="ml-2 break-all">
        {file.filename}
      </span>
    </div>
    <div class="px-2 my-auto">
      {$t("filetype." + file.file_type)}
    </div>
    <div class="px-2 my-auto">{formatSize(file.size)}</div>
  </div>
{/each}
