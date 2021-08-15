importScripts("md5.js");
let index = 0;
let uploadId = null;

self.onmessage = async (e) => {
  const message = e.data;
  if (message.type === "uploadId") {
    uploadId = message.data;
  } else if (message.type === "data") {
    const dataArray = new Uint8Array(message.data);
    const data = [...dataArray];
    const hash = md5(data);

    let xhr = new XMLHttpRequest();
    let endpoint = `/api/file/upload/${uploadId}?index=${index}&hash=${hash}`;

    xhr.open('POST', endpoint);

    xhr.upload.onprogress = (e) => {
      console.log(`${e.loaded} bytes transferred\n`);
    }

    xhr.onload = (e) => {
      console.log("task complete: ", e);
      index++;
      self.postMessage("done");
    }

    xhr.onerror = (e) => {
      console.log("error happened: ", e);
    }

    // xhr.send(JSON.stringify(payload));
    xhr.send(dataArray);

  } else {
    console.log("Unkown instructions");
  }
}


