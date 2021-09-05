<script lang="ts">
  import { onMount } from "svelte";
  import { autoPlayStore } from "../utils/store";
  import Plyr from "plyr";

  export let dirs: Array<string>;
  export let filename: string;
  export let onComplete: () => void;
  let player: Plyr;
  let videoPath: string;
  let trackPath = "https://media.vimejs.com/subs/english.vtt";
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

    player.on("ended", (_) => {
      onComplete();
    });

    player.on("ready", (_) => {
      if ($autoPlayStore) {
        player.play();
      }
    });
  });

  $: if (dirs && filename) {
    buildVideoPath();
  }

  $: if (player && videoPath) {
    player.source = {
      type: "video",
      sources: [
        {
          src: videoPath,
          type: "video/mp4",
        },
      ],
    };
  }

  const buildVideoPath = () => {
    let dir = dirs.join("/");
    let filePath = dir ? dir + "/" + filename : filename;

    videoPath = "/api/file?path=" + encodeURIComponent(filePath);
  };
</script>

<div>
  <video id="player" crossorigin="anonymous" playsinline controls loop={isLoop}>
    <source src={videoPath} type="video/mp4" />
    <track kind="captions" src={trackPath} default />
  </video>
</div>
