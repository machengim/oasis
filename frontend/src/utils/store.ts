import { get, Writable, writable } from 'svelte/store';
import type { INotification, IUploadTask } from './types';

export const notificationStore: Writable<INotification> = writable(null);

export function setNotification(type: 'success' | 'error', msg: string) {
    const newNotification: INotification = { type, msg };
    notificationStore.set(newNotification);
}

export const uploadTaskStore: Writable<IUploadTask[]> = writable([]);

export function addUploadTasks(files: FileList) {
    const tasks: IUploadTask[] = [];

    for (const file of files) {
        const task: IUploadTask = {
            filename: file.name,
            size: file.size,
            progress: 0,
            complete: false,
        };

        tasks.push(task);
    }

    uploadTaskStore.set(tasks);
}