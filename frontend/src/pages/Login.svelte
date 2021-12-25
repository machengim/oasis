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
    siteStore,
  } from "../utils/store";
  import { useNavigate } from "svelte-navigator";
  import { validateForm } from "../utils/util";
  import type { ILoginRequest, IUser } from "../utils/types";
  import Spinner from "../components/Spinner.svelte";

  const navigate = useNavigate();
  const focus = useFocus();
  const allow_guest = $siteStore.allow_guest;
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
    } finally {
      isLoading = false;
    }
  };

  const sendGuestLoginRequest = async () => {
    isLoading = true;
    try {
      const guest: IUser = await api.get("/api/login/guest", "json");
      userStore.set(guest);
      console.log(guest);
      setNotification("success", $t("message.success.login"));
      navigate("/files");
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.login_fail"));
    } finally {
      isLoading = false;
    }
  };
</script>

{#if $isLoadingI18N}
  <Spinner />
{:else}
  <div class="relative container-height w-full">
    <form on:submit={onConfirm} bind:this={form}>
      <div
        class="w-80 center bg-gray-50 shadow rounded-lg flex flex-col items-center p-8"
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
        {#if allow_guest}
          <div
            class="w-full flex flex-row justify-end"
            style="margin-bottom: -1rem;"
          >
            <div
              class="hover:bg-blue-400 hover:text-white cursor-pointer rounded px-1"
              on:click={sendGuestLoginRequest}
            >
              {$t("component.login.guest_login")}
            </div>
          </div>
        {/if}
      </div>
    </form>
  </div>
{/if}
