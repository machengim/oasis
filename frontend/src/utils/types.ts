export interface INotification {
  type: 'success' | 'error';
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
  new_password: string
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

export interface ILoopIcon {
  type: ELoopMethod;
  selected: boolean;
}

export interface ISiteBrief {
  name: string,
  version: string,
  language: string,
  update_freq: string,
}

export interface ISiteFull {
  name: string,
  version: string,
  language: string,
  storage: string,
  update_freq: string,
}

export interface ILink {
  name: string,
  url: string,
}

export interface IUser {
  username: string;
  permission: number;
  expire: number;
}

export interface IUploadTask {
  file: File;
  targetDir: string;
  status: EUploadStatus;
  progress: number;
}

export interface IPreUploadRequest {
  filename: string;
  size: number;
  targetDir: string;
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

export enum EIconType {
  success = "success",
  error = "error",
  profile = "profile",
  add = "add",
  close = "close",
  up = "up",
  down = "down",
  folder = "folder",
  code = "code",
  pdf = "pdf",
  image = "image",
  text = "text",
  music = "music",
  video = "video",
  back = "back",
  forward = "forward",
  expand = "expand",
  shuffle = "shuffle",
  loop = "loop",
  repeat = "repeat",
  unknown = "unknown",
  closecircle = "closecircle",
  link = "link",
  play = "play",
  play_forward = "play_forward",
  play_back = "play_back",
  play_speed = "play_speed",
  close_caption = "close_caption",
  play_next = "play_next",
  list = "list"
}

export enum EIconColor {
  green = "green",
  red = "red",
  gray = "gray",
  black = "black",
  white = "white",
  blue = "blue",
  yellow = "yellow",
  pink = "pink"
}

export enum ELoopMethod {
  repeat = "repeat",  // repeat single file
  shuffle = "shuffle",  // random shuffle play list
  loop = "loop"     // loop play list in sequence
}

export enum EUploadStatus {
  waiting = "waiting",
  preparing = "preparing",
  uploading = "uploading",
  finishing = "finishing",
  success = "success",
  failed = "failed"
}
