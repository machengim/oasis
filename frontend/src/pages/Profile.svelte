<script lang="ts">
  import { useNavigate, useFocus } from "svelte-navigator";
  import { t } from "svelte-i18n";
  import {
    setNotification,
    sectionStore,
    getUsername,
    resetTitle,
  } from "../utils/store";
  import Button from "../components/Button.svelte";
  import * as api from "../utils/api";
  import type { IChangePasswordRequest } from "../utils/types";

  const navigate = useNavigate();
  const focus = useFocus();
  let username = getUsername();
  let isLoading = false;
  let old_password = "";
  let new_password = "";
  let form: HTMLFormElement;
  let isMissingPassword = false;

  sectionStore.set("profile");
  resetTitle();

  const onConfirm = async (e: Event) => {
    isMissingPassword = false;
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    if (!old_password && !new_password) {
      return;
    }
    if (!old_password || !new_password) {
      isMissingPassword = true;
    }

    isLoading = true;
    try {
      await sendChangePasswordRequest();
      setNotification("success", $t("message.success.update_password"));
      navigate("/login");
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.update_password_fail"));
    }

    isLoading = false;
  };

  const validateForm = (): boolean => {
    if (!form.checkValidity()) {
      form.reportValidity();
      return false;
    }

    return true;
  };

  const sendChangePasswordRequest = async () => {
    const payload: IChangePasswordRequest = {
      username,
      old_password,
      new_password,
    };
    const endpoint = "/api/user/password";

    try {
      await api.put(endpoint, payload, false);
    } catch (e) {
      throw e;
    }
  };

  const onCancel = () => {
    navigate(-1);
  };
</script>

<div class="relative container-height w-full">
  <form on:submit={onConfirm} bind:this={form}>
    <div
      class="w-100 mx-auto center bg-gray-50 shadow rounded-lg flex flex-col items-center p-2 md:p-8"
    >
      <div class="text-xl font-bold mb-8 text-gray-700">
        {$t("component.profile.title")}
      </div>
      <div class="w-full grid grid-cols-3 mb-4">
        <div class="text-right">{$t("form.username")}:</div>
        <div class="col-span-2">
          <span class="ml-4 px-2">{username}</span>
        </div>
      </div>
      <div class="w-full grid grid-cols-3 mb-4">
        <div class="text-right">{$t("form.old_password")}:</div>
        <div class="col-span-2">
          <input
            required={false}
            type="password"
            minLength={6}
            maxLength={16}
            class="ml-4 w-40 border rounded focus:outline-none px-2"
            bind:value={old_password}
            use:focus
          />
        </div>
      </div>
      <div class="w-full grid grid-cols-3 mb-2">
        <div class="text-right">{$t("form.new_password")}:</div>
        <div class="col-span-2">
          <input
            required={false}
            type="password"
            minLength={6}
            maxLength={16}
            class="ml-4 w-40 border rounded focus:outline-none px-2"
            bind:value={new_password}
          />
        </div>
      </div>
      {#if isMissingPassword}
        <div class="text-red-500">{$t("message.error.miss_password")}</div>
      {/if}
      <div class="mt-8 mb-2 flex justify-center">
        <Button
          value={isLoading ? $t("button.confirming") : $t("button.confirm")}
          onClick={onConfirm}
          disabled={isLoading}
          size="big"
          color="blue"
          type="submit"
          className="mr-8"
        />
        <Button value={$t("button.cancel")} onClick={onCancel} size="big" />
      </div>
    </div>
  </form>
</div>

<style>
  .w-100 {
    width: 25rem;
    max-width: 90vw;
  }
</style>
