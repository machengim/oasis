<script lang="ts">
  import { onDestroy } from "svelte";
  import { pwdStore, setNotification, completeFileStore } from "../utils/store";
  import FloatButton from "../sections/FloatButton.svelte";
  import Icon from "../components/Icon.svelte";
  import type { IFile, IFileOrder } from "../utils/types";
  import * as api from "../utils/api";
  import { formatSize, formatTimestamp } from "../utils/util";

  let files: IFile[] = [];
  let newCompleteFile: IFile;
  let isLoading = false;
  let root = +localStorage.getItem("root_dir");
  let pwd = root;
  let order: IFileOrder = { key: "name", asc: true };
  let selectedFile: IFile = null;

  const unsubscribePwd = pwdStore.subscribe((value) => {
    if (value > 0 && pwd !== value) {
      pwd = value;
    }
  });

  const unsubscribeCompleteFile = completeFileStore.subscribe((value) => {
    if (value) {
      newCompleteFile = value;
      completeFileStore.set(null);
    }
  });

  onDestroy(() => {
    unsubscribePwd();
    unsubscribeCompleteFile();
  });

  $: if (pwd > 0) {
    fetchFiles();
  }

  $: if (files.length > 0 && order) {
    orderFiles();
  }

  $: if (newCompleteFile && newCompleteFile.parent_id === pwd) {
    files = [...files, newCompleteFile];
  }

  const fetchFiles = async () => {
    try {
      isLoading = true;
      files = await api.get(`/api/dir/${pwd}`);
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
    let dirs = files.filter((f) => f.file_type.toUpperCase() === "DIR");
    let others = files.filter((f) => f.file_type.toUpperCase() !== "DIR");

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
        result = a.last_modified_at - b.last_modified_at;
        break;
      case "type":
        result =
          a.file_type > b.file_type ? 1 : a.file_type < b.file_type ? -1 : 0;
        break;
      default:
        break;
    }

    return result * ascFactor;
  };

  const getFileStyle = (file: IFile) => {
    if (file === selectedFile) {
      return "grid grid-cols-5 border-b border-gray-200 py-2 bg-blue-400 text-white";
    } else {
      return "grid grid-cols-5 border-b border-gray-200 py-2";
    }
  };

  const clickFile = (file: IFile) => {
    if (file !== selectedFile) {
      selectedFile = file;
      files = files;
    }
  };

  const rightClickFile = (e: Event, file: IFile) => {
    e.preventDefault();

    clickFile(file);
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
        <span
          class="cursor-pointer hover:text-gray-400"
          on:click={() => changeOrder("type")}>Type</span
        >
        {#if order.key === "type" && order.asc}
          <Icon type="up" size="tiny" color="gray" className="ml-2" />
        {:else if order.key === "type"}
          <Icon type="down" size="tiny" color="gray" className="ml-2" />
        {/if}
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
    {#if pwd !== root}
      <div class="grid grid-cols-5 border-b border-gray-200 py-2">
        <div class="col-span-2 px-2">..</div>
        <div class="px-2" />
        <div class="px-2">2021-08-16 14:39</div>
        <div class="px-2" />
      </div>
    {/if}
    {#each files as file}
      <div
        class={getFileStyle(file)}
        on:contextmenu={(e) => rightClickFile(e, file)}
        on:click={() => clickFile(file)}
      >
        <div class="col-span-2 px-2">{file.filename}</div>
        <div class="px-2">{file.file_type}</div>
        <div class="px-2">{formatTimestamp(file.last_modified_at)}</div>
        <div class="px-2">{formatSize(file.size)}</div>
      </div>
    {/each}
  </div>
</div>
