<script lang="ts">
  import { setNotification } from "../utils/store";
  import { t } from "svelte-i18n";
  import Spinner from "../components/Spinner.svelte";
  import * as api from "../utils/api";
  import { EFileType } from "../utils/enums";

  export let dirs: Array<string>;
  export let filename: string;
  export let fileType: EFileType;
  let content: string;
  let isLoading = false;

  $: if (dirs && filename) {
    fetchText();
  }

  const fetchText = async () => {
    isLoading = true;
    const endpoint = buildResourcePath();

    try {
      content = await api.get(endpoint, "text");
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.read_file_error"));
    }

    isLoading = false;
  };

  const buildResourcePath = () => {
    let dir = dirs.join("/");
    let filePath = dir ? dir + "/" + filename : filename;

    return "/api/file/" + encodeURIComponent(filePath);
  };
</script>

{#if isLoading}
  <Spinner />
{:else}
  <div
    class="p-2 viewer-height overflow-y-auto border-2 border-gray-500 shadow"
  >
    {#if fileType === EFileType.Text}
      <pre class="text-lg">
        {content}
      </pre>
    {:else}
      <pre>
        <code>
          {content}
        </code>
      </pre>
    {/if}
  </div>
{/if}
