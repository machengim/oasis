importScripts("/vendor/md5/md5.js");

self.onmessage = async (e) => {
    const file = e.data;
    const buffer = await file.arrayBuffer();
    let dataArray = new Uint8Array(buffer);
    const hash = md5(dataArray);
    console.log("in vender, hash is ", hash);
    self.postMessage(hash);
}