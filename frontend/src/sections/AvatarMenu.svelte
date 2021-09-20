<script lang="ts">
  import { t, isLoading as isLoadingI18N, locale } from "svelte-i18n";
  import { useNavigate } from "svelte-navigator";
  import { readCookie } from "../utils/util";

  const navigate = useNavigate();
  const username = readCookie("uname") || "";
  export let onSignOut: () => void;
  export let onClose: () => void;

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
  {#if username.length > 0}
    <div class="px-4 my-1 font-bold">{username}</div>
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
      on:click={onSignOut}
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
