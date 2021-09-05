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

export enum FileType {
  Code = "Code",
  Image = "Image",
  Pdf = "Pdf",
  Music = "Music",
  Text = "Text",
  Video = "Video",
  Unknown = "Unknown",
}