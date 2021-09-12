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
  file_type: FileType,
  size: number,
}

export interface IFileOrder {
  key: "name" | "type" | "size",
  asc: boolean,
}

export interface IPartialBlob {
  blob: Blob,
  start: number,
  end: number,
  size: number,
  type: string,
}

export enum FileType {
  Code = "Code",
  Dir = "Dir",
  Image = "Image",
  Pdf = "Pdf",
  Music = "Music",
  Text = "Text",
  Video = "Video",
  Unknown = "Unknown",
}

export enum IconType {
  success = "success",
  error = "error",
  profile = "profile",
  add = "add",
  close = "close",
  up = "up",
  down = "down",
  folder = "folder",
}