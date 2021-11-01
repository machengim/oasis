<script lang="ts">
  import { useNavigate } from "svelte-navigator";
  import { t } from "svelte-i18n";
  import {
    setNotification,
    dirsStore,
    filesStore,
    titleStore,
    uploadTaskStore,
    completeTaskStore,
  } from "../utils/store";
  import type { IFile, IFileOrder, IUploadTask } from "../utils/types";
  import { EUploadStatus } from "../utils/types";
  import { EIconType } from "../utils/types";
  import * as api from "../utils/api";
  import Icon from "../components/Icon.svelte";
  import Spinner from "../components/Spinner.svelte";
  import BreadCrum from "../components/BreadCrum.svelte";
  import { formatSize, compareFile, inferFileType } from "../utils/util";
  import FileIcon from "../components/FileIcon.svelte";
  import Button from "../components/Button.svelte";
  import PromptModal from "../modals/PromptModal.svelte";
  import { onDestroy } from "svelte";

  const navigate = useNavigate();
  export let dirs: Array<string>;
  let files: Array<IFile> = [];
  let order: IFileOrder = { key: "name", asc: true };
  let isLoading = false;
  let fileSelector: HTMLInputElement;
  let title: string;
  let text: string;
  let result = false;
  let showPromptModal = false;

  const unsubscribeTaskUpdate = completeTaskStore.subscribe((task) => {
    if (task && task.status === EUploadStatus.success) {
      let encodedDir = encodeURIComponent(dirs.join("/"));
      if (encodedDir === task.targetDir) {
        const file = task.file;
        const fileEntry: IFile = {
          file_type: inferFileType(file.name),
          filename: file.name,
          size: file.size,
        };
        files.push(fileEntry);
        files = files;
      }
    }
  });

  onDestroy(() => {
    unsubscribeTaskUpdate();
  });

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

  const openSelectFileDialog = () => {
    if (fileSelector) {
      fileSelector.click();
    }
  };

  const selectUploadFile = async (e: Event) => {
    const target = e.target as HTMLInputElement;
    const filelist = target.files as FileList;
    const targetDir = encodeURIComponent(dirs.join("/"));

    for (const file of filelist) {
      if (files.findIndex((f) => f.filename === file.name) >= 0) {
        title = "Filename existed";
        text = `File <b>${file.name}</b> already existed. Are you sure you want to overwrite it?`;
        result = false;
        showPromptModal = true;

        while (showPromptModal) {
          await new Promise((r) => setTimeout(r, 200));
        }

        if (!result) {
          continue;
        }
      }

      const upload: IUploadTask = {
        file,
        targetDir,
        status: EUploadStatus.waiting,
        progress: 0,
      };

      uploadTaskStore.set(upload);
    }
  };

  const setResult = (r: boolean) => {
    result = r;
  };

  const closePrompModal = () => {
    showPromptModal = false;
  };
</script>

{#if showPromptModal}
  <PromptModal
    {title}
    {text}
    onClose={closePrompModal}
    setResult={(r) => setResult(r)}
  />
{/if}
<div class="relative w-full h-full">
  <div class="w-11/12 lg:w-4/5 h-full mx-auto my-4 lg:mt-4 lg:mb-10">
    <div class="flex flex-row items-center justify-between">
      <BreadCrum {dirs} className="py-1" />
      <div>
        <Button onClick={openSelectFileDialog} color="blue" value="+ Upload" />
        <input
          type="file"
          class="hidden"
          bind:this={fileSelector}
          multiple
          on:change={selectUploadFile}
        />
      </div>
    </div>
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
