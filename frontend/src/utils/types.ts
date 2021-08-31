export interface INotification {
  type: 'success' | 'error';
  msg: string;
}

export interface ISetupRequest {
  username: string;
  password: string;
  storage: string;
}

export interface ILoginRequest {
  username: string;
  password: string;
}

export interface IFile {
  filename: string,
  file_type: string,
  size: number,
}