<script lang="ts">
  import { useNavigate } from "svelte-navigator";
  import { t } from "svelte-i18n";
  import {
    setNotification,
    dirsStore,
    filesStore,
    titleStore,
  } from "../utils/store";
  import type { IFile, IFileOrder } from "../utils/types";
  import { EIconType } from "../utils/types";
  import * as api from "../utils/api";
  import Icon from "../components/Icon.svelte";
  import Spinner from "../components/Spinner.svelte";
  import BreadCrum from "../components/BreadCrum.svelte";
  import { formatSize, compareFile } from "../utils/util";
  import FileIcon from "../components/FileIcon.svelte";

  const navigate = useNavigate();
  export let dirs: Array<string>;
  let files: Array<IFile> = [];
  let order: IFileOrder = { key: "name", asc: true };
  let isLoading = false;

  $: if (dirs.length >= 1) {
    titleStore.set(dirs[dirs.length - 1]);
  }

  $: fetchDirContent(dirs);

  $: if (files.length > 0 && order) {
    orderFiles();
  }

  const fetchDirContent = async (dirs: Array<string>) => {
    let endpoint = "/api/dir";
    if (dirs.length > 0) {
      endpoint += "?path=" + encodeURIComponent(dirs.join("/"));
    }

    isLoading = true;
    try {
      files = await api.get(endpoint);
      dirsStore.set(dirs);
      filesStore.set(files);
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_dir_error"));
    }

    isLoading = false;
  };

  const changeOrder = (key: "name" | "type" | "size") => {
    let newOrder = Object.assign({}, order);
    if (key === order.key) {
      newOrder.asc = !order.asc;
    } else {
      newOrder.key = key;
      newOrder.asc = true;
    }

    order = newOrder;
  };

  const orderFiles = () => {
    let dirs = files.filter((f) => f.file_type.toUpperCase() === "DIR");
    let others = files.filter((f) => f.file_type.toUpperCase() !== "DIR");

    dirs.sort((a, b) => compareFile(a, b, order));
    others.sort((a, b) => compareFile(a, b, order));

    files = dirs.concat(others);
  };

  const selectFile = (file: IFile) => {
    let path: string;
    if (file.file_type.toLowerCase() === "dir") {
      path = dirs.join("/") + "/" + file.filename;
    } else {
      path = dirs.join("/") + "?view=" + file.filename;
    }

    navigateFile(path);
  };

  const backToParentDir = () => {
    const path = dirs.slice(0, -1).join("/");
    navigateFile(path);
  };

  const navigateFile = (path: string) => {
    const pathPrefix = path.startsWith("/") ? path : "/" + path;
    const targetPath = "/files" + pathPrefix;

    navigate(targetPath);
  };
</script>

<div class="relative w-full h-full">
  <div class="w-11/12 lg:w-4/5 h-full mx-auto my-4 lg:mt-4 lg:mb-10">
    <BreadCrum {dirs} className="py-1" />
    {#if isLoading}
      <Spinner />
    {:else}
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
        <div class="px-2  flex flex-row items-center">
          <span
            class="cursor-pointer hover:text-gray-400"
            on:click={() => changeOrder("type")}
            >{$t("component.dir_list.type")}</span
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
      {#if dirs && dirs.length > 0}
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
    {/if}
  </div>
</div>
