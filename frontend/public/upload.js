importScripts("md5.min.js");
let index = 1;
let uploadId = null;

self.onmessage = (e) => {
  const message = e.data;
  if (message.type === "uploadId") {
    uploadId = message.data;
    console.log("current upload id: ", uploadId);
  } else if (message.type === "data") {
    const hash = md5(message.data);
    const payload = {
      id: index,
      data: message.data,
      hash
    };
    console.log("payload: ", payload);
    index++;
    self.postMessage("done");
  }
  else {
    console.log("Unkown instructions");
  }
}
