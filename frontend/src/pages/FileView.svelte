<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { t } from "svelte-i18n";
  import { useNavigate } from "svelte-navigator";
  import {
    dirsStore,
    filesStore,
    setNotification,
    loopStore,
    titleStore,
  } from "../utils/store";
  import { EIconColor, ELoopMethod, FileType, EIconType } from "../utils/types";
  import type { IFile, IFileOrder, ILoopIcon } from "../utils/types";
  import Spinner from "../components/Spinner.svelte";
  import BreadCrum from "../components/BreadCrum.svelte";
  import MediaPlayer from "../players/MediaPlayer.svelte";
  import { inferFileType, compareArray, compareFile } from "../utils/util";
  import * as api from "../utils/api";
  import TextViewer from "../players/TextViewer.svelte";
  import ImageViewer from "../players/ImageViewer.svelte";
  import PdfViewer from "../players/PdfViewer.svelte";
  import Icon from "../components/Icon.svelte";
  import FileLinkModal from "../modals/FileLinkModal.svelte";

  const navigate = useNavigate();
  export let dirs: Array<string>;
  export let filename: string;
  let filePath: string;
  let trackPath: string;
  let siblings: Array<IFile>;
  let filesInStore: Array<IFile> = [];
  let isLoading = false;
  let fileType: FileType;
  let loopIcons: Array<ILoopIcon> = [];
  let showFileLinkModal = false;
  // Fix compareDir bug when at the root dir.
  let rootFetched = false;

  const unsubscribeFiles = filesStore.subscribe((files) => {
    filesInStore = files;
  });

  onMount(() => {
    initLoopIcons();
  });

  onDestroy(() => {
    unsubscribeFiles();
  });

  $: if (filename) {
    titleStore.set(filename);
    fileType = extractFileType();
    filePath = buildFilePath();
  }

  $: if (filename && fileType === FileType.Video && filesInStore.length > 0) {
    trackPath = buildTrackPath();
  }

  $: if (fileType && filesInStore) {
    if (dirs.length === 0 && $dirsStore.length === 0 && !rootFetched) {
      fetchDirContent(dirs);
      rootFetched = true;
    } else if (compareArray(dirs, $dirsStore)) {
      const order: IFileOrder = { key: "name", asc: true };
      const siblingFiles = $filesStore.filter((f) => f.file_type === fileType);
      siblingFiles.sort((a, b) => compareFile(a, b, order));
      siblings = siblingFiles;
    } else {
      fetchDirContent(dirs);
    }
  }

  const initLoopIcons = () => {
    const icons = [
      { type: ELoopMethod.repeat, selected: false },
      { type: ELoopMethod.shuffle, selected: false },
      { type: ELoopMethod.loop, selected: false },
    ];

    if ($loopStore) {
      const icon = icons.find((icon) => icon.type === $loopStore);
      icon.selected = true;
    }

    loopIcons = icons;
  };

  const fetchDirContent = async (dirs: Array<string>) => {
    let endpoint = "/api/dir";
    if (dirs.length > 0) {
      endpoint += "?path=" + encodeURIComponent(dirs.join("/"));
    }

    isLoading = true;
    try {
      let files: Array<IFile> = await api.get(endpoint);
      dirsStore.set(dirs);
      filesStore.set(files);
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_dir_error"));
    }

    isLoading = false;
  };

  const buildFilePath = () => {
    const dir = dirs.join("/");
    const path = dir ? dir + "/" + filename : filename;

    return "/api/file/" + encodeURIComponent(path);
  };

  const buildTrackPath = () => {
    const splits = filename.split(".");
    if (splits.length <= 1) return null;

    splits.pop();
    const vttTrackName = splits.join(".") + ".vtt";
    const srtTrackName = splits.join(".") + ".srt";
    const findTrack = filesInStore.find(
      (file) => file.filename === vttTrackName || file.filename === srtTrackName
    );
    if (!findTrack) return null;
    const dir = dirs.join("/");
    const filePath = dir ? dir + "/" + vttTrackName : vttTrackName;

    return "/api/file/track/" + encodeURIComponent(filePath);
  };

  const extractFileType = () => {
    const splits = filename.split(".");
    let file_ext: string;

    if (splits.length < 2) {
      file_ext = null;
    } else {
      file_ext = splits.slice(-1)[0].toLowerCase();
    }

    return inferFileType(file_ext);
  };

  const selectSibling = (index: number) => {
    const file = siblings[index];
    const path = dirs.join("/") + "?view=" + file.filename;

    navigateFile(path);
  };

  const navigateFile = (path: string) => {
    const pathPrefix = path.startsWith("/") ? path : "/" + path;
    const targetPath = "/files" + pathPrefix;

    navigate(targetPath);
  };

  const onMediaComplete = () => {
    const loop = $loopStore;
    if (!loop || loop === ELoopMethod.repeat) return;

    moveNext();
  };

  const moveNext = () => {
    const currentIndex = siblings.findIndex((s) => s.filename === filename);
    const size = siblings.length;
    let nextIndex: number;

    if ($loopStore === ELoopMethod.shuffle) {
      nextIndex = getRandom(size, currentIndex);
    } else {
      nextIndex = (currentIndex + 1) % size;
    }

    selectSibling(nextIndex);
  };

  const moveBack = () => {
    let currentIndex = siblings.findIndex((s) => s.filename === filename);
    if (currentIndex - 1 >= 0) {
      selectSibling(currentIndex - 1);
    }
  };

  const getRandom = (size: number, exclude: number) => {
    let random: number;

    do {
      random = Math.floor(Math.random() * size);
    } while (random === exclude);

    return random;
  };

  const getIconType = (loopIcon: ILoopIcon) => {
    switch (loopIcon.type) {
      case ELoopMethod.repeat:
        return EIconType.repeat;
      case ELoopMethod.shuffle:
        return EIconType.shuffle;
      case ELoopMethod.loop:
        return EIconType.loop;
      default:
        return null;
    }
  };

  const getIconColor = (loopIcon: ILoopIcon) => {
    return loopIcon.selected ? EIconColor.black : EIconColor.gray;
  };

  const selectLoopMethod = (index: number) => {
    const currentIndex = loopIcons.findIndex((i) => i.selected);
    if (currentIndex >= 0) loopIcons[currentIndex].selected = false;
    if (index === currentIndex) {
      loopStore.set(null);
    } else {
      loopIcons[index].selected = true;
      loopStore.set(loopIcons[index].type);
    }

    loopIcons = loopIcons;
  };

  const openFileLinkModal = () => {
    showFileLinkModal = true;
  };

  const closeFileLinkModal = () => {
    showFileLinkModal = false;
  };
