<script lang="ts">
  import { t, isLoading as isLoadingI18N } from "svelte-i18n";
  import { Link } from "svelte-navigator";
  import Button from "../components/Button.svelte";
  import Spinner from "../components/Spinner.svelte";
  import { validateForm } from "../utils/util";
  import * as api from "../utils/api";
  import type { IResetPasswordRequest } from "../utils/types";
  import { sectionStore, setNotification, userStore } from "../utils/store";

  let form: HTMLFormElement;
  let submitting = false;
  let username = "";
  let password = "";
  let repeatPw = "";
  let code = "";
  let step = 1;
  let unmatchPw = false;

  sectionStore.set("login");

  const onConfirm = async (e: Event) => {
    e.preventDefault();
    unmatchPw = false;

    if (!validateForm(form)) {
      return;
    }

    if (password !== repeatPw) {
      unmatchPw = true;
      return;
    }

    await sendResetPasswordRequest();
  };

  const sendResetPasswordRequest = async () => {
    submitting = true;
    const uuid = window.location.pathname.split("/reset-password/")[1];
    const req: IResetPasswordRequest = {
      uuid,
      code,
      password,
      username,
    };

    try {
      await api.post("/api/user/reset-password", req, false);
      setNotification("success", $t("message.success.reset_password"));
      step = 2;
      userStore.set(null);
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.reset_password"));
    } finally {
      submitting = false;
    }
  };
</script>

{#if $isLoadingI18N}
  <Spinner />
{:else}
  <div class="relative container-height w-full">
    {#if step === 1}
      <form on:submit={onConfirm} bind:this={form}>
        <div
          class="w-80 center bg-gray-50 shadow rounded-lg flex flex-col items-center p-6"
        >
          <div class="text-xl font-bold mb-8 text-gray-700">
            {$t("component.reset_password.title")}
          </div>
          <div class="w-full grid grid-cols-4 mb-4">
            <div>{$t("form.username")}:</div>
            <div class="col-span-3 ml-2">
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
            <div>{$t("component.reset_password.code")}:</div>
            <div class="col-span-3 ml-2">
              <input
                required
                minLength={4}
                maxLength={16}
                class="ml-2 w-40 border rounded focus:outline-none px-2"
                bind:value={code}
              />
            </div>
          </div>
          <div class="w-full grid grid-cols-4 mb-4">
            <div>{$t("form.password")}:</div>
            <div class="col-span-3 ml-2">
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
          <div class="w-full grid grid-cols-4 mb-4">
            <div>{$t("component.reset_password.repeat")}:</div>
            <div class="col-span-3 ml-2">
              <input
                required
                type="password"
                minLength={6}
                maxLength={16}
                class="ml-2 w-40 border rounded focus:outline-none px-2"
                bind:value={repeatPw}
              />
            </div>
          </div>
          <div class="w-full text-red-400 mb-8">
            {#if unmatchPw}
              {$t("component.reset_password.unmatch")}
            {/if}
          </div>
          <div class="mb-2">
            <Button
              value={submitting
                ? $t("button.confirming")
                : $t("button.confirm")}
              onClick={onConfirm}
              disabled={submitting}
              size="big"
              color="blue"
              type="submit"
            />
          </div>
        </div>
      </form>
    {:else}
      <div
        class="w-80 center bg-gray-50 shadow rounded-lg flex flex-col items-center p-6"
      >
        <div class="text-xl font-bold mb-4 text-gray-700">
          {$t("component.reset_password.title")}
        </div>
        <div>
          {$t("component.reset_password.notice-before")}<Link
            to="/login"
            class="text-blue-400 hover:text-white hover:bg-blue-400"
            >{$t("section.login")}</Link
          >{$t("component.reset_password.notice-after")}
        </div>
      </div>
    {/if}
  </div>
{/if}
