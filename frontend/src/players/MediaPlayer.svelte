<script lang="ts">
  import { onMount } from "svelte";
  import { loopStore } from "../utils/store";
  import Plyr from "plyr";
  import { ELoopMethod, FileType } from "../utils/types";
  import { checkMobile } from "../utils/util";

  export let filePath: string;
  export let trackPath: string = null;
  export let fileType: FileType;
  export let onComplete: () => void;

  let player: Plyr;
  let isLoop = false;

  onMount(() => {
    player = initPlayer();

    player.on("ready", (_) => {
      player.play();
    });

    player.on("ended", (_) => {
      if ($loopStore === ELoopMethod.repeat) {
        player.currentTime = 0;
      } else {
        onComplete();
      }
    });

    document.addEventListener("fullscreenchange", onFullScreen, true);
  });

  $: if (filePath && player) {
    const mediaType = getMediaType();
    const trackSrc = mediaType === "video" ? trackPath : "";

    player.source = {
      type: mediaType,
      sources: initMediaSource(),
      tracks: [
        {
          kind: "captions",
          label: "Caption",
          src: trackSrc,
          default: true,
        },
      ],
    };
  }

  const getMediaType = () => {
    return fileType === FileType.Video ? "video" : "audio";
  };

  const initPlayer = () => {
    const mediaType = getMediaType();
    const mediaControls = [
      "play-large",
      "play",
      "progress",
      "current-time",
      "duration",
      "mute",
      "volume",
      "captions",
      "settings",
    ];

    if (mediaType === "video") {
      mediaControls.push("fullscreen");
    }

    return new Plyr(mediaType, {
      controls: mediaControls,
      settings: ["speed", "loop"],
      keyboard: { focused: true, global: true },
      speed: { selected: 1, options: [0.5, 0.75, 1, 1.25, 1.5, 2] },
    });
  };

  const initMediaSource = () => {
    const mediaSrcType = getMediaType() === "video" ? "video/mp4" : "audio/mp3";
    return [
      {
        src: filePath,
        type: mediaSrcType,
      },
    ];
  };

  const onFullScreen = async () => {
    if (!checkMobile()) return;
    const fullscreen = !!document.fullscreenElement;
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
</script>

<div>
  {#if fileType === FileType.Video}
    <video
      class="player"
      crossorigin="anonymous"
      playsinline
      controls
      loop={isLoop}
    >
      <source type="video/mp4" />
      <track kind="captions" default />
    </video>
  {:else}
    <div
      class="mt-4 lg:mt-36 lg:w-2/3 mx-auto p-2 border-2 rounded border-blue-400 shadow"
    >
      <audio class="player" controls>
        <source type="audio/mp3" />
        <track kind="captions" default />
      </audio>
    </div>
  {/if}
</div>
