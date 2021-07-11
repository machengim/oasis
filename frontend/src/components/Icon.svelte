<script lang="ts">
  export let type: "success" | "error";
  export let color: "green" | "yellow" | "red" = "green";

  const buildStyle = () => {
    let style = "w-8 h-8";
    if (color) {
      style += " fill-" + color;
    }

    return style;
  };

  const getIcon = () => {
    switch (type) {
      case "success":
        return `<path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM364.25,186.29l-134.4,160a16,16,0,0,1-12,5.71h-.27a16,16,0,0,1-11.89-5.3l-57.6-64a16,16,0,1,1,23.78-21.4l45.29,50.32L339.75,165.71a16,16,0,0,1,24.5,20.58Z" />`;
      case "error":
        return `<path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm86.63,272L320,342.63l-64-64-64,64L169.37,320l64-64-64-64L192,169.37l64,64,64-64L342.63,192l-64,64Z" />`;
      default:
        return null;
    }
  };
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  class={buildStyle()}
  viewBox="0 0 512 512"
>
  {@html getIcon()}
</svg>

<style>
  .fill-green {
    fill: rgb(52, 211, 153);
  }
  .fill-red {
    fill: rgb(248, 113, 113);
  }
</style>
