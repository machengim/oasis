<script lang="ts">
  import { onMount } from "svelte";
  import { autoPlayStore } from "../utils/store";
  import Plyr from "plyr";
  import { FileType } from "../utils/types";

  export let dirs: Array<string>;
  export let filename: string;
  export let fileType: FileType;
  export let onComplete: () => void;
  let player: Plyr;
  let isLoop = false;

  onMount(() => {
    player = initPlayer();

    player.on("ready", (_) => {
      if ($autoPlayStore) {
        player.play();
      }
    });

    player.on("ended", (_) => {
      onComplete();
    });
  });

  $: if (dirs && filename && player) {
    const mediaType = getMediaType();
    const trackSrc = mediaType === "video" ? buildTrackPath() : "";

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
        src: buildMediaPath(),
        type: mediaSrcType,
      },
    ];
  };

  const buildMediaPath = () => {
    let dir = dirs.join("/");
    let filePath = dir ? dir + "/" + filename : filename;

    return "/api/file?path=" + encodeURIComponent(filePath);
  };

  const buildTrackPath = () => {
    const splits = filename.split(".");
    if (splits.length <= 1) return null;

    splits.pop();
    const trackFilename = splits.join("") + ".vtt";
    let dir = dirs.join("/");
    let filePath = dir ? dir + "/" + trackFilename : trackFilename;

    return "/api/track?path=" + encodeURIComponent(filePath);
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
    <div class="mt-36 lg:px-24">
      <audio class="player" controls>
        <source type="audio/mp3" />
        <track kind="captions" default />
      </audio>
    </div>
  {/if}
</div>
