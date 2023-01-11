<script lang="ts">
  import { useNavigate } from "svelte-navigator";
  import { t } from "svelte-i18n";
  import {
    setNotification,
    dirsStore,
    filesStore,
    titleStore,
    uploadTaskStore,
    resetTitle,
    clickStore,
    userStore,
    siteStore,
  } from "../utils/store";
  import type {
    ContextMenuAction,
    IFile,
    IFileOrder,
    IMousePosition,
    IUploadTask,
  } from "../utils/types";
  import { EUploadStatus } from "../utils/enums";
  import * as api from "../utils/api";
  import Spinner from "../components/Spinner.svelte";
  import BreadCrum from "../components/BreadCrum.svelte";
  import { buildEncodeFilePath, compareFile, isFile } from "../utils/util";
  import Button from "../components/Button.svelte";
  import PromptModal from "../modals/PromptModal.svelte";
  import NewFilenameModal from "../modals/NewFilenameModal.svelte";
  import DeleteFileModal from "../modals/DeleteFileModal.svelte";
  import FileVisibilityModal from "../modals/FileVisibilityModal.svelte";
  import ContextMenu from "../sections/ContextMenu.svelte";
  import { onDestroy, onMount } from "svelte";
  import FilesList from "../sections/FilesList.svelte";
  import DirBrowser from "../sections/DirBrowser.svelte";
  import CopyMoveFileModal from "../modals/CopyMoveFileModal.svelte";

  const navigate = useNavigate();
  const user = $userStore;
  let dirs = $dirsStore;
  let files: Array<IFile> = $filesStore;
  let storage = $siteStore.storage;
  let order: IFileOrder = { key: "name", asc: true };
  let isLoading = false;
  let fileSelector: HTMLInputElement;
  let title: string;
  let text: string;
  let result = false;
  let resultForAll = false;
  let showPromptModal = false;
  let showNewMenu = false;
  let showNewFilenameModal = false;
  let showContextMenu = false;
  let showFileVisibilityModal = false;
  let showDeleteFileModal = false;
  let showDirBrowser = false;
  let showCopyFileModal = false;
  let contextPos: IMousePosition;
  let contextFile: IFile;
  let targetDir: string;
  let copyOrMove: "copy" | "move" = "copy";

  const unsubscribeDirs = dirsStore.subscribe((d) => (dirs = d));

  const unsubscribeFiles = filesStore.subscribe((f) => (files = f));

  const unsubscribeClick = clickStore.subscribe((click) => {
    if (click > 0) {
      if (showNewMenu) {
        showNewMenu = false;
      }
      if (showContextMenu) {
        showContextMenu = false;
        contextFile = null;
      }
    }
  });

  const unsubscribeStorage = siteStore.subscribe(
    (site) => (storage = site.storage)
  );

  onDestroy(() => {
    unsubscribeDirs();
    unsubscribeFiles();
    unsubscribeClick();
    unsubscribeStorage();
  });

  $: if (dirs.length >= 1) {
    titleStore.set(dirs[dirs.length - 1]);
  } else {
    resetTitle();
  }

  $: fetchDirContent(dirs);

  $: if (files.length > 0 && order) {
    orderFiles();
  }

  const fetchDirContent = async (targetDirs: Array<string>) => {
    isLoading = true;

    let endpoint = "/api/dir";
    if (targetDirs.length > 0) {
      endpoint += "?path=" + encodeURIComponent(targetDirs.join("/"));
    }

    try {
      const newFiles: Array<IFile> = await api.get(endpoint);
      if (dirs === targetDirs) {
        filesStore.set(newFiles);
      }
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

    await uploadFiles(filelist);
  };

  const uploadFiles = async (filelist: FileList) => {
    for (const file of filelist) {
      try {
        await isFile(file);
      } catch (e) {
        console.log("Ignore folder upload: ", file.name);
        continue;
      }

      if (
        !resultForAll &&
        files.findIndex((f) => f.filename === file.name) >= 0
      ) {
        title = "Filename existed";
        text = `File <b>${file.name}</b> already existed. Are you sure you want to overwrite it?`;
        result = false;
        showPromptModal = true;

        while (showPromptModal) {
          await new Promise((r) => setTimeout(r, 200));
        }

        if (!result && !resultForAll) {
          continue;
        }
      }

      const upload: IUploadTask = {
        file,
        targetDir: dirs,
        status: EUploadStatus.waiting,
        progress: 0,
      };

      const tasks = $uploadTaskStore;
      tasks.push(upload);
      uploadTaskStore.set(tasks);
    }

    resultForAll = false;
  };

  const setResult = (r: boolean) => {
    result = r;
  };

  const setResultAll = (r: boolean) => {
    resultForAll = r;
  };

  const closePrompModal = () => {
    showPromptModal = false;
  };

  const openNewFilenameModal = () => {
    showNewFilenameModal = true;
  };

  const closeNewFilenameModal = () => {
    showNewFilenameModal = false;
    contextFile = null;
  };

  const closeFileVisibilityModal = () => {
    showFileVisibilityModal = false;
    contextFile = null;
  };

  const toggleShowNewMenu = (e: Event) => {
    e.stopPropagation();
    showNewMenu = !showNewMenu;
  };

  const openContextMenu = (e: MouseEvent, file: IFile) => {
    contextFile = file;
    contextPos = { x: e.pageX, y: e.pageY };
    showContextMenu = true;
  };

  const onDownload = (contextFile: IFile) => {
    if (contextFile.file_type === "Dir") {
      // TODO: download dir
    } else {
      const filePath =
        "/api/file/" + buildEncodeFilePath(dirs, contextFile.filename);
      const link = document.createElement("a");
      link.download = contextFile.filename;
      link.href = filePath;
      link.click();
    }
  };

  const onContextAction = (action: ContextMenuAction) => {
    showContextMenu = false;

    switch (action) {
      case "rename":
        showNewFilenameModal = true;
        break;
      case "delete":
        showDeleteFileModal = true;
        break;
      case "visibility":
        showFileVisibilityModal = true;
        break;
      case "close":
        contextFile = null;
        break;
      case "copy":
        copyOrMove = "copy";
        showDirBrowser = true;
        break;
      case "move":
        copyOrMove = "move";
        showDirBrowser = true;
        break;
      case "download":
        onDownload(contextFile);
        break;
    }
  };

  const closeDeleteFileModal = () => {
    showDeleteFileModal = false;
    contextFile = null;
  };
</script>

{#if showContextMenu}
  <ContextMenu pos={contextPos} {onContextAction} {contextFile} />
{/if}
{#if showDirBrowser && storage}
  <DirBrowser
    onClose={() => (showDirBrowser = false)}
    onSelect={(v) => {
      targetDir = v.replace(storage, "");
      showCopyFileModal = true;
    }}
    root={storage}
  />
{/if}
{#if showPromptModal}
  <PromptModal
    {title}
    {text}
    onClose={closePrompModal}
    setResult={(r) => setResult(r)}
    hasExtraResult={true}
    setExtraResult={(r) => setResultAll(r)}
  />
{/if}
{#if showNewFilenameModal}
  <NewFilenameModal
    {dirs}
    {files}
    {contextFile}
    onClose={closeNewFilenameModal}
  />
{/if}
{#if showDeleteFileModal}
  <DeleteFileModal {dirs} {contextFile} onClose={closeDeleteFileModal} />
{/if}
{#if showFileVisibilityModal}
  <FileVisibilityModal
    {dirs}
    {contextFile}
    onClose={closeFileVisibilityModal}
  />
{/if}
{#if showCopyFileModal}
  <CopyMoveFileModal
    source={contextFile}
    sourceDirs={dirs}
    target={targetDir}
    mode={copyOrMove}
    onClose={() => (showCopyFileModal = false)}
  />
{/if}

<div class="relative w-full h-full">
  <div class="w-11/12 lg:w-4/5 h-full mx-auto my-4 lg:mt-4 lg:mb-10">
    <div class="flex flex-row items-center justify-between">
      <BreadCrum {dirs} className="py-1" />
      {#if user && user.permission > 0}
        <div class="relative flex flex-row justify-end">
          <Button
            onClick={toggleShowNewMenu}
            value={$t("component.dir_list.new")}
            color="blue"
          />
          <input
            type="file"
            class="hidden"
            bind:this={fileSelector}
            multiple
            on:change={selectUploadFile}
          />
          {#if showNewMenu}
            <div
              class="absolute w-32 top-9 right-0 py-1 shadow-sm rounded-sm bg-white border"
            >
              <div
                class="px-2 py-1 hover:bg-gray-400 hover:text-white cursor-pointer text-center"
                on:click={openNewFilenameModal}
              >
                {$t("component.dir_list.create_folder")}
              </div>
              <div
                class="px-2 py-1 hover:bg-gray-400 hover:text-white cursor-pointer text-center"
                on:click={openSelectFileDialog}
              >
                {$t("component.dir_list.upload_files")}
              </div>
            </div>
          {/if}
        </div>
      {/if}
    </div>
    {#if isLoading}
      <Spinner />
    {:else}
      <FilesList
        {dirs}
        {files}
        {order}
        {selectFile}
        {changeOrder}
        {backToParentDir}
        {openContextMenu}
        {uploadFiles}
      />
    {/if}
  </div>
</div>
