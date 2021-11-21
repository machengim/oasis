export enum EFileType {
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
