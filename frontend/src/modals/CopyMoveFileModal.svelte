<script lang="ts">
  import { t } from "svelte-i18n";
  import Button from "../components/Button.svelte";
  import Modal from "../components/Modal.svelte";
  import { EFileType } from "../utils/enums";
  import type { ICopyMoveTask, IFile } from "../utils/types";
  import { buildEncodeFilePath, sleep } from "../utils/util";
  import * as api from "../utils/api";
  import { deleteFile } from "../utils/store";

  export let onClose: () => void;
  export let source: IFile;
  export let target: string;
  export let sourceDirs: string[];
  export let mode: "copy" | "move";
  let isLoading = false;
  let step = 1;
  let progress = 0;
  let done = false;
  let error = false;
  let overwrite = $t("modal.copy_move.no");

  const onConfirm = async () => {
    isLoading = true;
    step = 2;
    const payload = {
      source: buildEncodeFilePath(sourceDirs, source.filename),
      target: buildEncodeFilePath([], target),
      is_copy: mode === "copy",
      overwrite: overwrite === $t("modal.copy_move.yes"),
    };

    try {
      const uuid: string = await api.post(
        `/api/file/copy-move`,
        payload,
        false
      );
      const endpoint = `/api/file/copy-move-status/${uuid}`;
      let task: ICopyMoveTask = await api.get(endpoint, "json");
      while (task.status !== "Success" && task.status !== "Failed") {
        progress = task.progress;
        await sleep(500);
        task = await api.get(endpoint, "json");
      }

      if (task.status === "Success") {
        progress = 1.0;
        done = true;
      } else {
        throw new Error("task failed");
      }

      if (mode === "move") {
        source.dir = sourceDirs.join("/") || "/";
        deleteFile(source);
      }
    } catch (e) {
      console.error(e);
      error = true;
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
  {#if step === 1}
    <div class="p-4 text-lg">
      <div>
        {$t("modal.copy_move.text1")}{mode === "copy"
          ? $t("modal.copy_move.copying")
          : $t("modal.copy_move.moving")}{source.file_type === EFileType.Dir
          ? $t("common.folder")
          : $t("common.file")}
        <b>{source.filename}</b>{$t("modal.copy_move.text2")}
        <b>{target || "/"}</b>
      </div>
      <div class="mt-4">
        {$t("modal.copy_move.text3")}
      </div>
      <select bind:value={overwrite} class="w-20 px-2 border bg-gray-50">
        <option>{$t("modal.copy_move.no")}</option>
        <option>{$t("modal.copy_move.yes")}</option>
      </select>
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
  {:else}
    <div class="p-4 text-lg">
      <div>
        {$t(`modal.copy_move.${mode}`)}<b>{source.filename}</b>{$t(
          `modal.copy_move.to`
        )}<b>{target}</b>
      </div>
      <div>
        {$t("modal.copy_move.progress")}<b>{(progress * 100).toFixed(2)}%</b>
      </div>
      {#if done}
        <div class="mt-4">{$t("modal.copy_move.done_text")}</div>
      {:else if error}
        <div class="mt-4 text-red-500">{$t("modal.copy_move.failed_text")}</div>
      {/if}
    </div>
    <div class="w-full p-4 flex flex-row justify-end">
      <Button
        onClick={onClose}
        disabled={!done && !error}
        color="white"
        value={done || error ? "Close" : "Processing"}
      />
    </div>
  {/if}
</Modal>