</script>

<div class="relative w-full h-full">
  <div class="w-11/12 lg:w-4/5 h-full mx-auto my-4 lg:mt-4 lg:mb-10">
    {#if showFileLinkModal}
      <FileLinkModal {filename} {filePath} onClose={closeFileLinkModal} />
    {/if}
    <div class="flex flex-row items-center">
      <BreadCrum {dirs} {filename} className="py-1 mr-2" />
      <Icon
        type={EIconType.link}
        size="small"
        color={EIconColor.black}
        className="transform -rotate-45 cursor-pointer"
        onClick={openFileLinkModal}
      />
    </div>
    {#if isLoading}
      <Spinner />
    {:else}
      <div class="flex flex-row flex-wrap mt-4">
        <div class="w-full lg:w-3/4">
          {#if fileType === FileType.Video || fileType === FileType.Music}
            <MediaPlayer
              {filePath}
              {fileType}
              {trackPath}
              onComplete={onMediaComplete}
            />
          {:else if fileType === FileType.Text || fileType === FileType.Code}
            <TextViewer {dirs} {filename} {fileType} />
          {:else if fileType === FileType.Pdf}
            <PdfViewer {filePath} />
          {:else if fileType === FileType.Image}
            <ImageViewer
              {filePath}
              {filename}
              {onMediaComplete}
              {moveBack}
              {moveNext}
            />
          {:else}
            <div class="text-lg my-4">Cannot display this file.</div>
          {/if}
        </div>
        <div class="flex flex-col w-full lg:w-1/4 lg:pl-8 mt-4 lg:mt-0">
          <div class="bg-gray-100 rounded-lg">
            <div
              class="flex flex-row items-center justify-between px-2 py-1 mb-2 border-b"
            >
              <div class="text-xl">{$t("component.file_view.filelist")}</div>
              {#if fileType === FileType.Video || fileType === FileType.Music || fileType === FileType.Image}
                <div class="flex flex-row items-center">
                  {#each loopIcons as icon, i}
                    <Icon
                      type={getIconType(icon)}
                      color={getIconColor(icon)}
                      size="small"
                      className="ml-2 cursor-pointer"
                      onClick={() => selectLoopMethod(i)}
                    />
                  {/each}
                </div>
              {/if}
            </div>
            <div class="filelist-height overflow-y-auto flex-grow px-1">
              {#each siblings as sibling, i}
                {#if filename === sibling.filename}
                  <div class="bg-blue-400 text-white rounded mb-1">
                    <span class="px-2 py-1">
                      {sibling.filename}
                    </span>
                  </div>
                {:else}
                  <div
                    class="rounded cursor-pointer hover:bg-blue-400 hover:text-white mb-1"
                    on:click={() => selectSibling(i)}
                  >
                    <span class="px-2 py-1">
                      {sibling.filename}
                    </span>
                  </div>
                {/if}
              {/each}
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  @media only screen and (min-width: 320px) {
    .filelist-height {
      height: 12rem;
    }
  }

  @media only screen and (min-width: 1024px) {
    .filelist-height {
      height: 20rem;
    }
  }
</style>
