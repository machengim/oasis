<script lang="ts">
  export let path: string = "/";

  const buildNav = () => {
    console.log("inside: ", path);
    switch (path) {
      case "/setup":
        return "Oasis &gt; Setup";
      case "/login":
        return "Oasis &gt; Login";
      default:
        return "Oasis";
    }
  };
</script>

<div class="w-full h-14 bg-gray-50 shadow">
  <div class="w-4/5 h-full flex flex-row justify-between items-center mx-auto">
    <div class="text-xl">{@html buildNav()}</div>
  </div>
</div>
