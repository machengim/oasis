<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { autoPlayStore } from "../utils/store";
  import { getRange } from "../utils/api";
  import Icon from "../components/Icon.svelte";
  import Spinner from "../components/Spinner.svelte";
  import type { IPartialBlob } from "../utils/types";
  import { checkMobile } from "../utils/util";

  export let dirs: Array<string>;
  export let filename: string;
  export let onMediaComplete: () => void;
  export let moveNext: () => void;
  export let moveBack: () => void;

  let player: HTMLElement;
  let imagePath: string;
  let imageType: string;
  let start: number = 0;
  let imgSrc: string = null;
  let blobs: Array<Blob> = [];
  let isLoading = false;
  let showMenu = false;
  let touchPointX = -1;
  let loaded = false;
  let loadTime = 0;
  let autoPlayTimeout: NodeJS.Timeout;
  let menuTimeout: NodeJS.Timeout;

  const onAutoPlay = (isAutoPlay: boolean) => {
    if (isAutoPlay) {
      autoPlayTimeout = setTimeout(() => {
        onMediaComplete();
      }, 5000);
    } else if (autoPlayTimeout) {
      clearTimeout(autoPlayTimeout);
    }
  };

  const unsubscribeAutoPlay = autoPlayStore.subscribe((autoPlay) => {
    onAutoPlay(autoPlay);
  });

  onMount(() => {
    onAutoPlay($autoPlayStore);

    player.addEventListener("touchstart", onTouchStart, true);
    player.addEventListener("touchmove", onTouchMove, true);
  });

  onDestroy(() => {
    unsubscribeAutoPlay();
  });

  $: if (dirs && filename) {
    reset();
    imagePath = buildMediaPath();
  }

  $: if (imagePath) {
    isLoading = true;
    fetchImage(0);
  }

  $: if (start) {
    fetchImage(start);
  }

  $: if (blobs.length > 0) {
    const current = +new Date();
    if (loaded || current - loadTime > 700) {
      const imageBlob = new Blob(blobs, { type: imageType });
      imgSrc = URL.createObjectURL(imageBlob);
      loadTime = current;
      if (isLoading) {
        isLoading = false;
      }
    }
  }

  $: if (showMenu) {
    menuTimeout = setTimeout(() => {
      showMenu = false;
    }, 2000);
  }

  const reset = () => {
    touchPointX = -1;
    start = 0;
    imgSrc = null;
    imageType = null;
    blobs = [];
    isLoading = false;
    loaded = false;
    loadTime = 0;

    if (menuTimeout) {
      clearTimeout(menuTimeout);
      menuTimeout = setTimeout(() => {
        showMenu = false;
      }, 2000);
    }

    if (autoPlayTimeout) {
      clearTimeout(autoPlayTimeout);
    }

    onAutoPlay($autoPlayStore);
  };

  const buildMediaPath = () => {
    let dir = dirs.join("/");
    let filePath = dir ? dir + "/" + filename : filename;

    return "/api/file?path=" + encodeURIComponent(filePath);
  };

  const fetchImage = async (startFrom: number) => {
    const partialBlob: IPartialBlob = await getRange(imagePath, startFrom);
    if (!imageType) {
      imageType = partialBlob.type;
    }

    blobs.push(partialBlob.blob);
    blobs = blobs;
    if (partialBlob.end + 1 < partialBlob.size) {
      start = partialBlob.end + 1;
    } else {
      loaded = true;
    }
  };

  const toggleFullScreen = async () => {
    if (player && !document.fullscreenElement) {
      try {
        await player.requestFullscreen();
      } catch (e) {
        console.error(e);
      }

      if (checkMobile()) {
        await window.screen.orientation.lock("landscape");
      }
    } else {
      await document.exitFullscreen();
      if (checkMobile()) {
        window.screen.orientation.unlock();
      }
    }
  };

  const onMoveInImage = () => {
    if (!showMenu) showMenu = true;
  };

  const changeImage = (step: number) => {
    if (step === 1) {
      moveNext();
    } else if (step === -1) {
      moveBack();
    }
  };

  const handleKeydown = (event: KeyboardEvent) => {
    const key = event.key;

    switch (key) {
      case "ArrowLeft":
        changeImage(-1);
        break;
      case "ArrowRight":
        changeImage(1);
      default:
        break;
    }
  };

  const onTouchStart = (event: TouchEvent) => {
    touchPointX = event.touches[0].clientX;
  };

  const onTouchMove = (event: TouchEvent) => {
    if (touchPointX < 0) return;

    const newPointX = event.changedTouches[0].clientX;

    if (newPointX - touchPointX > 5) {
      moveBack();
    } else if (newPointX - touchPointX < -5) {
      moveNext();
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<div
  class="relative w-full h-full bg-black"
  bind:this={player}
  on:mousemove={onMoveInImage}
>
  {#if isLoading}
    <div class="w-full h-full flex flex-row justify-center items-center">
      <Spinner />
    </div>
  {:else}
    <div class="w-full h-full flex flex-row justify-center items-center">
      <img src={imgSrc} alt={filename} class="max-w-full max-h-full" />
    </div>
    {#if showMenu}
      <div class="absolute top-0 left-0 z-10 w-full h-full">
        <div
          class="w-full h-full flex flex-row justify-between items-center my-auto"
        >
          <Icon
            type="back"
            color="gray"
            size="large"
            className="cursor-pointer"
            onClick={() => changeImage(-1)}
          />
          <Icon
            type="expand"
            color="gray"
            size="large"
            className="cursor-pointer"
            onClick={toggleFullScreen}
          />
          <Icon
            type="forward"
            color="gray"
            size="large"
            className="cursor-pointer"
            onClick={() => changeImage(1)}
          />
        </div>
      </div>
    {/if}
  {/if}
</div>
