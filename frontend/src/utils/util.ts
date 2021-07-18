import { notificationStore } from "../utils/store";
import type { INotification } from "../utils/types";

export function upperFirstChar(input: string) {
  return input.charAt(0).toUpperCase() + input.slice(1);
}

export function setNotification(type: 'success' | 'error', msg: string) {
  const newNotification: INotification = { type, msg };
  notificationStore.set(newNotification);
}