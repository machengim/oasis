self.onmessage = async (e) => {
  const { file, hash, start, end, index } = e.data;

  let buffer = await file.slice(start, end).arrayBuffer();
  let data = Array.from(new Uint8Array(buffer));
  let payload = { hash, index, data };

  let xhr = new XMLHttpRequest();
  xhr.open('POST', "/api/upload");

  xhr.onload = (e) => {
    self.postMessage({ type: "progress", data: end });
  }

  xhr.onerror = (e) => {
    self.postMessage({ type: "error", data: e });
  }

  xhr.send(JSON.stringify(payload));
}