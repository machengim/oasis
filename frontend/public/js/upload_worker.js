self.onmessage = async (e) => {
  const { task, start } = e.data;
  const file = task.file;
  const sliceLength = 2 * 1024 * 1024;
  const end = Math.min(start + sliceLength, file.size);
  const index = Math.round(start / sliceLength) + 1;
  const buffer = await file.slice(start, end).arrayBuffer();
  const endpoint = `/api/upload/${task.uuid}/${index}`;

  const xhr = new XMLHttpRequest();

  xhr.open('POST', endpoint);

  xhr.onload = (_) => {
    self.postMessage(end);
  }

  xhr.onerror = (e) => {
    throw e;
  }

  xhr.send(buffer);
}