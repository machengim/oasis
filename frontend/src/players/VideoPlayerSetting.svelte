<script lang="ts">
  export let speed: number;
  export let loop: "none" | "current" | "list";

  const speeds = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
  const loops = ["None", "Current File", "Play List"];
  let selectedMenu: "speed" | "loop" | "main" = "main";

  const selectMenu = (menu: "speed" | "loop" | "main") => {
    selectedMenu = menu;
  };
</script>

<div class="h-30">
  {#if selectedMenu === "speed"}
    <div
      class="p-1 hover:bg-gray-500 cursor-pointer"
      on:click={() => selectMenu("main")}
    >
      &lt; Settings
    </div>
    <hr />
    {#each speeds as speed}
      <div class="p-1 hover:bg-gray-500 cursor-pointer">{speed}</div>
    {/each}
  {:else if selectedMenu === "loop"}
    <div
      class="p-1 hover:bg-gray-500 cursor-pointer"
      on:click={() => selectMenu("main")}
    >
      &lt; Settings
    </div>
    <hr />
    {#each loops as loop}
      <div class="p-1 hover:bg-gray-500 cursor-pointer">{loop}</div>
    {/each}
  {:else}
    <div
      class="flex flex-row justify-between p-1 hover:bg-gray-500 cursor-pointer"
      on:click={() => selectMenu("speed")}
    >
      <span>Speed: {speed}</span><span> &gt;</span>
    </div>
    <div
      class="flex flex-row justify-between p-1 hover:bg-gray-500 cursor-pointer"
      on:click={() => selectMenu("loop")}
    >
      <span>Loop: {loop}</span><span> &gt;</span>
    </div>
  {/if}
</div>
