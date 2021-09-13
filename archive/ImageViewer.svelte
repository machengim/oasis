<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { loopStore } from "../utils/store";
  import { getRange } from "../utils/api";
  import Icon from "../components/Icon.svelte";
  import Spinner from "../components/Spinner.svelte";
  import type { IPartialBlob } from "../utils/types";
  import { checkMobile } from "../utils/util";
  import { EIconType, EIconColor, ELoopMethod } from "../utils/types";

  export let filename: string;
  export let filePath: string;
  export let onMediaComplete: () => void;
  export let moveNext: () => void;
  export let moveBack: () => void;

  let player: HTMLElement;
  let imgDiv: HTMLElement;
  let imageType: string;
  let start: number = 0;
  let imgSrc: string = null;
  let blobs: Array<Blob> = [];
  let isLoading = false;
  let showMenu = false;
  let touchPointX = -1;
  let loaded = false;
  let loadTime = 0;
  let fullscreen = false;
  let autoPlayTimeout: NodeJS.Timeout;
  let menuTimeout: NodeJS.Timeout;

  const onAutoPlay = (loopMethod: ELoopMethod) => {
    if (loopMethod) {
      autoPlayTimeout = setTimeout(() => {
        onMediaComplete();
      }, 5000);
    } else if (autoPlayTimeout) {
      clearTimeout(autoPlayTimeout);
    }
  };

  const unsubscribeAutoPlay = loopStore.subscribe((value) => {
    if (value && value !== ELoopMethod.repeat) {
      onAutoPlay(value);
    } else if (autoPlayTimeout) {
      clearTimeout(autoPlayTimeout);
    }
  });

  onMount(() => {
    onAutoPlay($loopStore);

    player.addEventListener("touchstart", onTouchStart, true);
    player.addEventListener("touchmove", onTouchMove, true);
    document.addEventListener("fullscreenchange", onFullScreenChange, true);
  });

  onDestroy(() => {
    unsubscribeAutoPlay();

    if (autoPlayTimeout) {
      clearTimeout(autoPlayTimeout);
    }
    if (menuTimeout) {
      clearTimeout(menuTimeout);
    }

    player.removeEventListener("touchstart", onTouchStart);
    player.removeEventListener("touchmove", onTouchMove);
    document.removeEventListener("fullscreenchange", onFullScreenChange);
  });

  $: if (filePath) {
    reset();
    isLoading = true;
    fetchImage(0);
  }

  $: if (start) {
    fetchImage(start);
  }

  $: if (blobs.length > 0) {
    const current = +new Date();
    if (loaded || current - loadTime > 600) {
      let imageBlob = new Blob(blobs, { type: imageType });
      const prevSrc = imgSrc;
      imgSrc = URL.createObjectURL(imageBlob);
      loadTime = current;

      if (isLoading) {
        isLoading = false;
      }
      if (prevSrc) {
        URL.revokeObjectURL(prevSrc);
      }
    }
  }

  $: if (showMenu) {
    menuTimeout = setTimeout(() => {
      showMenu = false;
    }, 2000);
  }

  const reset = () => {
    if (imgSrc) {
      URL.revokeObjectURL(imgSrc);
    }

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

    onAutoPlay($loopStore);
  };

  const fetchImage = async (startFrom: number) => {
    const currentPath = filePath;
    let partialBlob: IPartialBlob = await getRange(filePath, startFrom);
    if (currentPath !== filePath) {
      return;
    }

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
    if (player && !fullscreen) {
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
        break;
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

    if (newPointX - touchPointX > 60) {
      moveBack();
    } else if (newPointX - touchPointX < -60) {
      moveNext();
    }
  };

  const onFullScreenChange = () => {
    fullscreen = !!document.fullscreenElement;
    toggleImgClass();
  };

  const buildImgClass = () => {
    let className = "w-full flex flex-row justify-center items-center";
    className += fullscreen ? " h-full" : " img-height";

    return className;
  };

  const toggleImgClass = () => {
    if (!imgDiv) return;

    const imgHeight = "img-height";
    const hFull = "h-full";

    if (fullscreen) {
      imgDiv.classList.remove(imgHeight);
      imgDiv.classList.add(hFull);
    } else {
      imgDiv.classList.remove(hFull);
      imgDiv.classList.add(imgHeight);
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<div
  class="relative w-full h-full bg-black"
  bind:this={player}
  on:dblclick={toggleFullScreen}
  on:mousemove={onMoveInImage}
>
  {#if isLoading}
    <div
      class="w-full img-height py-6 flex flex-row justify-center items-center"
    >
      <Spinner text={"Loading " + filename + "..."} />
    </div>
  {:else}
    <div class={buildImgClass()} bind:this={imgDiv}>
      <img
        src={imgSrc}
        alt={filename}
        class="max-w-full h-full object-contain"
      />
    </div>
    {#if showMenu}
      <div class="absolute top-0 left-0 z-10 w-full h-full">
        {#if fullscreen}
          <div class="absolute top-2 right-2 xl:top-8 xl:right-8">
            <Icon
              type={EIconType.closecircle}
              color={EIconColor.gray}
              size="large"
              className="cursor-pointer"
              onClick={toggleFullScreen}
            />
          </div>
        {:else}
          <div class="absolute bottom-2 right-2 xl:bottom-8 xl:right-8">
            <Icon
              type={EIconType.expand}
              color={EIconColor.gray}
              size="large"
              className="cursor-pointer"
              onClick={toggleFullScreen}
            />
          </div>
        {/if}
        <div
          class="w-full h-full px-2 xl:px-8 flex flex-row justify-between items-center my-auto"
        >
          <Icon
            type={EIconType.back}
            color={EIconColor.gray}
            size="large"
            className="cursor-pointer"
            onClick={() => changeImage(-1)}
          />
          <Icon
            type={EIconType.forward}
            color={EIconColor.gray}
            size="large"
            className="cursor-pointer"
            onClick={() => changeImage(1)}
          />
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  @media only screen and (min-width: 600px) {
    .img-height {
      height: 60vh;
    }
  }

  @media only screen and (min-width: 1024px) {
    .img-height {
      height: 80vh;
    }
  }
</style>
