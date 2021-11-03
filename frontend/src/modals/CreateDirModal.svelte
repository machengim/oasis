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
      error = "Folder name should not be empty";
      return;
    }

    if (newFolderName.length > 255) {
      error = "Folder name too long";
      return;
    }

    if (files.findIndex((f) => f.filename === newFolderName) >= 0) {
      error = "This name has been used";
      return;
    }

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

      setNotification("success", "New folder created successfully");
    } catch (e) {
      setNotification("error", "New folder create failed");
      console.error(e);
    }

    isLoading = false;
    onClose();
  };
</script>

<Modal {onClose} title="Create Folder">
  <div class="p-4 text-lg">
    <p>
      You are creating a folder inside <b>{parent}</b>. Please enter the new
      folder's name below:
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
