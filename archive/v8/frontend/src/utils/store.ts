import { Writable, writable } from 'svelte/store';
import type { IProgress, INotification, IUploadTask, IFile, IFileAction } from './types';

// Current dir store
export const pwdStore: Writable<IFile> = writable(null);

// Current dirs store
export const dirsStore: Writable<Array<string>> = writable([]);

// Current files in dir store
export const filesStore: Writable<Array<IFile>> = writable([]);

// Notification store
export const notificationStore: Writable<INotification> = writable(null);

export function setNotification(type: 'success' | 'error', msg: string) {
    const newNotification: INotification = { type, msg };
    notificationStore.set(newNotification);
}

// Upload progress store
export const progressStore: Writable<IProgress> = writable(null);

export function setProgress(id: number, progress: number) {
    const newProgress: IProgress = { id, progress };
    progressStore.set(newProgress);
}

// Upload task store
export const uploadTaskStore: Writable<IUploadTask[]> = writable([]);

let nextId = 1;

export function addUploadTasks(files: FileList, parent_id: number) {
    const tasks: IUploadTask[] = [];

    for (const file of files) {
        const task: IUploadTask = {
            id: nextId,
            file,
            parent_id,
            progress: 0,
            status: "pending"
        };

        tasks.push(task);
        nextId++;
    }

    uploadTaskStore.set(tasks);
}

// finished upload file store
export const completeFileStore: Writable<IFile> = writable(null);

// finished update file store
export const fileActionStore: Writable<IFileAction> = writable(null);

// upload worker store
export const workerStore: Writable<Worker> = writable(null);

// Global mouse click event
export const clickEventStore: Writable<number> = writable(0);

// Dir info for breadcrumb
export const dirStructureStore: Writable<Array<IFile>> = writable([]);