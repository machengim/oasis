<script lang="ts">
  import { t } from "svelte-i18n";
  import Button from "../components/Button.svelte";
  import { validateForm } from "../utils/util";
  import * as api from "../utils/api";
  import { sectionStore, setNotification } from "../utils/store";

  let form: HTMLFormElement;
  let username = "";
  let submitting = false;
  let filename = "";
  let step = 1;

  sectionStore.set("login");

  const onConfirm = async (e: Event) => {
    e.preventDefault();

    if (!validateForm(form)) {
      return;
    }

    await sendForgotPasswordRequest();
  };

  const sendForgotPasswordRequest = async () => {
    submitting = true;
    const url = window.location.protocol + "//" + window.location.host;

    const payload = {
      url,
      username,
    };

    try {
      filename = await api.post("/api/user/forgot-password", payload, false);
      step = 2;
    } catch (err) {
      console.error(err);
      setNotification("error", $t("message.error.forgot_password"));
    } finally {
      submitting = false;
    }
  };
</script>

<div class="relative container-height w-full">
  {#if step === 1}
    <form on:submit={onConfirm} bind:this={form}>
      <div
        class="w-80 center bg-gray-50 shadow rounded-lg flex flex-col items-center p-6"
      >
        <div class="text-xl font-bold mb-8 text-gray-700">
          {$t("component.forgot_password.title")}
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
        <Button
          type="submit"
          value={submitting ? $t("button.confirming") : $t("button.confirm")}
          size="big"
          color="blue"
          className="my-2"
          disabled={submitting}
          onClick={onConfirm}
        />
      </div>
    </form>
  {:else if step === 2 && filename}
    <div
      class="w-80 center bg-gray-50 shadow rounded-lg flex flex-col items-center p-6"
    >
      <div class="text-xl font-bold mb-4 text-gray-700">
        {$t("component.forgot_password.title")}
      </div>
      <div>
        <p>
          {$t("component.forgot_password.notice-p1")}
          <b>data/temp/{filename}.txt</b>.
        </p>
        <p class="mt-4">
          {$t("component.forgot_password.notice-p2")}
        </p>
      </div>
    </div>
  {/if}
</div>
