import type { IFile, IUploadRequest, IUploadTask } from "./types";
import { EUploadStatus } from './enums';
import * as api from './api';
import { updateTask, pushWorker, removeWorker, terminateWorkers, pushFile } from "./store";
import { inferFileType } from "./util";

export async function upload(task: IUploadTask) {
  const worker = new Worker("/js/md5_worker.js");
  pushWorker(worker);
  worker.postMessage(task.file);

  worker.onmessage = async (e: MessageEvent<string>) => {
    const hash = e.data;
    worker.terminate();
    removeWorker(worker);

    task.hash = hash;
    const payload = buildUploadRequest(task);

    try {
      await preUpload(task, payload);
      await uploadFile(task, hash);
    } catch (e) {
      console.error(e);
    }
  }
}

function buildUploadRequest(task: IUploadTask) {
  const payload: IUploadRequest = {
    filename: task.file.name,
    size: task.file.size,
    target: encodeURIComponent(task.targetDir.join("/")),
    hash: task.hash,
  };

  return payload;
}

async function preUpload(task: IUploadTask, payload: IUploadRequest) {
  try {
    const uuid: string = await api.post("/api/pre-upload", payload, false);
    task.uuid = uuid;
    updateTask(task.file, EUploadStatus.uploading, 0);
  } catch (e) {
    updateTask(task.file, EUploadStatus.failed, task.progress);
    throw e;
  }
}

async function uploadFile(task: IUploadTask, hash: string) {
  if (!task.uuid || !task.hash) return;

  const file = task.file;
  let start = 0;
  const worker = new Worker("/js/upload_worker.js");
  pushWorker(worker);

  worker.postMessage({ task, start });
  worker.onmessage = async (e) => {
    start = e.data;
    const progress = start / file.size;

    if (progress >= 1) {
      updateTask(task.file, EUploadStatus.finishing, task.progress);
      worker.terminate();
      removeWorker(worker);

      try {
        await finishUpload(task);
      } catch (e) {
        throw e;
      }
    } else {
      updateTask(task.file, EUploadStatus.uploading, progress);
      worker.postMessage({ task, start });
    }

  }

  worker.onerror = (e) => {
    updateTask(task.file, EUploadStatus.failed, task.progress);
    worker.terminate();
    removeWorker(worker);
    throw e;
  }
}

async function finishUpload(task: IUploadTask) {
  try {
    await api.post(`/api/finish-upload/${task.uuid}`, null, false);
    updateTask(task.file, EUploadStatus.success, task.progress);

    const newFile: IFile = {
      dir: task.targetDir,
      file_type: inferFileType(task.file.name),
      size: task.file.size,
      filename: task.file.name
    };

    pushFile(newFile);
  } catch (e) {
    updateTask(task.file, EUploadStatus.failed, task.progress);
    throw e;
  }
}

export async function cancelUploads(tasks: Array<IUploadTask>) {
  const tasksToRemove = tasks.filter(t => !!t.uuid);
  if (tasksToRemove.filter(t => t.status === EUploadStatus.preparing ||
    t.status === EUploadStatus.uploading ||
    t.status === EUploadStatus.finishing).length > 0) {
    terminateWorkers();
  }

  const uuids = tasksToRemove.map((t) => t.uuid);

  try {
    const payload = { uuids };
    await api.post(`/api/cancel-upload`, payload, false);
  } catch (e) {
    throw e;
  }
}