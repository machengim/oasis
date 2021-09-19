<script lang="ts">
  export let value: string;
  export let onClick: (e) => void;
  export let type: "button" | "submit" = "button";
  export let size: "big" | "normal" | "small" = "normal";
  export let color: "blue" | "white" | "gray" = "white";
  export let className: string = "";
  export let disabled: boolean = false;

  const buildStyle = () => {
    let style = "cursor-pointer w-min transition duration-200";
    if (className) {
      style += " " + className;
    }

    switch (size) {
      case "big":
        style += " px-4 py-2 rounded-lg shadow text-lg";
        break;
      case "small":
        style += " px-2 rounded shadow";
        break;
      default:
        style += " px-2 py-1 rounded shadow";
        break;
    }

    switch (color) {
      case "blue":
        style += " bg-blue-400 hover:bg-blue-500 text-white";
        break;
      case "white":
        style += " bg-white hover:bg-gray-200 text-black";
        break;
      case "gray":
        style += " bg-gray-100 hover:bg-blue-200";
        break;
      case "white":
        style += " border-gray-500";
        break;
      default:
        break;
    }

    return style;
  };
</script>

<input {type} {value} {disabled} class={buildStyle()} on:click={onClick} />
