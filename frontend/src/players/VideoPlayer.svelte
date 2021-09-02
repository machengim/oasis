<script lang="ts">
  import { Player, DefaultUi, Settings, Video } from "@vime/svelte";

  export let dirs: Array<string>;
  export let filename: string;

  // Bug when using Vime with typescript.
  const props = {
    class: "",
    style: "",
  };
  let video_path: string;

  $: if (dirs && filename) {
    buildVideoPath();
  }

  const buildVideoPath = () => {
    let dir = dirs.join("/");
    let file_path = dir ? dir + "/" + filename : filename;

    video_path = "/api/file?path=" + encodeURIComponent(file_path);
  };
</script>

<Player {...props}>
  <!-- svelte-ignore a11y-media-has-caption -->
  <Video crossOrigin="" {...props}>
    <!-- These are passed directly to the underlying HTML5 `<video>` element. -->
    <!-- Why `data-src`? Lazy loading, you can always use `src` if you prefer.  -->
    <source data-src={video_path} type="video/mp4" />
    <track
      default
      kind="subtitles"
      src="https://media.vimejs.com/subs/english.vtt"
      label="Caption"
    />
  </Video>

  <!-- We turn off the controls that come with the default UI. -->
  <DefaultUi noSettings {...props}>
    <!-- We setup the default controls and pass in any options. -->
    <Settings {...props}>
      <div>
        <div
          class="flex flex-row justify-between p-1 hover:bg-gray-500 cursor-pointer"
        >
          <span>Speed: 1.0</span><span> &gt;</span>
        </div>
        <div
          class="flex flex-row justify-between p-1 hover:bg-gray-500 cursor-pointer"
        >
          <span>Loop: None</span><span> &gt;</span>
        </div>
      </div>
    </Settings>
  </DefaultUi>
</Player>
