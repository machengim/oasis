import { EUploadStatus, IUploadRequest, IUploadTask } from "./types";
import * as api from './api';
import { updateTask, completeTaskStore, pushWorker, removeWorker } from "./store";

export async function upload(task: IUploadTask) {
  const worker = new Worker("js/md5_worker.js");
  pushWorker(worker);
  worker.postMessage(task.file);

  worker.onmessage = async (e: MessageEvent<string>) => {
    const hash = e.data;
    worker.terminate();
    removeWorker(worker);

    task.hash = hash;
    const payload = buildUploadRequest(task, hash);

    try {
      await preUpload(task, payload);
      await uploadFile(task, hash);
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

  const worker = new Worker("js/upload_worker.js");
  pushWorker(worker);

  worker.postMessage({ file, hash, start, end, index });
  worker.onmessage = async (e) => {
    const message = e.data;
    if (message.type === "progress") {
      const progress = message.data / file.size;
      if (progress >= 1) {
        updateTask(task.file, EUploadStatus.finishing, task.progress);
        worker.terminate();
        removeWorker(worker);

        let payload = buildUploadRequest(task, hash);

        try {
          await finishUpload(task, payload);
        } catch (e) {
          throw e;
        }
      } else {
        updateTask(task.file, EUploadStatus.uploading, progress);
        start = end;
        end = Math.min(start + sliceLength, file.size);
        index++;
        worker.postMessage({ file, hash, start, end, index });
      }
    } else if (e.type === "error") {
      updateTask(task.file, EUploadStatus.failed, task.progress);
      worker.terminate();
      removeWorker(worker);
      throw e.data;
    }
  }
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