## OASIS

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/machengim/oasis/blob/main/LICENSE-MIT) [![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/machengim/oasis)](https://github.com/machengim/oasis/releases) [![Docker](https://img.shields.io/badge/docker-v0.2-orange)](https://hub.docker.com/r/machengim/oasis)

[English README](https://github.com/machengim/oasis/blob/main/README.md)

自建文件服务器。

![](https://github.com/machengim/oasis/blob/main/doc/Oasis_demo.jpg?raw=true)

### 安装

1. 从 [release](https://github.com/machengim/oasis/releases) 页面下载
2. 解压缩
3. 在 `oasis.conf` 中设置服务器的IP和端口
4. 运行 `oasis` 或 `oasis.exe`
5. 从浏览器访问服务器的 IP 地址

### 功能

- 用户验证
- 文件预览
- 文件下载
- 文件上传
- 播放列表
- 移动端适配
- 临时分享链接
- 外部媒体播放器支持(通过分享链接)
- 多平台支持 (Linux, MacOS, Windows, Docker)
- I18n (英语, 中文)

### 文件格式支持

- 文本
- 图片 (浏览器支持)
- 音频 (浏览器支持)
- 视频 (浏览器支持)
- 字幕 (srt / vtt 格式, 支持 Chrome, Firefox 和 Edge 浏览器)
- PDF (由 pdf.js 支持)

### 规划

- [ ] 多用户管理
- [ ] HTTPS
- [ ] 更多文件格式支持

### 技术栈

- [Svelte](https://svelte.dev)
- [Rocket](https://rocket.rs)
- [Tailwind](https://tailwindcss.com)

### 致谢

- [Pdf.js](https://mozilla.github.io/pdf.js)
- [Plyr](https://plyr.io)

### 构建

- Node 14.17+
- Rust 1.54+

```
cd path/to/oasis
node build.js
```

### Docker

https://hub.docker.com/r/machengim/oasis

```
docker pull machengim/oasis

docker run --name oasis -t -d \
-v <db>:/opt/oasis/db \
-v <storage>:/home/storage \
-p <port>:8000 machengim/oasis
```
