<script lang="ts">
  import { onDestroy } from "svelte";
  import Button from "../components/Button.svelte";
  import Modal from "../components/Modal.svelte";
  import { validateForm } from "../utils/util";
  import { dirsStore, fileActionStore, setNotification } from "../utils/store";
  import * as api from "../utils/api";
  import { subscribe } from "svelte/internal";
  import type { IFile, IFileAction } from "../utils/types";

  export let onClose: () => void;
  let dirs: Array<string>;
  let form: HTMLFormElement;
  let folderName = "";
  let isSaving = false;
  // let pwd = +localStorage.getItem("root_dir");

  // const unsubscribePwd = subscribe(pwdStore, (value) => {
  //   if (value > 0 && value !== pwd) {
  //     pwd = value;
  //   }
  // });

  const unsubscribeDirs = subscribe(dirsStore, (value) => {
    dirs = value;
  });

  onDestroy(() => {
    unsubscribeDirs();
  });

  const onSubmit = async (e: Event) => {
    e.preventDefault();

    if (!validateForm(form)) {
      return;
    }

    await sendCreateFolderRequest();
  };

  const sendCreateFolderRequest = async () => {
    const payload = {
      paths: dirs,
      dir_name: folderName,
    };

    try {
      const result: IFile = await api.post("/api/dir", payload, true);
      // completeFileStore.set(result);
      const CreateDirAction: IFileAction = { action: "complete", file: result };
      fileActionStore.set(CreateDirAction);
      onClose();
    } catch (e) {
      console.error(e);
      setNotification("error", "Dir " + folderName + " created failed");
    }
  };
</script>

<Modal title="Creating Folder" {onClose}>
  <form on:submit={onSubmit} bind:this={form}>
    <div class="p-4 flex flex-col">
      <span>Please enter the folder name:</span>
      <input
        required
        minlength={1}
        maxlength={128}
        class="w-60 border rounded focus:outline-none focus:border-blue-400 px-2 mt-2"
        bind:value={folderName}
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
