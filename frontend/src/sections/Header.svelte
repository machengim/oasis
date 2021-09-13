<script lang="ts">
  import { onDestroy } from "svelte";
  import { sectionStore } from "../utils/store";
  import Icon from "../components/Icon.svelte";
  import Title from "./Title.svelte";
  import { EIconType } from "../utils/types";

  let section: string;

  const unsubscribeSection = sectionStore.subscribe((newSection) => {
    if (newSection) section = newSection;
  });

  onDestroy(() => {
    unsubscribeSection();
  });
</script>

<div class="w-full h-14 bg-gray-50 shadow">
  <div
    class="w-11/12 lg:w-4/5 h-full flex flex-row justify-between items-center mx-auto"
  >
    <Title {section} />
    {#if section === "files"}
      <div class="flex flex-row">
        <Icon type={EIconType.profile} />
      </div>
    {/if}
  </div>
</div>
