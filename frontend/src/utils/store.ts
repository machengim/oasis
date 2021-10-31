import { get, Writable, writable } from 'svelte/store';
import type { IFile, INotification, ELoopMethod, IUser, ISiteFull, IUploadTask } from './types';
import * as constants from '../assets/constants.json';

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

export const sectionStore: Writable<string> = writable(null);

export const dirsStore: Writable<Array<string>> = writable([]);

export const filesStore: Writable<Array<IFile>> = writable([]);

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

export const uploadTaskStore: Writable<IUploadTask> = writable(null);