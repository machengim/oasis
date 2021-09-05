<script lang="ts">
  export let toggleCheck: (b: boolean) => void;
  export let defaultCheck = false;
  let isChecked = defaultCheck;

  const onToggleCheck = () => {
    isChecked = !isChecked;
    toggleCheck(isChecked);
  };
</script>

<div>
  <label class="switch">
    <input type="checkbox" on:change={onToggleCheck} checked={isChecked} />
    <span class="slider round" />
  </label>
</div>

<style>
  .switch {
    position: relative;
    display: inline-block;
    width: 2rem;
    height: 1rem;
    vertical-align: middle;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #ccc;
    -webkit-transition: 0.4s;
    transition: 0.4s;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 1rem;
    width: 1rem;
    left: 0;
    bottom: 0;
    background-color: white;
    -webkit-transition: 0.4s;
    transition: 0.4s;
  }

  input:checked + .slider {
    background-color: #2196f3;
  }

  input:focus + .slider {
    box-shadow: 0 0 1px #2196f3;
  }

  input:checked + .slider:before {
    -webkit-transform: translateX(1rem);
    -ms-transform: translateX(1rem);
    transform: translateX(1rem);
  }

  .slider.round {
    border-radius: 1rem;
  }

  .slider.round:before {
    border-radius: 50%;
  }
</style>
