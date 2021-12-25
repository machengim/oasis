<script lang="ts">
  import { useNavigate } from "svelte-navigator";
  import { t } from "svelte-i18n";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import * as api from "../utils/api";
  import { setNotification, userStore, uploadTaskStore } from "../utils/store";
  import Spinner from "../components/Spinner.svelte";
  import { cancelUploads } from "../utils/upload";

  const navigate = useNavigate();
  export let onClose = () => {};
  let isLoading = false;

  const signOut = async () => {
    isLoading = true;

    try {
      const uploadingTasks = $uploadTaskStore;
      if (uploadingTasks.length > 0) {
        await cancelUploads(uploadingTasks);
        uploadTaskStore.set([]);
      }
      await api.get("/api/user/signout", "raw");
      userStore.set(null);
      setNotification("success", $t("message.success.signout"));
      onClose();
      navigate("/login");
    } catch (e) {
      console.error(e);
      setNotification("error", $t("message.error.signout_fail"));
    }

    isLoading = false;
  };
</script>

<Modal {onClose} size="small" title={$t("modal.signout.title")}>
  {#if isLoading}
    <Spinner />
  {:else}
    <div class="p-4 text-lg">{$t("modal.signout.notice")}</div>
    <div class="w-full p-4 flex flex-row justify-end">
      <Button
        onClick={signOut}
        color="blue"
        value={isLoading ? $t("button.confirming") : $t("button.confirm")}
        className="mr-4"
        disabled={isLoading}
      />
      <Button onClick={onClose} value={$t("button.cancel")} />
    </div>
  {/if}
</Modal>
