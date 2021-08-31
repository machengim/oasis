import { Writable, writable } from 'svelte/store';
import type { INotification } from './types';

// Notification store
export const notificationStore: Writable<INotification> = writable(null);

export function setNotification(type: 'success' | 'error', msg: string) {
    const newNotification: INotification = { type, msg };
    notificationStore.set(newNotification);
}

export const sectionStore: Writable<string> = writable(null);