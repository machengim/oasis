<script lang="ts">
  import { onMount } from "svelte";
  import Plyr from "plyr";

  export let dirs: Array<string>;
  export let filename: string;
  let player: Plyr;
  let video_path: string;
  let track_path = "https://media.vimejs.com/subs/english.vtt";
  let is_loop = false;

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
  });

  $: if (dirs && filename) {
    buildVideoPath();
  }

  const buildVideoPath = () => {
    let dir = dirs.join("/");
    let file_path = dir ? dir + "/" + filename : filename;

    video_path = "/api/file?path=" + encodeURIComponent(file_path);
  };
</script>

<div>
  <video
    id="player"
    crossorigin="anonymous"
    playsinline
    controls
    loop={is_loop}
  >
    <source src={video_path} type="video/mp4" />
    <track kind="captions" src={track_path} default />
  </video>
</div>
