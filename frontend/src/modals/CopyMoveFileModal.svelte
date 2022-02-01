<script lang="ts">
  import { t } from "svelte-i18n";
  import Button from "../components/Button.svelte";
  import Modal from "../components/Modal.svelte";
  import { EFileType } from "../utils/enums";
  import type { IFile } from "../utils/types";
  import { buildEncodeFilePath } from "../utils/util";
  import * as api from "../utils/api";
  import { setNotification } from "../utils/store";

  export let onClose: () => void;
  export let source: IFile;
  export let target: string;
  export let sourceDirs: string[];
  export let mode: "copy" | "move";
  let isLoading = false;

  const onConfirm = async () => {
    isLoading = true;
    const payload = {
      source: buildEncodeFilePath(sourceDirs, source.filename),
      target: buildEncodeFilePath([], target),
    };

    try {
      await api.post("/api/file/copy", payload, false);
      setNotification("success", $t("message.success.copy_file"));
      onClose();
    } catch (e) {
      setNotification("error", $t("message.error.copy_file"));
      console.error(e);
    } finally {
      isLoading = false;
    }
  };
</script>

<Modal
  {onClose}
  title={`${$t(`modal.copy_move.${mode}`)}${
    source.file_type === EFileType.Dir ? $t("common.folder") : $t("common.file")
  }`}
>
  <div class="p-4 text-lg">
    {$t("modal.copy_move.text1")}{mode === "copy"
      ? $t("modal.copy_move.copying")
      : $t("modal.copy_move.moving")}{source.file_type === EFileType.Dir
      ? $t("common.folder")
      : $t("common.file")}
    <b>{source.filename}</b>{$t("modal.copy_move.text2")}
    <b>{target || "/"}</b>{$t("modal.copy_move.text3")}
  </div>

  <div class="w-full p-4 flex flex-row justify-end">
    <Button
      onClick={onConfirm}
      color="blue"
      value={isLoading ? $t("button.confirming") : $t("button.confirm")}
      className="mr-4"
      disabled={isLoading}
    />
    <Button
      onClick={onClose}
      disabled={isLoading}
      value={$t("button.cancel")}
    />
  </div>
</Modal>
