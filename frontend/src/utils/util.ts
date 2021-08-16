export function upperFirstChar(input: string) {
  return input.charAt(0).toUpperCase() + input.slice(1);
}

export function formatSize(size: number) {
  if (size < 0) {
    return null;
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