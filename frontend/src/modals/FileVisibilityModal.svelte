<script lang="ts">
  import { t } from "svelte-i18n";
  import Button from "../components/Button.svelte";
  import Modal from "../components/Modal.svelte";
  import type { IFile } from "../utils/types";
  import { buildEncodeFilePath } from "../utils/util";
  import * as api from "../utils/api";
  import { setNotification, updateFile } from "../utils/store";
  import { EFileType } from "../utils/enums";

  export let onClose: () => void;
  export let dirs: Array<string>;
  export let contextFile: IFile;
  const action = contextFile.least_permission === 0 ? "hide" : "show";
  const isDir = contextFile.file_type === EFileType.Dir;
  let isLoading = false;

  const buildTitle = () => {
    return action === "show"
      ? $t("component.context_menu.show")
      : $t("component.context_menu.hide");
  };

  const onConfirm = async () => {
    isLoading = true;
    const endpoint =
      "/api/file/" +
      buildEncodeFilePath(dirs, contextFile.filename) +
      "/visibility";
    const visible = action === "show";
    try {
      await api.put(endpoint, { visible }, false);

      contextFile.dir = dirs.join("/") || "/";
      const newFile: IFile = {
        ...contextFile,
        least_permission: visible ? 0 : 1,
      };
      updateFile(contextFile, newFile);
      setNotification("success", "Visibility set successfully");
      onClose();
    } catch (e) {
      setNotification("error", "Visibility set failed");
      console.error(e);
    } finally {
      isLoading = false;
    }
  };
</script>

<Modal {onClose} size="small" title={buildTitle()}>
  <div class="p-4 text-lg">
    {$t("modal.visibility_set.text_before")}{action === "hide"
      ? $t("modal.visibility_set.hiding")
      : $t("modal.visibility_set.showing")}{isDir
      ? $t("common.folder")
      : $t("common.file")}
    <b>{contextFile.filename}</b>
    {$t("modal.visibility_set.text_after")}
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
