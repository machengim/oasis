<script lang="ts">
  import type { EIconType } from "../utils/enums";
  import { EIconColor } from "../utils/enums";

  export let type: EIconType;
  export let color: EIconColor = EIconColor.gray;
  export let hoverColor: EIconColor = null;
  export let size: "small" | "tiny" | "default" | "large" | "huge" = "default";
  export let className: string = "";
  export let onClick: (e) => void = null;
  let iconColor = "#000";
  let iconHtml: string = null;
  let svgClass = "";

  $: if (color) {
    iconColor = convertColor(color);
  }

  $: if (iconColor) {
    iconHtml = getIcon();
  }

  $: if (type && iconColor) {
    svgClass = buildStyle();
  }

  const buildStyle = () => {
    let style: string;
    switch (size) {
      case "small":
        style = " w-6 h-6";
        break;
      case "tiny":
        style = "w-4 h-4";
        break;
      case "large":
        style = "w-12 h-12";
        break;
      case "huge":
        style = "w-16 h-16";
        break;
      default:
        style = "w-8 h-8";
    }

    if (color) {
      style += " fill-" + color;
    }

    return style;
  };

  const convertColor = (color: EIconColor) => {
    switch (color) {
      case "green":
        return "#34D399";
      case "blue":
        return "#60a5fa";
      case "red":
        return "#EF4444";
      case "yellow":
        return "#FBBF24";
      case "gray":
        return "#9CA3AF";
      case "pink":
        return "#F472B6";
      default:
        return color;
    }
  };

  const changeToHoverColor = () => {
    if (hoverColor) {
      iconColor = convertColor(hoverColor);
    }
  };

  const resetColor = () => {
    iconColor = convertColor(color);
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
        return `<line x1="256" y1="112" x2="256" y2="400" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="400" y1="256" x2="112" y2="256" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "close":
        return `<line x1="368" y1="368" x2="144" y2="144" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="368" y1="144" x2="144" y2="368" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "down":
        return `<polyline points="112 184 256 328 400 184" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:48px"/>`;
      case "up":
        return `<polyline points="112 328 256 184 400 328" style="fill:none;stroke:${iconColor};stroke-linecap:square;stroke-miterlimit:10;stroke-width:48px"/>`;
      case "folder":
        return `<path d="M64,192V120a40,40,0,0,1,40-40h75.89a40,40,0,0,1,22.19,6.72l27.84,18.56A40,40,0,0,0,252.11,112H408a40,40,0,0,1,40,40v40" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M479.9,226.55,463.68,392a40,40,0,0,1-39.93,40H88.25a40,40,0,0,1-39.93-40L32.1,226.55A32,32,0,0,1,64,192h384.1A32,32,0,0,1,479.9,226.55Z" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "code":
        return `<polyline points="160 368 32 256 160 144" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><polyline points="352 368 480 256 352 144" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="304" y1="96" x2="208" y2="416" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "pdf":
        return `<path d="M421.006 294.74c-6.824-6.723-21.957-10.283-44.986-10.586-15.589-0.172-34.351 1.201-54.085 3.964-8.837-5.099-17.946-10.647-25.094-17.329-19.231-17.958-35.284-42.886-45.288-70.297 0.652-2.56 1.207-4.81 1.724-7.106 0 0 10.833-61.53 7.966-82.333-0.396-2.853-0.638-3.681-1.404-5.898l-0.941-2.417c-2.947-6.796-8.724-13.997-17.782-13.604l-5.458-0.172c-10.101 0-18.332 5.166-20.493 12.887-6.569 24.217 0.209 60.446 12.49 107.369l-3.144 7.643c-8.794 21.438-19.815 43.030-29.539 62.079l-1.264 2.477c-10.23 20.020-19.513 37.014-27.928 51.411l-8.688 4.594c-0.632 0.334-15.522 8.209-19.014 10.322-29.628 17.69-49.262 37.771-52.519 53.708-1.036 5.085-0.265 11.593 5.007 14.606l8.403 4.229c3.646 1.826 7.489 2.751 11.427 2.751 21.103 0 45.601-26.286 79.349-85.183 38.965-12.685 83.326-23.229 122.206-29.045 29.629 16.684 66.071 28.272 89.071 28.272 4.084 0 7.606-0.39 10.466-1.147 4.411-1.168 8.129-3.684 10.396-7.097 4.463-6.716 5.367-15.966 4.156-25.438-0.36-2.811-2.605-6.287-5.034-8.66zM105.823 407.024c3.849-10.521 19.080-31.322 41.603-49.778 1.416-1.148 4.904-4.416 8.097-7.451-23.552 37.562-39.324 52.533-49.7 57.229zM239.217 99.843c6.783 0 10.642 17.097 10.962 33.127s-3.429 27.28-8.079 35.604c-3.851-12.324-5.713-31.75-5.713-44.452 0 0-0.283-24.279 2.83-24.279v0zM199.426 318.747c4.725-8.458 9.641-17.378 14.665-26.839 12.246-23.158 19.979-41.278 25.739-56.173 11.455 20.842 25.722 38.56 42.492 52.756 2.093 1.771 4.31 3.551 6.638 5.325-34.105 6.748-63.582 14.955-89.534 24.931v0zM414.451 316.826c-2.076 1.299-8.026 2.050-11.854 2.050-12.354 0-27.636-5.647-49.063-14.833 8.234-0.609 15.781-0.919 22.551-0.919 12.391 0 16.060-0.054 28.175 3.036 12.114 3.090 12.269 9.367 10.191 10.666v0z"></path><path d="M458.903 114.538c-11.105-15.146-26.587-32.85-43.589-49.852s-34.706-32.482-49.852-43.589c-25.787-18.91-38.296-21.097-45.462-21.097h-248c-22.056 0-40 17.944-40 40v432c0 22.056 17.943 40 40 40h368c22.056 0 40-17.944 40-40v-312c0-7.166-2.186-19.675-21.097-45.462v0zM392.687 87.313c15.35 15.35 27.4 29.199 36.29 40.687h-76.977v-76.973c11.491 8.89 25.339 20.939 40.687 36.286v0zM448 472c0 4.336-3.664 8-8 8h-368c-4.336 0-8-3.664-8-8v-432c0-4.336 3.664-8 8-8 0 0 247.978-0.001 248 0v112c0 8.836 7.162 16 16 16h112v312z"></path>`;
      case "image":
        return `<rect x="48" y="80" width="416" height="352" rx="48" ry="48" style="fill:none;stroke:${iconColor};stroke-linejoin:round;stroke-width:32px"/><circle cx="336" cy="176" r="32" style="fill:none;stroke:${iconColor};stroke-miterlimit:10;stroke-width:32px"/><path d="M304,335.79,213.34,245.3A32,32,0,0,0,169.47,244L48,352" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M224,432,347.34,308.66a32,32,0,0,1,43.11-2L464,368" style="fill:none;${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "text":
        return `<path d="M416,221.25V416a48,48,0,0,1-48,48H144a48,48,0,0,1-48-48V96a48,48,0,0,1,48-48h98.75a32,32,0,0,1,22.62,9.37L406.63,198.63A32,32,0,0,1,416,221.25Z" style="fill:none;stroke:${iconColor};stroke-linejoin:round;stroke-width:32px"/><path d="M256,56V176a32,32,0,0,0,32,32H408" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="176" y1="288" x2="336" y2="288" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="176" y1="368" x2="336" y2="368" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "music":
        return `<path d="M192,218v-6c0-14.84,10-27,24.24-30.59l174.59-46.68A20,20,0,0,1,416,154V176" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M416,295.94v80c0,13.91-8.93,25.59-22,30l-22,8c-25.9,8.72-52-10.42-52-38h0a33.37,33.37,0,0,1,23-32l51-18.15c13.07-4.4,22-15.94,22-29.85V58a10,10,0,0,0-12.6-9.61L204,102a16.48,16.48,0,0,0-12,16v226c0,13.91-8.93,25.6-22,30l-52,18c-13.88,4.68-22,17.22-22,32h0c0,27.58,26.52,46.55,52,38l22-8c13.07-4.4,22-16.08,22-30v-80" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "video":
        return `<path d="M374.79,308.78,457.5,367A16,16,0,0,0,480,352.38V159.62A16,16,0,0,0,457.5,145l-82.71,58.22A16,16,0,0,0,368,216.3v79.4A16,16,0,0,0,374.79,308.78Z" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M268,384H84a52.15,52.15,0,0,1-52-52V180a52.15,52.15,0,0,1,52-52H268.48A51.68,51.68,0,0,1,320,179.52V332A52.15,52.15,0,0,1,268,384Z" style="fill:none;stroke:${iconColor};stroke-miterlimit:10;stroke-width:32px"/>`;
      case "unknown":
        return `<path d="M160,164s1.44-33,33.54-59.46C212.6,88.83,235.49,84.28,256,84c18.73-.23,35.47,2.94,45.48,7.82C318.59,100.2,352,120.6,352,164c0,45.67-29.18,66.37-62.35,89.18S248,298.36,248,324" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:40px"/><circle cx="248" cy="399.99" r="32" style="fill:${iconColor};"/>`;
      case "back":
        return `<polyline points="328 112 184 256 328 400" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:48px"/>`;
      case "forward":
        return `<polyline points="184 112 328 256 184 400" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:48px"/>`;
      case "expand":
        return `<polyline points="432 320 432 432 320 432" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="421.8" y1="421.77" x2="304" y2="304" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><polyline points="80 192 80 80 192 80" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="90.2" y1="90.23" x2="208" y2="208" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><polyline points="320 80 432 80 432 192" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="421.77" y1="90.2" x2="304" y2="208" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><polyline points="192 432 80 432 80 320" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="90.23" y1="421.8" x2="208" y2="304" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "shuffle":
        return `<polyline points="400 304 448 352 400 400" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><polyline points="400 112 448 160 400 208" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M64,352h85.19a80,80,0,0,0,66.56-35.62L256,256" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M64,160h85.19a80,80,0,0,1,66.56,35.62l80.5,120.76A80,80,0,0,0,362.81,352H416" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M416,160H362.81a80,80,0,0,0-66.56,35.62L288,208" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "loop":
        return `<polyline points="320 120 368 168 320 216" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M352,168H144a80.24,80.24,0,0,0-80,80v16" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><polyline points="192 392 144 344 192 296" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><path d="M160,344H368a80.24,80.24,0,0,0,80-80V248" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "repeat":
        return `<path d="M256,256s-48-96-126-96c-54.12,0-98,43-98,96s43.88,96,98,96c37.51,0,71-22.41,94-48" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"/><path d="M256,256s48,96,126,96c54.12,0,98-43,98-96s-43.88-96-98-96c-37.51,0-71,22.41-94,48" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"/>`;
      case "closecircle":
        return `<path d="M448,256c0-106-86-192-192-192S64,150,64,256s86,192,192,192S448,362,448,256Z" style="fill:none;stroke:${iconColor};stroke-miterlimit:10;stroke-width:32px"/><line x1="320" y1="320" x2="192" y2="192" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="192" y1="320" x2="320" y2="192" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "link":
        return `<path d="M208,352H144a96,96,0,0,1,0-192h64" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:36px"/><path d="M304,160h64a96,96,0,0,1,0,192H304" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:36px"/><line x1="163.29" y1="256" x2="350.71" y2="256" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:36px"/>`;
      case "play":
        return `<polygon points="96 448 416 256 96 64 96 448"/>`;
      case "play_forward":
        return `<path d="M32,145.52v221c0,13.28,13,21.72,23.63,15.35l188.87-113c9.24-5.53,9.24-20.07,0-25.6l-188.87-113C45,123.8,32,132.24,32,145.52Z" style="fill:none;stroke:${iconColor};stroke-miterlimit:10;stroke-width:32px"/><path d="M260.57,145.52v221c0,13.28,13,21.72,23.63,15.35l188.87-113c9.24-5.53,9.24-20.07,0-25.6l-188.87-113C273.56,123.8,260.57,132.24,260.57,145.52Z" style="fill:none;stroke:${iconColor};stroke-miterlimit:10;stroke-width:32px"/>`;
      case "play_back":
        return `<path d="M480,145.52v221c0,13.28-13,21.72-23.63,15.35L267.5,268.8c-9.24-5.53-9.24-20.07,0-25.6l188.87-113C467,123.8,480,132.24,480,145.52Z" style="fill:none;stroke:${iconColor};stroke-miterlimit:10;stroke-width:32px"/><path d="M251.43,145.52v221c0,13.28-13,21.72-23.63,15.35L38.93,268.8c-9.24-5.53-9.24-20.07,0-25.6l188.87-113C238.44,123.8,251.43,132.24,251.43,145.52Z" style="fill:none;stroke:${iconColor};stroke-miterlimit:10;stroke-width:32px"/>`;
      case "play_speed":
        return `<path d="M326.1,231.9l-47.5,75.5a31,31,0,0,1-7,7,30.11,30.11,0,0,1-35-49l75.5-47.5a10.23,10.23,0,0,1,11.7,0A10.06,10.06,0,0,1,326.1,231.9Z"/><path d="M256,64C132.3,64,32,164.2,32,287.9A223.18,223.18,0,0,0,88.3,436.4c1.1,1.2,2.1,2.4,3.2,3.5a25.19,25.19,0,0,0,37.1-.1,173.13,173.13,0,0,1,254.8,0,25.19,25.19,0,0,0,37.1.1l3.2-3.5A223.18,223.18,0,0,0,480,287.9C480,164.2,379.7,64,256,64Z" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="256" y1="128" x2="256" y2="160" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"/><line x1="416" y1="288" x2="384" y2="288" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"/><line x1="128" y1="288" x2="96" y2="288" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"/><line x1="165.49" y1="197.49" x2="142.86" y2="174.86" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"/><line x1="346.51" y1="197.49" x2="369.14" y2="174.86" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"/>`;
      case "close_caption":
        return `<path d="M0,80V432H512V80ZM464,255.78c0,25.74-1.6,45.32-3.77,77.22s-19.2,54.34-59.09,57.86S305.37,394.71,256,394.6c-49,.11-105.14-.11-145.14-3.74s-56.8-26-59.09-57.86S48,281.52,48,255.78s.11-42.46,3.77-77.22,23-54.12,59.09-57.64S209.14,117.4,256,117.4s109,0,145.14,3.52,55.43,23,59.09,57.64S464,230.15,464,255.78Z"/><path d="M367.57,282.84v.77c0,17.93-11.11,28.49-25.95,28.49s-24.84-11.88-26.27-28.49c0,0-1.31-8.69-1.31-26.29a229.5,229.5,0,0,1,1.53-28.6c2.64-18.7,11.77-28.49,26.6-28.49s26.49,12.76,26.49,32.12v.55h49.58c0-24.09-6.05-45.76-18.25-59.4S369.76,153,345.8,153a108.06,108.06,0,0,0-33,4.73,58.82,58.82,0,0,0-25.94,16.61C279.63,182.3,274,192.86,270,206.17s-6,30-6,50.27c0,19.8,1.65,36.3,4.84,49.61s8,23.87,14.4,31.79a49.76,49.76,0,0,0,24,16.5q14.5,4.62,34,4.62c27.47,0,47.26-7,59.13-20.57S418,305.06,418,279.1H367.35C367.57,279.1,367.57,281.85,367.57,282.84Z"/><path d="M197.3,282.84v.77c0,17.93-11.1,28.49-25.94,28.49s-24.84-11.88-26.27-28.49c0,0-1.31-8.69-1.31-26.29a229.5,229.5,0,0,1,1.53-28.6c2.64-18.7,11.77-28.49,26.6-28.49S198.4,213,198.4,232.35v.55H248c0-24.09-6-45.76-18.25-59.4S199.5,153,175.54,153a108.06,108.06,0,0,0-33,4.73,58.82,58.82,0,0,0-25.94,16.61c-7.26,7.92-12.86,18.48-16.93,31.79s-6,30-6,50.27c0,19.8,1.65,36.3,4.84,49.61s8,23.87,14.4,31.79a49.76,49.76,0,0,0,24,16.5q14.51,4.62,34,4.62c27.48,0,47.27-7,59.14-20.57s17.81-33.33,17.81-59.29H197.08C197.3,279.1,197.3,281.85,197.3,282.84Z"/>`;
      case "play_next":
        return `<path d="M112,111V401c0,17.44,17,28.52,31,20.16l247.9-148.37c12.12-7.25,12.12-26.33,0-33.58L143,90.84C129,82.48,112,93.56,112,111Z" style="fill:none;stroke:${iconColor};stroke-miterlimit:10;stroke-width:32px"/><line x1="400" y1="80" x2="400" y2="432" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"/>`;
      case "list":
        return `<line x1="160" y1="144" x2="448" y2="144" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="160" y1="256" x2="448" y2="256" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><line x1="160" y1="368" x2="448" y2="368" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><circle cx="80" cy="144" r="16" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><circle cx="80" cy="256" r="16" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/><circle cx="80" cy="368" r="16" style="fill:none;stroke:${iconColor};stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"/>`;
      case "search":
        return `<path d="M456.69,421.39,362.6,327.3a173.81,173.81,0,0,0,34.84-104.58C397.44,126.38,319.06,48,222.72,48S48,126.38,48,222.72s78.38,174.72,174.72,174.72A173.81,173.81,0,0,0,327.3,362.6l94.09,94.09a25,25,0,0,0,35.3-35.3ZM97.92,222.72a124.8,124.8,0,1,1,124.8,124.8A124.95,124.95,0,0,1,97.92,222.72Z"/>`;
      default:
        return null;
    }
  };
</script>

<span
  class={className}
  on:click={onClick}
  on:mouseover={changeToHoverColor}
  on:mouseout={resetColor}
>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    class={svgClass}
    viewBox="0 0 512 512"
  >
    {@html iconHtml}
  </svg>
</span>

<style>
  .fill-green {
    fill: #34d399;
  }
  .fill-red {
    fill: #ef4444;
  }
  .fill-gray {
    fill: #9ca3af;
  }
  .fill-black {
    fill: black;
  }
  .fill-white {
    fill: white;
  }
  .fill-blue {
    fill: #60a5fa;
  }
</style>
