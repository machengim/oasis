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

export interface IUploadTask {
  id: number;
  file: File;
  progress: number;
  status: "pending" | "uploading" | "finished";
}