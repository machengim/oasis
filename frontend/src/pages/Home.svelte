<script lang="ts">
  import { onDestroy } from "svelte";
  import {
    pwdStore,
    setNotification,
    completeFileStore,
    clickEventStore,
    fileActionStore,
  } from "../utils/store";
  import Icon from "../components/Icon.svelte";
  import type { IFile, IFileAction, IFileOrder } from "../utils/types";
  import * as api from "../utils/api";
  import { formatSize, formatTimestamp } from "../utils/util";
  import FloatButton from "../sections/FloatButton.svelte";
  import ContextMenu from "../sections/ContextMenu.svelte";
  import RenameFileModal from "../modals/RenameFileModal.svelte";

  let files: IFile[] = [];
  let newCompleteFile: IFile;
  let isLoading = false;
  let root = +localStorage.getItem("root_dir");
  let pwd = root;
  let order: IFileOrder = { key: "name", asc: true };
  let selectedFile: IFile;
  let isOpenContextMenu = false;
  let isOpenRenameModal = false;
  let mouseEvent: MouseEvent = null;
  let lastGlobalClickTime = 0;

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

  const unsubscribeClickEvent = clickEventStore.subscribe((value) => {
    if (value > 0) {
      lastGlobalClickTime = value;
    }
  });

  const unsubscribeFileAction = fileActionStore.subscribe((value) => {
    if (value && value.file.parent_id === pwd) {
      handleFileAction(value);
    }
  });

  onDestroy(() => {
    unsubscribePwd();
    unsubscribeCompleteFile();
    unsubscribeClickEvent();
    unsubscribeFileAction();
  });

  $: if (files) {
    console.log(files);
  }

  $: if (pwd > 0) {
    fetchFiles();
  }

  $: if (files.length > 0 && order) {
    orderFiles();
  }

  $: if (newCompleteFile && newCompleteFile.parent_id === pwd) {
    files = [...files, newCompleteFile];
    newCompleteFile = null;
  }

  $: if (lastGlobalClickTime > 0) {
    if (isOpenContextMenu) {
      isOpenContextMenu = false;
    }

    lastGlobalClickTime = 0;
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

  const handleFileAction = (fileAction: IFileAction) => {
    switch (fileAction.action) {
      case "modify":
        let id = files.findIndex(
          (file) => file.file_id === fileAction.file.file_id
        );
        if (id >= 0) {
          files[id] = fileAction.file;
          files = files;
        }
        break;

      default:
        break;
    }
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
    if (selectedFile && file.file_id === selectedFile.file_id) {
      return "grid grid-cols-5 border-b border-gray-200 py-2 bg-blue-400 text-white";
    } else {
      return "grid grid-cols-5 border-b border-gray-200 py-2";
    }
  };

  const clickFile = (file: IFile) => {
    if (!selectedFile || file.file_id !== selectedFile.file_id) {
      selectedFile = file;
      files = files;
    }
  };

  const rightClickFile = (e: MouseEvent, file: IFile) => {
    e.preventDefault();

    clickFile(file);
    mouseEvent = e;
    isOpenContextMenu = true;
  };

  const closeContextMenu = () => {
    isOpenContextMenu = false;
  };

  const closeRenameModal = () => {
    isOpenRenameModal = false;
  };

  const sendDeleteFileRequest = async () => {
    let endpoint = `/api/file/${selectedFile.file_id}`;

    try {
      await api.remove(endpoint, null, false);
      removeFileFromList(selectedFile);
    } catch (e) {
      console.error(e);
      setNotification("error", `Remove file ${selectedFile.filename} failed`);
    }
  };

  const removeFileFromList = (file: IFile) => {
    if (file.parent_id === pwd) {
      files = files.filter((f) => f.file_id !== file.file_id);
    }
  };

  const onContextMenuAction = (action: "rename" | "delete") => {
    switch (action) {
      case "rename":
        isOpenRenameModal = true;
        break;
      case "delete":
        let confirm = window.confirm(
          `Are you sure you want to delete ${selectedFile.filename} ?`
        );
        if (confirm) {
          sendDeleteFileRequest();
        }
        break;
      default:
        break;
    }

    isOpenContextMenu = false;
  };
</script>

<div class="relative w-full h-full">
  <FloatButton />
  {#if isOpenContextMenu}
    <ContextMenu {mouseEvent} onAction={onContextMenuAction} />
  {/if}
  {#if isOpenRenameModal}
    <RenameFileModal {selectedFile} onClose={closeRenameModal} />
  {/if}
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
