import { Writable, writable } from 'svelte/store';
import type { IFile, INotification, ELoopMethod } from './types';

export const notificationStore: Writable<INotification> = writable(null);

export function setNotification(type: 'success' | 'error', msg: string) {
    const newNotification: INotification = { type, msg };
    notificationStore.set(newNotification);
}

export const sectionStore: Writable<string> = writable(null);

export const dirsStore: Writable<Array<string>> = writable([]);

export const filesStore: Writable<Array<IFile>> = writable([]);

export const loopStore: Writable<ELoopMethod> = writable(null);