<script lang="ts">
  import { onDestroy } from "svelte";
  import Button from "../components/Button.svelte";
  import Modal from "../components/Modal.svelte";
  import { validateForm } from "../utils/util";
  import { pwdStore } from "../utils/store";
  import * as api from "../utils/api";
  import { subscribe } from "svelte/internal";

  export let onClose: () => void;
  let form: HTMLFormElement;
  let folderName = "";
  let isSaving = false;
  let pwd = +localStorage.getItem("root_dir");

  const unsubscribePwd = subscribe(pwdStore, (value) => {
    if (value > 0 && value !== pwd) {
      pwd = value;
    }
  });

  onDestroy(() => {
    unsubscribePwd();
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
      parent_id: pwd,
      folder_name: folderName,
    };

    console.log("payload: ", payload);
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
