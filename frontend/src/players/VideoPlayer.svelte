<script lang="ts">
  import { onMount } from "svelte";
  import { autoPlayStore } from "../utils/store";
  import Plyr from "plyr";

  export let dirs: Array<string>;
  export let filename: string;
  export let onComplete: () => void;
  let player: Plyr;
  let isLoop = false;

  onMount(() => {
    player = new Plyr("video", {
      controls: [
        "play-large",
        "play",
        "progress",
        "current-time",
        "duration",
        "mute",
        "volume",
        "captions",
        "settings",
        "fullscreen",
      ],
      settings: ["speed", "loop"],
      keyboard: { focused: true, global: true },
      speed: { selected: 1, options: [0.5, 0.75, 1, 1.25, 1.5, 2] },
    });

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
    player.source = {
      type: "video",
      sources: [
        {
          src: buildVideoPath(),
          type: "video/mp4",
        },
      ],
      tracks: [
        {
          kind: "captions",
          label: "Caption",
          src: buildTrackPath(),
          default: true,
        },
      ],
    };
  }

  const buildVideoPath = () => {
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
</div>
