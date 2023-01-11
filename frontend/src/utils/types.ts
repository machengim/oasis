import type * as enums from "./enums";

export interface INotification {
  type: "success" | "error";
  msg: string;
}

export interface ISetupRequest {
  sitename: string;
  username: string;
  password: string;
  storage: string;
  language: string;
}

export interface ILoginRequest {
  username: string;
  password: string;
}

export interface IUpdateConfigRequest {
  sitename: string;
  language: string;
  storage: string;
  update_freq: string;
  allow_guest: boolean;
}

export interface IUpdateAppNeedRespose {
  need: boolean;
  url: string;
}

export interface IUpdateAppInfo {
  version: string;
  descriptions: Array<IUpdateDescription>;
  changelog: string;
  url: string;
}

export interface IUpdateDescription {
  lang: string;
  detail: string;
}

export interface IChangePasswordRequest {
  username: string;
  old_password: string;
  new_password: string;
}

export interface IFile {
  dir?: string;
  filename: string;
  file_type: enums.EFileType;
  size: number;
  least_permission: number;
}

export interface IFileOrder {
  key: "name" | "type" | "size";
  asc: boolean;
}

export interface IPartialBlob {
  blob: Blob;
  start: number;
  end: number;
  size: number;
  type: string;
}

export interface ILoopIcon {
  type: enums.ELoopMethod;
  selected: boolean;
}

export interface ISiteBrief {
  name: string;
  version: string;
  language: string;
  update_freq: string;
  allow_guest: boolean;
}

export interface ISiteFull {
  name: string;
  version: string;
  language: string;
  storage: string;
  update_freq: string;
  allow_guest: boolean;
}

export interface ILink {
  name: string;
  url: string;
}

export interface IUser {
  username: string;
  permission: number;
  expire: number;
}

export interface IUploadTask {
  uuid?: string;
  file: File;
  targetDir: Array<string>;
  status: enums.EUploadStatus;
  progress: number;
  hash?: string;
}

export interface IUploadRequest {
  filename: string;
  size: number;
  target: string; // target directory to store the uploading file
  hash: string; // the md5 value of the file
}

export interface IMousePosition {
  x: number;
  y: number;
}

export interface IResetPasswordRequest {
  uuid: string;
  code: string;
  username: string;
  password: string;
}

export interface ICopyMoveTask {
  uuid: string;
  status: enums.ECopyMoveTaskStatus;
  source: string;
  target: string;
  progress: number;
  is_copy: boolean;
}

export type ContextMenuAction =
  | "rename"
  | "delete"
  | "close"
  | "visibility"
  | "copy"
  | "move"
  | "download";
