export interface INotification {
  type: 'success' | 'error';
  msg: string;
}

export interface ISetupRequest {
  username: string;
  password: string;
  storage: string;
  language: string;
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

export interface ILoopIcon {
  type: ELoopMethod;
  selected: boolean;
}

export interface ISiteBasic {
  version: string,
  language: string
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
  closecircle = "closecircle"
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