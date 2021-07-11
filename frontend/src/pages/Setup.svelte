<script lang="ts">
  import Button from "../components/Button.svelte";
  import DirBrowser from "../components/DirBrowser.svelte";

  let username = "";
  let password = "";
  let selectedDir = "";
  let form: HTMLFormElement;
  let isNoStorageError = false;
  let isOpenDirBrowser = false;

  const onConfirm = (e: Event) => {
    e.preventDefault();

    if (!form.checkValidity()) {
      form.reportValidity();
      return;
    }

    if (!selectedDir || selectedDir.trim().length === 0) {
      isNoStorageError = true;
      return false;
    }

    alert("submitted!");
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
      class="w-96 mx-auto mt-40 bg-gray-50 shadow rounded-lg flex flex-col items-center p-8 overflow-hidden"
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
              value={"Change"}
              onClick={() => (isOpenDirBrowser = true)}
            />
            <div class="mt-2 break-words">{selectedDir}</div>
          {:else}
            <Button
              value={"Select"}
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
          value="launch"
          onClick={onConfirm}
          spec="important"
          type="submit"
        />
      </div>
    </div>
  </form>
</div>
