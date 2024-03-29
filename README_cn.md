## OASIS

[![Build](https://github.com/machengim/oasis/actions/workflows/build_release.yml/badge.svg)](https://github.com/machengim/oasis/actions/workflows/build_release.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/machengim/oasis/blob/main/LICENSE-MIT) [![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/machengim/oasis)](https://github.com/machengim/oasis/releases) [![Docker](https://img.shields.io/badge/docker-v0.2.5-orange)](https://hub.docker.com/r/machengim/oasis)

[English README](https://github.com/machengim/oasis/blob/main/README.md)

自建文件服务器。

![](https://github.com/machengim/oasis/blob/main/doc/demo.png?raw=true)

### 安装

1. 从 [release](https://github.com/machengim/oasis/releases) 页面下载
2. 解压缩
3. 如果运行在 Linux 或 MacOS 中, 授予 `oasis` 文件可执行权限
4. (可选) 在 `oasis.conf` 中设置服务器的 IP 和端口
5. 运行 `oasis` 或 `oasis.exe`
6. 从浏览器访问服务器的 IP 地址

### Docker

https://hub.docker.com/r/machengim/oasis

```
docker run --name oasis -t -d \
-v <data>:/opt/oasis/data \
-v <storage>:/home/storage \
-p <port>:8000 machengim/oasis
```

### 构建

- Node 14+
- Rust 1.54+

```
cd path/to/oasis
node build.js
```

### 功能

- 用户验证
- 文件预览/下载/上传/搜索
- 媒体文件播放列表
- 文件外部链接
- I18n (英语, 中文)

### 文件格式支持

- 文本
- 图片 (浏览器支持)
- 音频 (浏览器支持)
- 视频 (浏览器支持)
- 字幕 (`srt` / `vtt` 格式, 支持 Chrome, Firefox 和 Edge 浏览器)
- PDF (由 pdf.js 支持)

### 技术栈

- [Svelte](https://svelte.dev)
- [Rocket](https://rocket.rs)
- [Tailwind](https://tailwindcss.com)

### 致谢

- [Pdf.js](https://mozilla.github.io/pdf.js)
- [Plyr](https://plyr.io)
