<script lang="ts">
  import Modal from "../components/Modal.svelte";
  import Button from "../components/Button.svelte";

  export let onClose: () => void;
  export let setResult: (r: boolean) => void;
  export let title = "";
  export let text = "";

  const onConfirm = () => {
    setResult(true);
    onClose();
  };
</script>

<Modal {onClose} {title}>
  <div class="p-4 text-lg">{@html text}</div>
  <div class="w-full p-4 flex flex-row justify-end">
    <Button onClick={onConfirm} color="blue" value="Yes" className="mr-4" />
    <Button onClick={onClose} value="No" />
  </div>
</Modal>
