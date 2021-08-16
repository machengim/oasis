import { get, Writable, writable } from 'svelte/store';
import type { IProgress, INotification, IUploadTask } from './types';

export const notificationStore: Writable<INotification> = writable(null);

export function setNotification(type: 'success' | 'error', msg: string) {
    const newNotification: INotification = { type, msg };
    notificationStore.set(newNotification);
}

export const progressStore: Writable<IProgress> = writable(null);

export function setProgress(id: number, progress: number) {
    const newProgress: IProgress = { id, progress };
    progressStore.set(newProgress);
}

export const uploadTaskStore: Writable<IUploadTask[]> = writable([]);

let nextId = 1;

export function addUploadTasks(files: FileList) {
    const tasks: IUploadTask[] = [];

    for (const file of files) {
        const task: IUploadTask = {
            id: nextId,
            file,
            progress: 0,
            status: "pending"
        };

        tasks.push(task);
        nextId++;
    }

    uploadTaskStore.set(tasks);
}