<script lang="ts">
  import { onDestroy } from "svelte";
  import { t } from "svelte-i18n";
  import { useNavigate } from "svelte-navigator";
  import { userStore } from "../utils/store";

  const navigate = useNavigate();
  let user = $userStore;
  export let onOpenModal: (name: string) => void;
  export let onClose: () => void;

  const unsubscribeUser = userStore.subscribe((newUser) => {
    if (newUser) {
      user = newUser;
    }
  });

  onDestroy(() => {
    unsubscribeUser();
  });

  const toPage = (page: "profile" | "settings" | "login") => {
    navigate("/" + page);
    onClose();
  };

  const clickInside = (e: MouseEvent) => {
    e.stopPropagation();
  };
</script>

<div
  class="absolute top-10 right-1 z-10 text-lg menu-width border rounded bg-white"
  on:click={clickInside}
>
  {#if user}
    <div class="px-4 my-1 font-bold">{user.username}</div>
    <hr />
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={() => toPage("profile")}
    >
      {$t("component.avatar_menu.profile")}
    </div>
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={() => toPage("settings")}
    >
      {$t("component.avatar_menu.settings")}
    </div>
    <hr />
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={() => onOpenModal("about")}
    >
      {$t("component.avatar_menu.about")}
    </div>
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={() => onOpenModal("shutdown")}
    >
      {$t("component.avatar_menu.shutdown")}
    </div>
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={() => onOpenModal("signout")}
    >
      {$t("component.avatar_menu.signout")}
    </div>
  {:else}
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={() => toPage("login")}
    >
      {$t("component.avatar_menu.login")}
    </div>
  {/if}
</div>

<style>
  .menu-width {
    min-width: 10rem;
  }
</style>
