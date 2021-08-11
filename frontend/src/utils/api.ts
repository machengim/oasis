import type { IUploadTask } from './types';

export async function get<T>(url: string): Promise<T> {
  let response: Response;
  try {
    response = await fetch(url);
  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return await response.json();
}

export async function post<T, S>(url: string, payload: T, jsonResponse: boolean): Promise<S> {
  let response: Response;
  try {
    response = await fetch(url, { body: JSON.stringify(payload), method: 'POST' });

  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return jsonResponse ? await response.json() : await response.text();
}

// TODO: process uploading.
export async function upload(task: IUploadTask) {
  const file = task.file;
  const filesize = file.size;
  const buffer = await file.arrayBuffer();
  const worker = new Worker('upload.js');
  const length = 2 * 1024 * 1024;

  const payload = {
    filename: file.name,
    size: filesize,
  };

  let uploadId: string = await post("/api/pre_upload", payload, false);
  console.log("Get response: ", uploadId);

  // TODO: send pre uploading request.
  // return;

  let start = 0;
  let sliceCount = 0;
  let end = Math.min(start + length, filesize);
  let slice = buffer.slice(start, end);
  worker.postMessage({ type: "uploadId", data: uploadId });
  worker.postMessage({ type: "data", data: slice });

  worker.onmessage = async (e) => {
    if (e.data === "done") {
      start = end;
      sliceCount++;
      if (end < filesize) {
        console.log("finished ", end);
        end = Math.min(start + length, filesize);
        slice = buffer.slice(start, end);
        worker.postMessage({ type: "data", data: slice });
      } else {
        console.log("terminate");
        worker.terminate();
        const payload = {
          filename: file.name,
          slices: sliceCount,
          upload_id: uploadId,
        };
        await post(`/api/finish-upload/${uploadId}`, payload, false);
      }
    }
  };
}
