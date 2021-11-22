<script lang="ts">
  import { onDestroy } from "svelte";
  import { siteStore, sectionStore, getSitename } from "../utils/store";
  import AvatarMenu from "./AvatarMenu.svelte";
  import Icon from "../components/Icon.svelte";
  import Title from "./Title.svelte";
  import { EIconColor, EIconType } from "../utils/enums";
  import type { IUpdateAppInfo } from "../utils/types";
  import SignOutModal from "../modals/SignOutModal.svelte";
  import ShutdownModal from "../modals/ShutdownModal.svelte";
  import AboutModal from "../modals/AboutModal.svelte";
  import UpdateModal from "../modals/UpdateModal.svelte";
  import SearchModal from "../modals/SearchModal.svelte";

  let sitename = getSitename();
  let section: string;
  let updateInfo: IUpdateAppInfo;
  let showMenu = false;
  let showSignOutModal = false;
  let showShutdownModal = false;
  let showAboutModal = false;
  let showUpdateModal = false;
  let showSearchModal = false;

  const unsubscribeSection = sectionStore.subscribe((newSection) => {
    if (newSection) section = newSection;
  });

  const unsubscribeSite = siteStore.subscribe((newSite) => {
    if (newSite && newSite.name !== sitename) {
      sitename = newSite.name;
    }
  });

  onDestroy(() => {
    unsubscribeSection();
    unsubscribeSite();
  });

  $: if (showMenu) {
    document.addEventListener("click", closeAvatarMenu, false);
  } else {
    document.removeEventListener("click", closeAvatarMenu);
  }

  $: if (updateInfo) {
    showAboutModal = false;
    showUpdateModal = true;
  }

  const openAvatarMenu = (e: Event) => {
    e.stopPropagation();
    showMenu = true;
  };

  const openModal = (name: string) => {
    showMenu = false;
    switch (name) {
      case "signout":
        showSignOutModal = true;
        break;
      case "shutdown":
        showShutdownModal = true;
        break;
      case "about":
        showAboutModal = true;
        break;
      case "search":
        showSearchModal = true;
        break;
      default:
        break;
    }
  };

  const closeAvatarMenu = () => {
    showMenu = false;
  };

  const closeSignOutModal = () => {
    showSignOutModal = false;
  };

  const clostShutdownModal = () => {
    showShutdownModal = false;
  };

  const closeAboutModal = () => {
    showAboutModal = false;
  };

  const setUpdateInfo = (info: IUpdateAppInfo) => {
    updateInfo = info;
  };

  const closeUpdateModal = () => {
    showUpdateModal = false;
  };

  const closeSearchModal = () => {
    showSearchModal = false;
  };
</script>

{#if showSignOutModal}
  <SignOutModal onClose={closeSignOutModal} />
{/if}
{#if showShutdownModal}
  <ShutdownModal onClose={clostShutdownModal} />
{/if}
{#if showAboutModal}
  <AboutModal onClose={closeAboutModal} {setUpdateInfo} />
{/if}
{#if showUpdateModal}
  <UpdateModal onClose={closeUpdateModal} {updateInfo} />
{/if}
{#if showSearchModal}
  <SearchModal onClose={closeSearchModal} />
{/if}
<div class="w-full h-14 bg-gray-50 shadow">
  <div
    class="w-11/12 lg:w-4/5 h-full flex flex-row justify-between items-center mx-auto"
  >
    <Title {sitename} {section} />
    {#if section === "files" || section === "settings" || section === "profile"}
      <div
        class="flex flex-row w-40 lg:w-96 items-center border-b pb-1 border-gray-400 cursor-pointer"
        on:click={() => openModal("search")}
      >
        <Icon type={EIconType.search} size="tiny" color={EIconColor.gray} />
        <span class="ml-2">Search..</span>
      </div>
      <div class="relative">
        <Icon
          type={EIconType.profile}
          onClick={openAvatarMenu}
          className="cursor-pointer"
        />
        {#if showMenu}
          <AvatarMenu onClose={closeAvatarMenu} onOpenModal={openModal} />
        {/if}
      </div>
    {/if}
  </div>
</div>
