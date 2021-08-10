<script lang="ts">
  import Icon from "../components/Icon.svelte";

  export let path: string = "/";

  const buildNav = () => {
    switch (path) {
      case "/setup":
        return "Oasis &gt; Setup";
      case "/login":
        return "Oasis &gt; Login";
      default:
        return "Oasis &gt; Files";
    }
  };
</script>

<div class="w-full h-14 bg-gray-50 shadow">
  <div class="w-4/5 h-full flex flex-row justify-between items-center mx-auto">
    <div class="text-xl">{@html buildNav()}</div>
    <div class="flex flex-row">
      <Icon type="profile" color="gray" />
    </div>
  </div>
</div>
