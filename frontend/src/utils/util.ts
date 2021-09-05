import  {FileType} from './types';

export function upperFirstChar(input: string) {
  return input.charAt(0).toUpperCase() + input.slice(1);
}

export function formatTimestamp(timestamp: number) {
  return new Date(timestamp).toISOString().slice(0, 16).replace("T", " ");
}

export function formatSize(size: number) {
  if (size <= 0) {
    return "-";
  }

  if (size < 1024) {
    return size + " B";
  }

  const units = ["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

  let u = -1;
  const dp = 1;
  const r = 10 ** dp;

  do {
    size /= 1024;
    ++u;
  } while (
    Math.round(Math.abs(size) * r) / r >= 1024 &&
    u < units.length - 1
  );

  return size.toFixed(dp) + " " + units[u];
};

export function validateForm(form: HTMLFormElement) {
  if (!form.checkValidity()) {
    form.reportValidity();
    return false;
  }

  return true;
};

export function captilizeFirst(input: string) {
  return input.charAt(0).toUpperCase() + input.slice(1);
}

export function compareArray<T>(arrayA: Array<T>, arrayB: Array<T>) {
  return arrayA.length === arrayB.length &&
    arrayA.every(function(value, index) { return value === arrayB[index]});
}

export function inferFileType(ext: string) {
  switch (ext.toLowerCase()) {
    case "c":
    case "cpp":
    case "js":
    case "ts":
    case "rs":
    case "py":
    case "java":
    case "html":
    case "css":
      return FileType.Code;
    case "png":
    case "gif":
    case "jpg":
    case "jpeg":
      return FileType.Image;
    case "mp3":
    case "ogg" :
    case "flac":
    case "aac":
      return FileType.Music;
    case "mp4":
    case "webm":
    case "mkv":
    case "avi":
    case "mov":
      return FileType.Video;
    case "pdf":
      return FileType.Pdf;
    case "txt":
      return FileType.Text;
    default:
      return FileType.Unknown;
  }
}