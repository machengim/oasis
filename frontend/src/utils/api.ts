import type { IPartialBlob } from '../utils/types';

export async function get<T>(url: string, jsonResponse: boolean = true): Promise<T> {
  let response: Response;
  try {
    response = await fetch(url);
  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return jsonResponse ? await response.json() : await response.text();
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