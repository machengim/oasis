<script lang="ts">
  import { t } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import type { IFile } from "../utils/types";
  import { EFileType } from "../utils/enums";
  import { buildEncodeFilePath } from "../utils/util";
  import { deleteFile, setNotification } from "../utils/store";
  import * as api from "../utils/api";

  export let onClose: () => void;
  export let dirs: Array<string>;
  export let contextFile: IFile;
  let fileType =
    contextFile.file_type === EFileType.Dir
      ? $t("common.folder")
      : $t("common.file");
  let isLoading = false;
  let title = $t("modal.delete_file.title") + fileType;

  const onConfirm = async () => {
    isLoading = true;

    const endpoint =
      "/api/file/" + buildEncodeFilePath(dirs, contextFile.filename);
    try {
      await api.remove(endpoint, null, false);
      contextFile.dir = dirs;
      deleteFile(contextFile);
      setNotification("success", $t("message.success.delete_file"));
    } catch (e) {
      setNotification("error", $t("message.error.delete_file"));
      console.error(e);
    }

    isLoading = false;
    onClose();
  };
</script>

<Modal {onClose} {title}>
  <div class="p-4 text-lg">
    {$t("modal.delete_file.text_before")}{fileType}
    <b>{contextFile.filename}</b>{$t("modal.delete_file.text_after")}
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
