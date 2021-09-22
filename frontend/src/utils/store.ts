import { get, Writable, writable } from 'svelte/store';
import type { IFile, INotification, ELoopMethod, ISiteFull } from './types';

export const siteStore: Writable<ISiteFull> = writable(null);

export function getSitename() {
    let site = get(siteStore);
    return (site && site.name) || "Oasis";
}

export function getLang() {
    let site = get(siteStore);
    return (site && site.language) || "en";
}

export const notificationStore: Writable<INotification> = writable(null);

export function setNotification(type: 'success' | 'error', msg: string) {
    const newNotification: INotification = { type, msg };
    notificationStore.set(newNotification);
}

export const sectionStore: Writable<string> = writable(null);

export const dirsStore: Writable<Array<string>> = writable([]);

export const filesStore: Writable<Array<IFile>> = writable([]);

export const loopStore: Writable<ELoopMethod> = writable(null);