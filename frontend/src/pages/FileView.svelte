<script lang="ts">
  import Spinner from "../components/Spinner.svelte";
  import BreadCrum from "../components/BreadCrum.svelte";
  import VideoPlayer from "../players/VideoPlayer.svelte";

  export let dirs: Array<string>;
  export let filename: string;
  let isLoading = false;
  let file_ext: string;

  $: if (filename) {
    extractFileExt(filename);
  }

  const extractFileExt = (filename: string) => {
    const splits = filename.split(".");
    if (splits.length < 2) {
      file_ext = null;
    } else {
      file_ext = splits.slice(-1)[0].toLowerCase();
    }

    console.log(file_ext);
  };
</script>

<div class="relative w-full h-full">
  <div class="w-11/12 lg:w-4/5 h-full mx-auto my-4 lg:mt-4 lg:mb-10">
    <BreadCrum {dirs} {filename} className="px-1 py-2" />
    {#if isLoading}
      <Spinner />
    {:else}
      <div class="flex flex-row flex-wrap mt-4">
        <div class="w-full lg:w-3/4">
          {#if file_ext === "mp4"}
            <!-- <VideoPlayer {dirs} {filename} /> -->
            <VideoPlayer {dirs} {filename} />
          {/if}
        </div>
        <div class="flex flex-col w-full lg:w-1/4 lg:pl-4 mt-4 lg:mt-0">
          File list
        </div>
      </div>
    {/if}
  </div>
</div>
