<script lang="ts">
  import { onDestroy } from "svelte";
  import { pwdStore, setNotification } from "../utils/store";
  import FloatButton from "../components/FloatButton.svelte";
  import Icon from "../components/Icon.svelte";
  import type { IFile, IFileOrder } from "../utils/types";
  import * as api from "../utils/api";
  import { formatSize } from "../utils/util";

  let files: IFile[] = [];
  let isLoading = false;
  let pwd = 0;
  let order: IFileOrder = { key: "name", asc: true };

  const unsubscribe = pwdStore.subscribe((value) => {
    if (value >= 0 && pwd !== value) {
      pwd = value;
    }

    pwdStore.set(-1);
  });

  onDestroy(() => {
    unsubscribe();
  });

  $: if (pwd >= 0) {
    fetchFiles();
  }

  $: if (files.length > 0 && order) {
    orderFiles();
  }

  const fetchFiles = async () => {
    try {
      isLoading = true;
      files = await api.get(`/api/file/dir/${pwd}`);
    } catch (e) {
      console.error(e);
      setNotification("error", "Cannot read dir content");
    }
    isLoading = false;
  };

  const changeOrder = (key: "name" | "type" | "size" | "lastModify") => {
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
    let dirs = files.filter((f) => f.is_dir === 1);
    let others = files.filter((f) => f.is_dir === 0);

    dirs.sort(compareFile);
    others.sort(compareFile);

    files = dirs.concat(others);
  };

  const compareFile = (a: IFile, b: IFile) => {
    let ascFactor = order.asc ? 1 : -1;
    let result = 0;

    switch (order.key) {
      case "name":
        const aUpper = a.filename.toUpperCase();
        const bUpper = b.filename.toUpperCase();
        result = aUpper > bUpper ? 1 : aUpper < bUpper ? -1 : 0;
        break;
      case "size":
        result = a.size - b.size;
        break;
      case "lastModify":
        const aTime = Date.parse(a.updated_at);
        const bTime = Date.parse(b.updated_at);
        result = aTime > bTime ? 1 : aTime < bTime ? -1 : 0;
        break;
      case "type":
        break;
      default:
        break;
    }

    return result * ascFactor;
  };
</script>

<div class="relative w-full h-full">
  <FloatButton />
  <div class="w-4/5 h-full mx-auto mt-10">
    <div class="grid grid-cols-5 border-b border-gray-200 py-2 font-bold">
      <div class="col-span-2 px-2 flex flex-row items-center">
        <span
          class="cursor-pointer hover:text-gray-400"
          on:click={() => changeOrder("name")}>Filename</span
        >
        {#if order.key === "name" && order.asc}
          <Icon type="up" size="tiny" color="gray" className="ml-2" />
        {:else if order.key === "name"}
          <Icon type="down" size="tiny" color="gray" className="ml-2" />
        {/if}
      </div>
      <div class="px-2  flex flex-row items-center">
        <span class="cursor-pointer hover:text-gray-400">Type</span>
      </div>
      <div class="px-2  flex flex-row items-center">
        <span
          class="cursor-pointer hover:text-gray-400"
          on:click={() => changeOrder("lastModify")}>Last Modified</span
        >
        {#if order.key === "lastModify" && order.asc}
          <Icon type="up" size="tiny" color="gray" className="ml-2" />
        {:else if order.key === "lastModify"}
          <Icon type="down" size="tiny" color="gray" className="ml-2" />
        {/if}
      </div>
      <div class="px-2  flex flex-row items-center">
        <span
          class="cursor-pointer hover:text-gray-400"
          on:click={() => changeOrder("size")}
          >Size
        </span>
        {#if order.key === "size" && order.asc}
          <Icon type="up" size="tiny" color="gray" className="ml-2" />
        {:else if order.key === "size"}
          <Icon type="down" size="tiny" color="gray" className="ml-2" />
        {/if}
      </div>
    </div>
    {#if pwd > 0}
      <div class="grid grid-cols-5 border-b border-gray-200 py-2">
        <div class="col-span-2 px-2">..</div>
        <div class="px-2" />
        <div class="px-2">2021-08-16 14:39</div>
        <div class="px-2" />
      </div>
    {/if}
    {#each files as file}
      <div class="grid grid-cols-5 border-b border-gray-200 py-2">
        <div class="col-span-2 px-2">{file.filename}</div>
        <div class="px-2">Text</div>
        <div class="px-2">{file.updated_at}</div>
        <div class="px-2">{formatSize(file.size)}</div>
      </div>
    {/each}
  </div>
</div>
