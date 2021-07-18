export async function get<T>(url: string): Promise<T> {
  const response = await fetch(url);
  if (!response.ok) {
    throw custom_error(response.status);
  }
  const data = await response.json();
  return data;
}

export async function post<T, S>(url: string, payload: T): Promise<S> {
  const response = await fetch(url, {body: JSON.stringify(payload), method: 'POST'});
  if (!response.ok) {
    throw custom_error(response.status);
  }
  const data = await response.json();
  return data;
}

function custom_error(code: number): Error {
  let msg = '';

  switch (code) {
    case 500:
      msg = 'Internal server error';
      break;
    default:
      break;
  }

  return new Error(msg);
}
