<script lang="ts">
  import {
    Player,
    DefaultUi,
    Settings,
    Video,
    Submenu,
    MenuRadioGroup,
    MenuRadio,
  } from "@vime/svelte";
  import VideoPlayerSetting from "./VideoPlayerSetting.svelte";

  export let dirs: Array<string>;
  export let filename: string;
  let currentTime = 0;
  let speed = 1.0;
  let loop: "none" | "current" | "list" = "none";
  let value = "1";

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

  const onTimeUpdate = (event: CustomEvent<number>) => {
    currentTime = event.detail;
  };

  const handleKeydown = (event: KeyboardEvent) => {
    let key = event.key;
    switch (key) {
      case "ArrowRight":
        currentTime += 5;
        break;
      case "ArrowLeft":
        currentTime -= 5;
        break;
      default:
        break;
    }
  };

  const onCheck = (event: Event) => {
    const radio = event.target as HTMLVmMenuRadioElement;
    value = radio.value;
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<Player
  {currentTime}
  on:vmCurrentTimeChange={onTimeUpdate}
  loop={loop === "current"}
  playbackRate={speed}
  {...props}
>
  <!-- svelte-ignore a11y-media-has-caption -->
  <Video crossOrigin="" {...props}>
    <source data-src={video_path} type="video/mp4" />
    <track
      default
      kind="subtitles"
      src="https://media.vimejs.com/subs/english.vtt"
      label="Caption"
    />
  </Video>

  <DefaultUi noSettings {...props}>
    <Settings {...props}>
      <Submenu label="Submenu 1" {...props}>
        <MenuRadioGroup {value} on:vmCheck={onCheck} {...props}>
          <MenuRadio label="Option 1" value="1" {...props} />
          <MenuRadio label="Option 2" value="2" {...props} />
          <MenuRadio label="Option 3" value="3" {...props} />
        </MenuRadioGroup>
      </Submenu>

      <Submenu label="Submenu 2" {...props}>Random content in here.</Submenu>
    </Settings>
  </DefaultUi>
</Player>
