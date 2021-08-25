<script lang="ts">
  import { useParams } from "svelte-navigator";
  import { filesStore } from "../utils/store";
  import type { IFile } from "../utils/types";

  const param = useParams();

  let file_id = 0;
  let file: IFile;
  let video: HTMLVideoElement;
  let siblings: Array<IFile> = [];
  $: file_id = +$param.file_id || 0;

  // TODO: if user jumps to this url directlym, there is no content in store.
  $: if (file_id > 0) {
    file = $filesStore.find((f) => f.file_id === file_id);
    siblings = $filesStore.filter((f) => f.file_type === file.file_type);
  }

  const handleKeydown = (e: KeyboardEvent) => {
    if (!video) return;
    e.preventDefault();
    const key = e.key;

    switch (key) {
      case "ArrowLeft":
        video.currentTime -= 5;
        break;
      case "ArrowRight":
        video.currentTime += 5;
        break;
      case "ArrowUp":
        video.volume += 0.05;
        break;
      case "ArrowDown":
        video.volume -= 0.05;
        break;
      default:
        break;
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="relative w-full h-full pb-12">
  <div class="w-11/12 lg:w-4/5 h-full mx-auto mt-4 lg:mt-10">
    <div class="text-gray-500 text-xl font-bold">
      {file.filename}
    </div>
    <div class="flex flex-row flex-wrap mt-4">
      <div class="w-full lg:w-3/4 mb-4">
        {#if file.file_type === "Video"}
          <video
            class="w-full max-h-full"
            controls
            playsinline
            bind:this={video}
          >
            <track kind="captions" />
            <source src={`/api/file/${file_id}`} type="video/mp4" />
            Your browser does not support the video tag.
          </video>
        {:else if file.file_type === "Pdf"}
          <embed
            src={`/api/file/${file_id}`}
            type="application/pdf"
            class="pdf-viewer"
          />
        {/if}
      </div>
      <div class="lg:ml-8 flex-grow">
        <div class="text-lg mb-2">Playlist:</div>
        {#each siblings as sibling}
          <div>{sibling.filename}</div>
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  .pdf-viewer {
    width: 100%;
    min-height: 70vh;
    max-height: 80vh;
  }
</style>
