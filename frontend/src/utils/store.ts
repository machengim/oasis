import { Writable, writable } from 'svelte/store';
import type { INotification } from './types';

export const notificationStore: Writable<INotification> = writable(null);