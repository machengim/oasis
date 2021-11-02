self.onmessage = async (e) => {
  const { file, hash, start, end, index } = e.data;
  const buffer = await file.slice(start, end).arrayBuffer();
  const xhr = new XMLHttpRequest();
  const endpoint = `/api/upload/${hash}/${index}`;
  xhr.open('POST', endpoint);

  xhr.onload = (_) => {
    self.postMessage({ type: "done", data: end });
  }

  xhr.onerror = (e) => {
    throw e;
  }

  xhr.send(buffer);
}