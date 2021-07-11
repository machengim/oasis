<script lang="ts">
  export let value: string;
  export let onClick: (e) => void;
  export let type: "button" | "submit" = "button";
  export let spec: "important" | "normal" | "bold" = "normal";
  export let className: string = "";
  export let disabled: boolean = false;

  const buildStyle = () => {
    let style = "cursor-pointer w-min transition duration-200";
    if (className) {
      style += " " + className;
    }

    switch (spec) {
      case "important":
        style +=
          " bg-blue-400 hover:bg-blue-500 text-white px-4 py-2 rounded-lg shadow";
        break;
      case "bold":
        style += " border bg-gray-200 hover:bg-gray-50 px-2 rounded shadow-sm";
      default:
        style += " border bg-gray-50 hover:bg-gray-100 px-2 rounded shadow-sm";
        break;
    }

    return style;
  };
</script>

<input {type} {value} {disabled} class={buildStyle()} on:click={onClick} />
