<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "../components/Icon.svelte";
  import { EIconColor, EIconType } from "../utils/types";
  import { checkMobile } from "../utils/util";

  export let filePath: string;
  let container: HTMLDivElement;
  let video: HTMLVideoElement;
  let fullscreen = false;

  onMount(() => {
    document.addEventListener("fullscreenchange", onFullScreenChange, true);
  });

  const onFullScreenChange = () => {
    fullscreen = !!document.fullscreenElement;
  };

  const toggleFullScreen = async () => {
    if (container && !fullscreen) {
      await container.requestFullscreen();
    } else {
      await document.exitFullscreen();

      // if (checkMobile()) {
      //   window.screen.orientation.unlock();
      // }
    }
  };
</script>

<div class="relative" bind:this={container}>
  <!-- svelte-ignore a11y-media-has-caption -->
  <video bind:this={video} preload="metadata">
    <source src={filePath} type="video/mp4" />
  </video>

  <div
    class="w-full h-full absolute left-0 top-0 bottom-0 py-4 flex flex-col justify-between bg-opacity-50 bg-black text-white"
  >
    {#if fullscreen}
      <div class="w-full px-4 flex flex-row justify-between">
        <Icon
          type={EIconType.back}
          color={EIconColor.white}
          className="cursor-pointer"
          onClick={toggleFullScreen}
        />
        <span>{filePath}</span>
        <span>&nbsp;</span>
      </div>
    {:else}
      <div>&nbsp;</div>
    {/if}
    <div class="w-full flex flex-row justify-center items-center">
      <Icon type={EIconType.play_back} color={EIconColor.white} />
      <Icon
        type={EIconType.play}
        color={EIconColor.white}
        size="huge"
        className="ml-24 mr-24"
      />
      <Icon type={EIconType.play_forward} color={EIconColor.white} />
    </div>
    <div class="w-full">
      <div class="w-full">
        <input type="range" value="0" min="0" class="w-full" />
      </div>
      <div class="px-4 flex flex-row justify-between items-center">
        <div>0:13 / 5:21</div>
        <div class="flex flex-row">
          <Icon
            type={EIconType.play_speed}
            color={EIconColor.white}
            className="mr-4"
          />
          <Icon
            type={EIconType.close_caption}
            color={EIconColor.white}
            className="mr-4"
          />
          {#if fullscreen}
            <Icon
              type={EIconType.list}
              color={EIconColor.white}
              className="mr-4"
            />
            <Icon
              type={EIconType.play_next}
              color={EIconColor.white}
              className="mr-4"
            />
          {/if}
          {#if !fullscreen}
            <Icon
              type={EIconType.expand}
              color={EIconColor.white}
              className="cursor-pointer"
              onClick={toggleFullScreen}
            />
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>
