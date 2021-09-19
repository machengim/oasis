<script lang="ts">
  import { useNavigate } from "svelte-navigator";
  import { readCookie } from "../utils/util";

  const navigate = useNavigate();
  const username = readCookie("uname") || "";
  export let onSignOut: () => void;

  const toPage = (page: "profile" | "setting" | "login") => {
    navigate("/" + page);
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
      Profile
    </div>
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={() => toPage("setting")}
    >
      Setting
    </div>
    <hr />
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={onSignOut}
    >
      Sign out
    </div>
  {:else}
    <div
      class="px-4 my-1 cursor-pointer hover:bg-blue-400 hover:text-white"
      on:click={() => toPage("login")}
    >
      Login
    </div>
  {/if}
</div>

<style>
  .menu-width {
    min-width: 10rem;
  }
</style>
