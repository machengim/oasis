<script lang="ts">
  import { t } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import type { IFile } from "../utils/types";
  import { FileType } from "../utils/types";
  import * as api from "../utils/api";
  import { pushFile, setNotification, updateFile } from "../utils/store";
  import { buildEncodeFilePath } from "../utils/util";

  export let onClose = () => {};
  export let dirs: Array<string>;
  export let files: Array<IFile> = [];
  export let contextFile: IFile = null;

  let parent = dirs.join("/") || "storage root";
  let isLoading = false;
  let newFilename = contextFile ? contextFile.filename : "";
  let error: string;
  let title: string;
  let text: string;
  let fileType: string;

  $: if (contextFile) {
    fileType = contextFile.file_type === FileType.Dir ? "folder" : "file";
  }

  $: title = contextFile
    ? "Rename " + fileType
    : $t("modal.create_folder.title");

  $: text = contextFile
    ? "You are renaming " +
      fileType +
      " <b>" +
      contextFile.filename +
      "</b>" +
      ", Please enter the name below:"
    : $t("modal.create_folder.text_before") +
      "<b>" +
      parent +
      "</b>" +
      $t("modal.create_folder.text_after");

  const onConfirm = async () => {
    error = null;

    if (!newFilename) {
      error = $t("modal.create_folder.error_empty");
    } else if (newFilename.length > 255) {
      error = $t("modal.create_folder.error_long");
    } else if (files.findIndex((f) => f.filename === newFilename) >= 0) {
      error = $t("modal.create_folder.error_used");
    }

    if (error) return;

    if (contextFile) {
      await sendRenameRequest();
    } else {
      await sendNewDirRequest();
    }
  };

  const sendNewDirRequest = async () => {
    isLoading = true;

    const parent = encodeURIComponent(dirs.join("/"));
    const payload = { parent, name: newFilename };
    try {
      await api.post("/api/dir", payload, false);
      const newFile: IFile = {
        dir: dirs,
        filename: newFilename,
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

  const sendRenameRequest = async () => {
    isLoading = true;

    const endpoint =
      "/api/file/" + buildEncodeFilePath(dirs, contextFile.filename);
    const payload = {
      new_name: newFilename,
    };

    try {
      await api.put(endpoint, payload, false);
      contextFile.dir = dirs;
      const newFile: IFile = { ...contextFile, filename: newFilename };
      updateFile(contextFile, newFile);
      setNotification("success", "Rename successfully");
    } catch (e) {
      setNotification("error", "Rename failed");
      console.error(e);
    }

    isLoading = false;
    onClose();
  };
</script>

<Modal {onClose} {title}>
  <div class="p-4 text-lg">
    <p>
      {@html text}
    </p>
    <input
      type="text"
      class="mt-2 w-40 lg:w-60 border rounded focus:outline-none px-2"
      bind:value={newFilename}
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
