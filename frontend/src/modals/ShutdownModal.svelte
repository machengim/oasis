<script lang="ts">
  import { useNavigate } from "svelte-navigator";
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
      setNotification("success", "Goodbye!");
      onClose();
      navigate("/login");
    } catch (e) {
      console.error(e);
      setNotification("error", "Shutdown server failed");
    }

    isLoading = false;
  };
</script>

<Modal {onClose} title="Shutdown">
  {#if isLoading}
    <Spinner />
  {:else}
    <div class="p-4 text-lg">Are you sure you want to shutdown the server?</div>
    <div class="w-full p-4 flex flex-row justify-end">
      <Button
        onClick={shutdown}
        color="blue"
        value="Confirm"
        className="mr-4"
        disabled={isLoading}
      />
      <Button onClick={onClose} value="Cancel" />
    </div>
  {/if}
</Modal>
