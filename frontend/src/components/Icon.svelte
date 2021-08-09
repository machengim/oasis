<script lang="ts">
  export let type: "success" | "error" | "profile" | "add";
  export let color: "green" | "yellow" | "red" | "gray" | "black" = "black";

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
      case "profile":
        return `<path d="M258.9,48C141.92,46.42,46.42,141.92,48,258.9,49.56,371.09,140.91,462.44,253.1,464c117,1.6,212.48-93.9,210.88-210.88C462.44,140.91,371.09,49.56,258.9,48ZM385.32,375.25a4,4,0,0,1-6.14-.32,124.27,124.27,0,0,0-32.35-29.59C321.37,329,289.11,320,256,320s-65.37,9-90.83,25.34a124.24,124.24,0,0,0-32.35,29.58,4,4,0,0,1-6.14.32A175.32,175.32,0,0,1,80,259C78.37,161.69,158.22,80.24,255.57,80S432,158.81,432,256A175.32,175.32,0,0,1,385.32,375.25Z"/><path d="M256,144c-19.72,0-37.55,7.39-50.22,20.82s-19,32-17.57,51.93C191.11,256,221.52,288,256,288s64.83-32,67.79-71.24c1.48-19.74-4.8-38.14-17.68-51.82C293.39,151.44,275.59,144,256,144Z"/>`;
      case "add":
        return `<line x1="256" y1="112" x2="256" y2="400" style="fill:none;stroke:white;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="400" y1="256" x2="112" y2="256" style="fill:none;stroke:white;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
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
  .fill-gray {
    fill: rgb(107, 114, 128);
  }
  .fill-black {
    fill: black;
  }
  .fill-white {
    fill: white;
  }
</style>
