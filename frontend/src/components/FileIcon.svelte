<script lang="ts">
  import type { IFile } from "../utils/types";
  import { EFileType, EIconColor, EIconType } from "../utils/enums";
  import Icon from "./Icon.svelte";

  export let file: IFile;
  let icon_type: EIconType;
  let color: EIconColor;

  $: setFileIcon(file);

  const setFileIcon = (file: IFile) => {
    switch (file.file_type) {
      case EFileType.Dir:
        icon_type = EIconType.folder;
        color = EIconColor.blue;
        break;
      case EFileType.Image:
        icon_type = EIconType.image;
        color = EIconColor.red;
        break;
      case EFileType.Code:
        icon_type = EIconType.code;
        color = EIconColor.pink;
        break;
      case EFileType.Pdf:
        icon_type = EIconType.pdf;
        color = EIconColor.red;
        break;
      case EFileType.Text:
        icon_type = EIconType.text;
        color = EIconColor.black;
        break;
      case EFileType.Video:
        icon_type = EIconType.video;
        color = EIconColor.yellow;
        break;
      case EFileType.Music:
        icon_type = EIconType.music;
        color = EIconColor.green;
        break;
      default:
        icon_type = EIconType.unknown;
        color = EIconColor.gray;
        break;
    }
  };
</script>

<Icon type={icon_type} {color} size="small" />
