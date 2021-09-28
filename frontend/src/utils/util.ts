import { getLocaleFromNavigator } from 'svelte-i18n';
import { FileType, IFile, IFileOrder } from './types';

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
    arrayA.every(function (value, index) { return value === arrayB[index] });
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
    case "sh":
      return FileType.Code;
    case "png":
    case "gif":
    case "jpg":
    case "jpeg":
      return FileType.Image;
    case "mp3":
    case "ogg":
    case "flac":
    case "aac":
    case "wav":
      return FileType.Music;
    case "mp4":
    case "webm":
    case "mkv":
    case "avi":
    case "mov":
    case "flv":
      return FileType.Video;
    case "pdf":
      return FileType.Pdf;
    case "txt":
    case "srt":
    case "vtt":
    case "md":
    case "json":
    case "yml":
    case "ini":
    case "conf":
      return FileType.Text;
    default:
      return FileType.Unknown;
  }
}

export function compareFile(a: IFile, b: IFile, order: IFileOrder) {
  let ascFactor = order.asc ? 1 : -1;
  let result = 0;

  switch (order.key) {
    case "name":
      const aUpper = a.filename.toUpperCase();
      const bUpper = b.filename.toUpperCase();
      result = aUpper > bUpper ? 1 : aUpper < bUpper ? -1 : 0;
      break;
    case "size":
      result = a.size - b.size;
      break;
    case "type":
      result =
        a.file_type > b.file_type ? 1 : a.file_type < b.file_type ? -1 : 0;
      break;
    default:
      break;
  }

  return result * ascFactor;
};

export function checkMobile() {
  return (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent));
}

export function readCookie(name: string) {
  const nameEQ = name + "=";
  const ca = document.cookie.split(';');
  for (let i = 0; i < ca.length; i++) {
    let c = ca[i];
    while (c.charAt(0) == ' ') c = c.substring(1, c.length);
    if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
  }

  return null;
}

export function getLocale() {
  let browserLocale = getLocaleFromNavigator();

  return browserLocale.startsWith("cn") ? "cn" : "en";
}

export function compareVersion(v1: string, v2: string) {
  const parts1 = v1.split(".").map(p => +p);
  const parts2 = v2.split(".").map(p => +p);

  for (let i = 0; i < Math.min(parts1.length, parts2.length); i++) {
    if (parts1[i] > parts2[i]) {
      return 1;
    } else if (parts1[i] < parts2[i]) {
      return -1;
    }
  }

  if (parts1.length > parts2.length) {
    return 1;
  } else if (parts1.length < parts2.length) {
    return -1;
  }

  return 0;
}

export function srtToVtt(input: string): string {
  let output = "WEBVTT\n\n";
  const lines = input.split("\n");

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (line.match(/^\d+\s*$/)) {
      continue;
    } else if (line.match(/^.*-->.*\s*$/)) {
      const time_line = line.replace(/,/g, ".");
      output += time_line + "\n";
    } else {
      output += line + "\n";
    }
  }

  return output;
}