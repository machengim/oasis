importScripts("md5.min.js");
let index = 0;
let uploadId = null;

self.onmessage = async (e) => {
  const message = e.data;
  if (message.type === "uploadId") {
    uploadId = message.data;
    console.log("current upload id: ", uploadId);
  } else if (message.type === "data") {
    console.log("message data is ", message.data);
    const hash = md5(message.data);
    const dataArray = new Uint8Array(message.data);
    const payload = {
      index,
      data: [...dataArray],
      hash
    };

    // console.log("payload: ", payload);

    let xhr = new XMLHttpRequest();
    let endpoint = `/api/upload/${uploadId}`;
    xhr.open('POST', endpoint);
    xhr.send(JSON.stringify(payload));
    xhr.addEventListener("load", (e) => {
      console.log("task complete, ", e);
      index++;
      self.postMessage("done");
    });
  }
  else {
    console.log("Unkown instructions");
  }
}


