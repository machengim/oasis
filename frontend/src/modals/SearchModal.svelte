<script lang="ts">
  import Modal from "../components/Modal.svelte";
  import Icon from "../components/Icon.svelte";
  import { EFileType, EIconColor, EIconType } from "../utils/enums";
  import FileIcon from "../components/FileIcon.svelte";

  export let onClose: () => void;
  let keyword: string;
</script>

<Modal {onClose} size="large" showTitle={false} clickOutToClose={true}>
  <div class="flex flex-row justify-between items-center p-4">
    <Icon type={EIconType.search} size="small" color={EIconColor.blue} />
    <input
      type="text"
      class="w-full ml-2 border-none focus:outline-none"
      bind:value={keyword}
      placeholder="Search.."
    />
    <Icon
      type={EIconType.close}
      color={EIconColor.gray}
      size="small"
      className="cursor-pointer"
      onClick={onClose}
    />
  </div>
  <hr />
  <div class="result-list overflow-y-auto">
    <div
      class="grid grid-cols-3 p-4 bg-white text-lg hover:bg-gray-200 cursor-pointer"
    >
      <div class="col-span-3 lg:col-span-1 px-2 flex flex-row items-center">
        <Icon type={EIconType.code} color={EIconColor.pink} size="small" />
        <div class="ml-2 text-black break-all overflow-ellipsis">
          zyx91283293.js
        </div>
      </div>
      <div
        class="col-span-3 lg:col-span-2 px-2 lg:px-0 text-sm text-gray-400 break-all overflow-ellipsis my-auto"
      >
        /home/cheng/Documents/safo3k43mkdfnamlkfaslkldsflkasm3l4afadsfreewdfs
      </div>
    </div>
    <div
      class="grid grid-cols-3 p-4 bg-white text-lg hover:bg-gray-200 cursor-pointer"
    >
      <div class="col-span-3 lg:col-span-1 px-2 flex flex-row items-center">
        <Icon type={EIconType.code} color={EIconColor.pink} size="small" />
        <div class="ml-2 text-black break-all overflow-ellipsis">
          zyx91283293.js
        </div>
      </div>
      <div
        class="col-span-3 lg:col-span-2 px-2 lg:px-0 text-sm text-gray-400 break-all overflow-ellipsis my-auto"
      >
        /home/cheng/Documents/safo3k43mkdfnamlkfaslkldsflkasm3l4afadsfreewdfs
      </div>
    </div>
  </div>
</Modal>

<style>
  @media only screen and (min-width: 320px) {
    .result-list {
      max-height: 20rem;
    }
  }

  @media only screen and (min-width: 768px) {
    .result-list {
      max-height: 40rem;
    }
  }
</style>
