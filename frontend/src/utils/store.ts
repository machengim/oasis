import { get, Writable, writable } from 'svelte/store';
import type { IFile, INotification, ELoopMethod, IUser, ISiteFull, IUploadTask, EUploadStatus } from './types';
import * as constants from '../assets/constants.json';
import { compareArray } from './util';

export const siteStore: Writable<ISiteFull> = writable(null);

export function getSitename() {
  let site = get(siteStore);
  return (site && site.name) || constants.app_name;
}

export function getLang() {
  let site = get(siteStore);
  return (site && site.language) || "en";
}

export const userStore: Writable<IUser> = writable(null);

export function getUsername() {
  let user = get(userStore);
  return (user && user.username) || "";
}

export const notificationStore: Writable<INotification> = writable(null);

export function setNotification(type: 'success' | 'error', msg: string) {
  const newNotification: INotification = { type, msg };
  notificationStore.set(newNotification);
}

export const clickStore: Writable<number> = writable(0);

export const sectionStore: Writable<string> = writable(null);

export const dirsStore: Writable<Array<string>> = writable([]);

export const filesStore: Writable<Array<IFile>> = writable([]);

export function pushFile(file: IFile) {
  if (!file.dir) return;

  const currentDir = get(dirsStore);
  if (compareArray(currentDir, file.dir)) {
    const newFiles = get(filesStore);
    newFiles.push(file);
    filesStore.set(newFiles);
  }
}

export function updateFile(oldFile: IFile, newFile: IFile) {
  if (!newFile.dir) return;

  const currentDir = get(dirsStore);
  if (compareArray(currentDir, newFile.dir)) {
    const files = get(filesStore);
    const index = files.findIndex(f => f.filename === oldFile.filename);
    if (index >= 0) {
      files[index] = newFile;
      filesStore.set(files);
    }
  }
}

export function deleteFile(file: IFile) {
  if (!file.dir) return;

  const currentDir = get(dirsStore);
  if (compareArray(currentDir, file.dir)) {
    const files = get(filesStore);
    const index = files.findIndex(f => f === file);
    if (index >= 0) {
      files.splice(index, 1);
      filesStore.set(files);
    }
  }
}

export const loopStore: Writable<ELoopMethod> = writable(null);

export const titleStore: Writable<string> = writable(null);

export function resetTitle() {
  const site = get(siteStore);
  if (site && site.name) {
    titleStore.set(site.name);
  } else {
    titleStore.set(constants.app_name);
  }
}

export const uploadTaskStore: Writable<Array<IUploadTask>> = writable([]);

export function updateTask(file: File, status: EUploadStatus, progress: number) {
  const tasks = get(uploadTaskStore);
  const index = tasks.findIndex(t => t.file === file);
  if (index >= 0) {
    tasks[index].status = status;
    tasks[index].progress = progress;
    uploadTaskStore.set(tasks);
  }
}

export const workerStore: Writable<Array<Worker>> = writable([]);

export function pushWorker(worker: Worker) {
  const newWorkers = get(workerStore);
  newWorkers.push(worker);
  workerStore.set(newWorkers);
}

export function removeWorker(worker: Worker) {
  const workers = get(workerStore);
  const newWorkers = workers.filter(w => w !== worker);
  workerStore.set(newWorkers);
}

export function terminateWorkers() {
  const workers = get(workerStore);
  for (const worker of workers) {
    worker.terminate();
  }

  workerStore.set([]);
}