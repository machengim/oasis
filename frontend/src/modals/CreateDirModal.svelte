<script lang="ts">
  import { t } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import type { IFile } from "../utils/types";
  import { FileType } from "../utils/types";
  import * as api from "../utils/api";
  import { pushFile, setNotification } from "../utils/store";

  export let onClose = () => {};
  export let dirs: Array<string>;
  export let files: Array<IFile> = [];

  let parent = dirs.join("/") || "storage root";
  let isLoading = false;
  let newFolderName: string;
  let error: string;

  const onConfirm = async () => {
    error = null;

    if (!newFolderName) {
      error = $t("modal.create_folder.error_empty");
    } else if (newFolderName.length > 255) {
      error = $t("modal.create_folder.error_long");
    } else if (files.findIndex((f) => f.filename === newFolderName) >= 0) {
      error = $t("modal.create_folder.error_used");
    }

    if (error) return;

    await sendNewDirRequest();
  };

  const sendNewDirRequest = async () => {
    isLoading = true;

    const parent = encodeURIComponent(dirs.join("/"));
    const payload = { parent, name: newFolderName };
    try {
      await api.post("/api/dir", payload, false);
      const newFile: IFile = {
        dir: dirs,
        filename: newFolderName,
        file_type: FileType.Dir,
        size: 0,
      };
      pushFile(newFile);

      setNotification("success", $t("message.success.create_dir"));
    } catch (e) {
      setNotification("error", $t("message.error.create_dir"));
      console.error(e);
    }

    isLoading = false;
    onClose();
  };
</script>

<Modal {onClose} title={$t("modal.create_folder.title")}>
  <div class="p-4 text-lg">
    <p>
      {$t("modal.create_folder.text_before")} <b>{parent}</b>
      {$t("modal.create_folder.text_after")}
    </p>
    <input
      type="text"
      class="mt-2 w-40 lg:w-60 border rounded focus:outline-none px-2"
      bind:value={newFolderName}
    />
    <p class="text-sm text-red-500 {error ? 'visible' : 'invisible'}">
      {error}
    </p>
  </div>

  <div class="w-full p-4 flex flex-row justify-end">
    <Button
      onClick={onConfirm}
      color="blue"
      value={isLoading ? $t("button.confirming") : $t("button.confirm")}
      className="mr-4"
      disabled={isLoading}
    />
    <Button onClick={onClose} value={$t("button.cancel")} />
  </div>
</Modal>
