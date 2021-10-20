<script lang="ts">
  import screenfull from "screenfull";
  import { onMount, onDestroy } from "svelte";
  import { loopStore } from "../utils/store";
  import Icon from "../components/Icon.svelte";
  import { checkMobile, checkSafari } from "../utils/util";
  import { EIconType, EIconColor, ELoopMethod } from "../utils/types";
  import Spinner from "../components/Spinner.svelte";
  import * as api from "../utils/api";

  export let filename: string;
  export let filePath: string;
  export let onMediaComplete: () => void;
  export let moveNext: () => void;
  export let moveBack: () => void;

  let player: HTMLElement;
  let imgDiv: HTMLElement;
  let isLoading = false;
  let imgSrc: string = null;
  let showMenu = false;
  let touchPointX = -1;
  let fullscreen = false;
  let autoPlayTimeout: NodeJS.Timeout;
  let menuTimeout: NodeJS.Timeout;
  let timeOutStart = -1;

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

    player.removeEventListener("touchstart", onTouchStart, true);
    player.removeEventListener("touchmove", onTouchMove, true);
    document.removeEventListener("fullscreenchange", onFullScreenChange, true);
  });

  $: if (filePath) {
    reset();
    fetchImage();
  }

  $: if (showMenu) {
    startMenuTimer();
  }

  const startMenuTimer = () => {
    timeOutStart = new Date().getTime();
    accurateTimeOut(2000, () => (showMenu = false));
  };

  const accurateTimeOut = (length: number, callback: () => void) => {
    const timer = (t: number, callback: () => void) => {
      menuTimeout = setTimeout(() => {
        const timeElapsed = new Date().getTime() - timeOutStart;
        if (length - timeElapsed < 200) {
          callback();
        } else {
          let t = length - timeElapsed;
          timer(t, callback);
        }
      }, t);
    };

    timer(length, callback);
  };

  const reset = () => {
    touchPointX = -1;
    isLoading = false;
    if (imgSrc) {
      URL.revokeObjectURL(imgSrc);
      imgSrc = null;
    }

    if (menuTimeout) {
      clearTimeout(menuTimeout);
      startMenuTimer();
    }

    if (autoPlayTimeout) {
      clearTimeout(autoPlayTimeout);
    }

    onAutoPlay($loopStore);
  };

  async function fetchImage() {
    isLoading = true;
    const currentPath = filePath;
    let imageBlob = await api.get(filePath, "blob");
    if (currentPath !== filePath) {
      return;
    }

    imgSrc = URL.createObjectURL(imageBlob);
    isLoading = false;
  }

  const toggleFullScreen = async () => {
    if (player && screenfull.isEnabled) {
      await screenfull.toggle(player, { navigationUI: "hide" });
      // Safari doesn't fire 'fullscreenchange' event.
      if (checkSafari()) {
        fullscreen = !fullscreen;
        await onFullScreenChange();
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

  const onFullScreenChange = async () => {
    if (!checkSafari()) {
      fullscreen = !!document.fullscreenElement;
    }

    toggleImgClass();

    if (!checkMobile()) return;
    if (fullscreen) {
      try {
        await window.screen.orientation.lock("landscape");
      } catch (e) {
        console.error("Unable to lock orientation: ", e);
      }
    } else {
      window.screen.orientation.unlock();
    }
  };

  const buildImgClass = () => {
    let className = "w-full flex flex-row justify-center items-center";
    className += fullscreen ? " h-full" : " viewer-height";

    return className;
  };

  const toggleImgClass = () => {
    if (!imgDiv) return;
    const imgHeight = "viewer-height";
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
      class="w-full viewer-height py-6 flex flex-row justify-center items-center"
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
