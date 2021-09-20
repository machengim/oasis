<script lang="ts">
  import { onDestroy } from "svelte";
  import { sectionStore } from "../utils/store";
  import AvatarMenu from "./AvatarMenu.svelte";
  import Icon from "../components/Icon.svelte";
  import Title from "./Title.svelte";
  import { EIconType } from "../utils/types";
  import SignOutModal from "../modals/SignOutModal.svelte";

  let section: string;
  let showMenu = false;
  let showSignOutModal = false;

  const unsubscribeSection = sectionStore.subscribe((newSection) => {
    if (newSection) section = newSection;
  });

  onDestroy(() => {
    unsubscribeSection();
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

  const closeAvatarMenu = () => {
    showMenu = false;
  };

  const signOut = () => {
    showMenu = false;
    showSignOutModal = true;
  };

  const closeSignOutModal = () => {
    showSignOutModal = false;
  };
</script>

{#if showSignOutModal}
  <SignOutModal onClose={closeSignOutModal} />
{/if}
<div class="w-full h-14 bg-gray-50 shadow">
  <div
    class="w-11/12 lg:w-4/5 h-full flex flex-row justify-between items-center mx-auto"
  >
    <Title {section} />
    {#if section === "files" || section === "settings" || section === "profile"}
      <div class="relative">
        <Icon
          type={EIconType.profile}
          onClick={openAvatarMenu}
          className="cursor-pointer"
        />
        {#if showMenu}
          <AvatarMenu onClose={closeAvatarMenu} onSignOut={signOut} />
        {/if}
      </div>
    {/if}
  </div>
</div>
