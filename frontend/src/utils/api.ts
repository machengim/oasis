import type { IPartialBlob, IUser } from '../utils/types';
import { userStore } from './store';

export async function get<T>(url: string, dataType: "json" | "text" | "blob" | "raw" = "json"): Promise<T> {
  let response: Response;
  try {
    response = await fetch(url);
  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return dataType === "json" ? await response.json() : dataType === "text" ? await response.text() : dataType === "blob" ? await response.blob() : response;
}

export async function getRange(url: string, start: number, end: number = 0): Promise<IPartialBlob> {
  let response: Response;
  let rangeHeader = "bytes=" + start + "-";
  if (end > 0) {
    rangeHeader += end;
  }

  try {
    response = await fetch(url, {
      headers: {
        "range": rangeHeader
      }
    });
  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return await parseRangeResponse(response);
}

export async function post<T, S>(url: string, payload: T, jsonResponse: boolean): Promise<S> {
  let response: Response;
  try {
    response = await fetch(url, { body: JSON.stringify(payload), method: 'POST' });

  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return jsonResponse ? await response.json() : await response.text();
}

export async function put<T, S>(url: string, payload: T, jsonResponse: boolean): Promise<S> {
  let response: Response;
  try {
    response = await fetch(url, { body: JSON.stringify(payload), method: 'PUT' });

  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return jsonResponse ? await response.json() : await response.text();
}

export async function remove<T, S>(url: string, payload: T, jsonResponse: boolean): Promise<S> {
  let response: Response;
  try {
    response = await fetch(url, { body: JSON.stringify(payload), method: 'DELETE' });

  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return jsonResponse ? await response.json() : await response.text();
}

export async function refresh_token() {
  try {
    const response = await fetch("/api/user/refresh");
    const user: IUser = await response.json();
    
    return user;
  } catch (e) {
    userStore.set(null);
  }
}

async function parseRangeResponse(response: Response): Promise<IPartialBlob> {
  const blob = await response.blob();
  const range = response.headers.get("content-range");
  const splits = range.split(/\s|-|\//);
  const type = response.headers.get("content-type");
  const partialBlob: IPartialBlob = {
    blob, start: +splits[1], end: +splits[2], size: +splits[3], type
  };

  return partialBlob;
}