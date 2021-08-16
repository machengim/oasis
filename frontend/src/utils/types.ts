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
  status: "pending" | "uploading" | "complete";
}

export interface IProgress {
  id: number;
  progress: number;
}

export interface IFile {
  file_id: number,
  filename: string,
  path: string,
  size: number,
  is_dir: number,
  owner_id: number,
  parent_id: number,
  created_at: string,
  updated_at: string,
}

export interface IFileOrder {
  key: "name" | "type" | "size" | "lastModify",
  asc: boolean,
}