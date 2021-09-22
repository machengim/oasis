<script lang="ts">
  import Icon from "./Icon.svelte";
  import { EIconColor, EIconType } from "../utils/types";

  export let size: "small" | "normal" | "big" = "normal";
  export let clickOutToClose = false;
  export let title = "Title";
  export let onClose: () => void;
  const clickInside = (e: Event) => {
    e.stopPropagation();
  };
</script>

<div
  class="fixed top-0 left-0 z-40 w-screen h-screen flex flex-col bg-gray-400 bg-opacity-40"
  on:click={clickOutToClose ? onClose : () => {}}
>
  <div
    class="modal-{size} mx-auto my-60 bg-white border-gray-400 shadow"
    on:click={clickInside}
  >
    <div class="border-b border-gray-200 p-4 flex flex-row justify-between">
      <div class="text-lg text-black">{title}</div>
      <Icon
        type={EIconType.close}
        color={EIconColor.gray}
        size="small"
        className="cursor-pointer"
        onClick={onClose}
      />
    </div>
    <slot />
  </div>
</div>

<style>
  @media only screen and (min-width: 320px) {
    .modal-small {
      width: 12rem;
    }
    .modal-normal {
      width: 16rem;
    }
    .modal-large {
      width: 20rem;
    }
  }

  @media only screen and (min-width: 1024px) {
    .modal-small {
      width: 20rem;
    }
    .modal-normal {
      width: 30rem;
    }
    .modal-large {
      width: 40rem;
    }
  }
</style>
