<script lang="ts">
  import { FileType } from "../utils/types";
  import Icon from "./Icon.svelte";

  export let file_type: FileType;
  let icon_type:
    | "folder"
    | "code"
    | "image"
    | "text"
    | "music"
    | "video"
    | "unknown";
  let color:
    | "green"
    | "yellow"
    | "red"
    | "pink"
    | "gray"
    | "black"
    | "white"
    | "blue";

  switch (file_type) {
    case FileType.Dir:
      icon_type = "folder";
      color = "blue";
      break;
    case FileType.Image:
      icon_type = "image";
      color = "red";
      break;
    case FileType.Code:
      icon_type = "code";
      color = "pink";
      break;
    case FileType.Text:
    case FileType.Pdf:
      icon_type = "text";
      color = "black";
      break;
    case FileType.Video:
      icon_type = "video";
      color = "yellow";
      break;
    case FileType.Music:
      icon_type = "music";
      color = "green";
      break;
    default:
      icon_type = "unknown";
      color = "gray";
      break;
  }
</script>

<Icon type={icon_type} {color} size="small" />
