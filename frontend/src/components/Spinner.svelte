<script lang="ts">
  export let text = "Loading..";
</script>

<div
  class="mt-20 transition ease-in-out duration-150 flex flex-row items-center justify-center"
>
  <svg
    class="animate-spin mr-3"
    width="1.5rem"
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
  >
    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" />
    <path
      class="opacity-75"
      fill="currentColor"
      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
    />
  </svg>
  <span>{text}</span>
</div>
