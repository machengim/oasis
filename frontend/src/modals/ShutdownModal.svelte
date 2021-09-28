<script lang="ts">
  import { useNavigate } from "svelte-navigator";
  import { t } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import * as api from "../utils/api";
  import { setNotification } from "../utils/store";
  import Spinner from "../components/Spinner.svelte";

  const navigate = useNavigate();
  export let onClose = () => {};
  let isLoading = false;

  const shutdown = async () => {
    isLoading = true;
    try {
      await api.get("/shutdown", "raw");
      setNotification("success", $t("message.success.goodbye"));
      onClose();
      navigate("/login");
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.success.shutdown_error"));
    }

    isLoading = false;
  };
</script>

<Modal {onClose} title={$t("modal.shutdown.title")}>
  {#if isLoading}
    <Spinner />
  {:else}
    <div class="p-4 text-lg">{$t("modal.shutdown.notice")}</div>
    <div class="w-full p-4 flex flex-row justify-end">
      <Button
        onClick={shutdown}
        color="blue"
        value={isLoading ? $t("button.confirming") : $t("button.confirm")}
        className="mr-4"
        disabled={isLoading}
      />
      <Button onClick={onClose} value={$t("button.cancel")} />
    </div>
  {/if}
</Modal>
