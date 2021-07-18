export async function get<T>(url: string): Promise<T> {
  let response: Response;
  try {
     response = await fetch(url);
  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return await response.json();
}

export async function post<T, S>(url: string, payload: T, needResponse: boolean): Promise<S> {
  let response: Response;
  try {
    response = await fetch(url, {body: JSON.stringify(payload), method: 'POST'});

  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new Error(response.status.toString());
  }

  return needResponse ? await response.json() : null;
}

