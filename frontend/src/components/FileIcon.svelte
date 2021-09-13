<script lang="ts">
  import type { IFile } from "../utils/types";
  import { FileType, EIconColor, EIconType } from "../utils/types";
  import Icon from "./Icon.svelte";

  export let file: IFile;
  let file_type: FileType;
  let icon_type: EIconType;
  let color: EIconColor;

  $: file_type = file.file_type;

  $: switch (file_type) {
    case FileType.Dir:
      icon_type = EIconType.folder;
      color = EIconColor.blue;
      break;
    case FileType.Image:
      icon_type = EIconType.image;
      color = EIconColor.red;
      break;
    case FileType.Code:
      icon_type = EIconType.code;
      color = EIconColor.pink;
      break;
    case FileType.Text:
    case FileType.Pdf:
      icon_type = EIconType.text;
      color = EIconColor.black;
      break;
    case FileType.Video:
      icon_type = EIconType.video;
      color = EIconColor.yellow;
      break;
    case FileType.Music:
      icon_type = EIconType.music;
      color = EIconColor.green;
      break;
    default:
      icon_type = EIconType.unknown;
      color = EIconColor.gray;
      break;
  }
</script>

<Icon type={icon_type} {color} size="small" />
