<script lang="ts">
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import { validateForm } from "../utils/util";
  import type { IFile } from "../utils/types";
  import * as api from "../utils/api";
  import { setNotification, fileActionStore } from "../utils/store";

  export let onClose: () => void;
  export let selectedFile: IFile;
  let form: HTMLFormElement;
  let filename = selectedFile && selectedFile.filename;
  let isSaving = false;

  const onSubmit = async (e: Event) => {
    e.preventDefault();

    if (!validateForm(form)) {
      return;
    }

    await sendRenameFileRequest();
  };

  const sendRenameFileRequest = async () => {
    const payload = {
      filename,
    };

    isSaving = true;

    try {
      const endpoint = `/api/file/${selectedFile.file_id}`;
      await api.put(endpoint, payload, false);
      selectedFile.filename = filename;
      fileActionStore.set({ action: "modify", file: selectedFile });
      onClose();
    } catch (e) {
      console.error(e);
      setNotification("error", `Rename file ${selectedFile.filename} failed`);
    }

    isSaving = false;
  };
</script>

<Modal title="Rename file/folder" {onClose}>
  <form on:submit={onSubmit} bind:this={form}>
    <div class="p-4 flex flex-col">
      <span>Please enter a new name:</span>
      <input
        required
        minlength={1}
        maxlength={128}
        class="w-60 border rounded focus:outline-none focus:border-blue-400 px-2 mt-2"
        bind:value={filename}
      />
    </div>
    <div class="border-t border-gray-200 mt-4 p-4 flex flex-row justify-end">
      <Button value="Cancel" onClick={onClose} />
      <Button
        value="Submit"
        color="blue"
        className="ml-4"
        onClick={onSubmit}
        disabled={isSaving}
      />
    </div>
  </form>
</Modal>
