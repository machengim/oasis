<script lang="ts">
  import { useNavigate } from "svelte-navigator";
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";
  import * as api from "../utils/api";
  import { setNotification } from "../utils/store";

  const navigate = useNavigate();
  export let onClose = () => {};
  let isLoading = false;

  const signOut = async () => {
    isLoading = true;
    try {
      await api.get("/api/signout", "raw");
      setNotification("success", "Sign out successfully,");
      onClose();
      navigate("/login");
    } catch (e) {
      console.error(e);
    }

    isLoading = false;
  };
</script>

<Modal {onClose} title="Sign Out">
  <div class="p-4 text-lg">Are you sure you want to sign out?</div>
  <div class="w-full p-4 flex flex-row justify-end">
    <Button
      onClick={signOut}
      color="blue"
      value="Yes"
      className="mr-4"
      disabled={isLoading}
    />
    <Button onClick={onClose} value="Cancel" />
  </div>
</Modal>
