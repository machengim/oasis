<script lang="ts">
  import { t, isLoading as isLoadingI18N } from "svelte-i18n";
  import { useFocus } from "svelte-navigator";
  import Button from "../components/Button.svelte";
  import * as api from "../utils/api";
  import {
    setNotification,
    sectionStore,
    userStore,
    resetTitle,
  } from "../utils/store";
  import { useNavigate } from "svelte-navigator";
  import { validateForm } from "../utils/util";
  import type { ILoginRequest, IUser } from "../utils/types";
  import Spinner from "../components/Spinner.svelte";

  const navigate = useNavigate();
  const focus = useFocus();
  let username = "";
  let password = "";
  let isLoading = false;
  let form: HTMLFormElement;

  sectionStore.set("login");
  resetTitle();

  const onConfirm = async (e: Event) => {
    e.preventDefault();

    if (!validateForm(form)) {
      return;
    }

    await sendLoginRequest();
  };

  const sendLoginRequest = async () => {
    const payload: ILoginRequest = {
      username,
      password,
    };

    isLoading = true;
    try {
      const user: IUser = await api.post("/api/login", payload, true);
      userStore.set(user);
      setNotification("success", $t("message.success.login"));
      navigate("/files");
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.login_fail"));
    }
    isLoading = false;
  };
</script>

{#if $isLoadingI18N}
  <Spinner />
{:else}
  <div class="absolute w-full">
    <form on:submit={onConfirm} bind:this={form}>
      <div
        class="w-80 mx-auto mt-28 bg-gray-50 shadow rounded-lg flex flex-col items-center p-8"
      >
        <div class="text-xl font-bold mb-8 text-gray-700">
          {$t("component.login.title")}
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
              use:focus
            />
          </div>
        </div>
        <div class="w-full grid grid-cols-4 mb-8">
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

        <div class="mb-2">
          <Button
            value={isLoading ? $t("button.logging") : $t("button.login")}
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
{/if}
