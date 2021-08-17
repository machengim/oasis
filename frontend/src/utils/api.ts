import type { IFile, IUploadTask } from './types';
import { completeFileStore, setProgress, workerStore } from '../utils/store';

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
  const length = 20 * 1024 * 1024;

  const payload = {
    filename: file.name,
    parent_id: task.parent_id,
    size: filesize,
  };

  let uploadId: string = await post("/api/file/before-upload", payload, false);

  let transferredBytes = 0;
  let start = 0;
  let end = Math.min(start + length, filesize);
  let slice = file.slice(start, end);
  let buffer = await slice.arrayBuffer();

  const worker = new Worker('upload.js');
  workerStore.set(worker);
  worker.postMessage({ type: "uploadId", data: uploadId });
  worker.postMessage(buffer, [buffer]);

  worker.onmessage = async (e) => {
    const message = e.data;
    if (message.type === "progress") {
      transferredBytes += message.data;
      task.progress = transferredBytes / filesize;
      setProgress(task.id, task.progress);
    } else if (message.type === "done") {
      if (end < filesize) {
        start = end;
        end = Math.min(start + length, filesize);
        slice = file.slice(start, end);
        buffer = await slice.arrayBuffer();

        worker.postMessage(buffer, [buffer]);
      } else {
        worker.terminate();
        const payload = {
          upload_id: uploadId,
        };

        const completeFile: IFile = await post(`/api/file/finish-upload`, payload, true);
        completeFileStore.set(completeFile);
      }
    } else if (message.type === "error") {
      // TODO: retry several times.
    }
  };
}
