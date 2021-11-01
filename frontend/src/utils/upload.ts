import { EUploadStatus, IUploadRequest, IUploadTask } from "./types";
import * as api from './api';
import { updateTask, completeTaskStore } from "./store";

export async function upload(task: IUploadTask) {
  const worker = new Worker("js/md5.js");
  worker.postMessage(task.file);

  worker.onmessage = async (e: MessageEvent<string>) => {
    const hash = e.data;
    worker.terminate();
    const payload = buildUploadRequest(task, hash);

    try {
      await preUpload(task, payload);
      await uploadFile(task, hash);
      await finishUpload(task, payload);
    } catch (e) {
      console.error(e);
    }
  }
}

function buildUploadRequest(task: IUploadTask, hash: string) {
  const payload: IUploadRequest = {
    filename: task.file.name,
    size: task.file.size,
    target: task.targetDir,
    hash,
  };

  return payload;
}

async function preUpload(task: IUploadTask, payload: IUploadRequest) {
  try {
    await api.post("/api/pre-upload", payload, false);
    updateTask(task.file, EUploadStatus.uploading, 0);
  } catch (e) {
    updateTask(task.file, EUploadStatus.failed, task.progress);
    throw e;
  }

}

async function uploadFile(task: IUploadTask, hash: string) {
  const file = task.file;
  const MB = 1024 * 1024;
  const sliceLength = file.size > 100 * MB ? 2 * MB : 1 * MB;
  let start = 0;
  let index = 0;
  let end = Math.min(start + sliceLength, file.size);

  while (start < file.size) {
    const bufer = await file.slice(start, end).arrayBuffer();
    const data = Array.from(new Uint8Array(bufer));
    const payload = {
      hash, index, data
    };

    try {
      await api.post("/api/upload", payload, false);
      const progress = end / file.size;
      updateTask(task.file, EUploadStatus.uploading, progress);

      index++;
      start = end;
      end = Math.min(start + sliceLength, file.size);
    } catch (e) {
      updateTask(task.file, EUploadStatus.failed, task.progress);
      throw e;
    }
  }

  updateTask(task.file, EUploadStatus.finishing, task.progress);
}

async function finishUpload(task: IUploadTask, payload: IUploadRequest) {
  try {
    await api.post("/api/finish-upload", payload, false);
    updateTask(task.file, EUploadStatus.success, task.progress);
    completeTaskStore.set(task);
  } catch (e) {
    updateTask(task.file, EUploadStatus.failed, task.progress);
    throw e;
  }
}