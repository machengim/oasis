<script lang="ts">
  import Button from "../components/Button.svelte";
  import DirBrowser from "../sections/DirBrowser.svelte";
  import * as api from "../utils/api";
  import type { ISetupRequest } from "../utils/types";
  import { setNotification } from "../utils/store";
  import { useNavigate } from "svelte-navigator";
  const navigate = useNavigate();

  let username = "";
  let password = "";
  let selectedDir = "";
  let form: HTMLFormElement;
  let isLoading = false;
  let isNoStorageError = false;
  let isOpenDirBrowser = false;

  const onConfirm = async (e: Event) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    isLoading = true;
    const result = await sendSetupRequest();
    isLoading = false;

    if (result) {
      navigate("/login");
    }
  };

  const validateForm = (): boolean => {
    if (!form.checkValidity()) {
      form.reportValidity();
      return false;
    }

    if (!selectedDir || selectedDir.trim().length === 0) {
      isNoStorageError = true;
      return false;
    }

    return true;
  };

  const sendSetupRequest = async (): Promise<boolean> => {
    const payload: ISetupRequest = {
      username,
      password,
      storage: encodeURIComponent(selectedDir),
    };

    try {
      await api.post("/api/sys/setup", payload, false);
      setNotification("success", "Launched successfully.");
    } catch (e) {
      console.error(e);
      if (e.message === "409") {
        setNotification("error", "Username existed.");
      } else {
        setNotification("error", "Launch failed");
      }
      return false;
    }

    return true;
  };
</script>

<div class="absolute w-full">
  {#if isOpenDirBrowser}
    <DirBrowser
      onClose={() => (isOpenDirBrowser = false)}
      onSelect={(v) => {
        selectedDir = v;
        isNoStorageError = false;
      }}
    />
  {/if}
  <form on:submit={onConfirm} bind:this={form}>
    <div
      class="w-96 mx-auto mt-28 bg-gray-50 shadow rounded-lg flex flex-col items-center p-8"
    >
      <div class="text-xl font-bold mb-8 text-gray-700">Server Setup</div>
      <div class="w-full grid grid-cols-4 mb-4">
        <div>username:</div>
        <div class="col-span-3">
          <input
            required
            minLength={1}
            maxLength={16}
            class="ml-2 w-40 border rounded focus:outline-none px-2"
            bind:value={username}
          />
        </div>
      </div>
      <div class="w-full grid grid-cols-4 mb-4">
        <div>password:</div>
        <div class="col-span-3">
          <input
            required
            type="password"
            minLength={6}
            maxLength={16}
            class="ml-2 w-40 border rounded focus:outline-none px-2"
            bind:value={password}
          />
        </div>
      </div>
      <div class="w-full grid grid-cols-4 mb-12">
        <div>storage:</div>
        <div class="col-span-3 pl-2">
          {#if selectedDir}
            <Button
              value="Change"
              size="small"
              onClick={() => (isOpenDirBrowser = true)}
            />
            <div class="mt-2 break-words">{selectedDir}</div>
          {:else}
            <Button
              value="Select"
              size="small"
              onClick={() => (isOpenDirBrowser = true)}
            />
          {/if}
          {#if isNoStorageError}
            <div class="text-red-500">Please choose a storage</div>
          {/if}
        </div>
      </div>
      <div class="mb-2">
        <Button
          value={isLoading ? "Lauching..." : "Launch"}
          onClick={onConfirm}
          disabled={isLoading}
          size="big"
          color="blue"
          type="submit"
        />
      </div>
    </div>
  </form>
</div>
