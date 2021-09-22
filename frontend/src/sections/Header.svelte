<script lang="ts">
  import { onDestroy } from "svelte";
  import { siteStore, sectionStore, getSitename } from "../utils/store";
  import AvatarMenu from "./AvatarMenu.svelte";
  import Icon from "../components/Icon.svelte";
  import Title from "./Title.svelte";
  import { EIconType } from "../utils/types";
  import SignOutModal from "../modals/SignOutModal.svelte";
  import ShutdownModal from "../modals/ShutdownModal.svelte";
  import AboutModal from "../modals/AboutModal.svelte";

  let sitename = getSitename();
  let section: string;
  let showMenu = false;
  let showSignOutModal = false;
  let showShutdownModal = false;
  let showAboutModal = false;

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
</script>

{#if showSignOutModal}
  <SignOutModal onClose={closeSignOutModal} />
{/if}
{#if showShutdownModal}
  <ShutdownModal onClose={clostShutdownModal} />
{/if}
{#if showAboutModal}
  <AboutModal onClose={closeAboutModal} />
{/if}
<div class="w-full h-14 bg-gray-50 shadow">
  <div
    class="w-11/12 lg:w-4/5 h-full flex flex-row justify-between items-center mx-auto"
  >
    <Title {sitename} {section} />
    {#if section === "files" || section === "settings" || section === "profile"}
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
