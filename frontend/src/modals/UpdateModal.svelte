<script lang="ts">
  import { t, locale } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import type { IUpdateAppInfo } from "../utils/types";

  export let onClose: () => void;
  export let updateInfo: IUpdateAppInfo;
  let description: string;

  $: if (updateInfo) {
    const descriptionLang = updateInfo.descriptions.find(
      (d) => d.lang === $locale
    );
    description = descriptionLang ? descriptionLang.detail : "";
  }

  const openUpdateUrl = () => {
    window.open(updateInfo.url);
  };
</script>

<Modal {onClose} title={$t("modal.update_app.title")}>
  <div class="p-4 text-lg">
    <div>
      <span>{$t("modal.update_app.version")}:</span>
      <span class="px-1 font-light">{updateInfo.version}</span>
    </div>
    <div class="mt-2">
      <span>{$t("modal.update_app.description")}:</span>
      <span class="px-1 font-light break-all">{description}</span>
    </div>
    <div class="w-full mt-4 p-4 flex flex-row justify-end">
      <Button
        onClick={openUpdateUrl}
        color="blue"
        value={$t("button.open_download")}
        className="mr-4"
      />
      <Button onClick={onClose} value={$t("button.close")} />
    </div>
  </div>
</Modal>
