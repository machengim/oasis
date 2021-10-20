## OASIS

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/machengim/oasis/blob/master/LICENSE-MIT) ![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/machengim/oasis)

[English README](https://github.com/machengim/oasis/blob/main/README.md)

极简文件服务和共享应用。

### 安装

1. 从release页面下载
2. 解压缩
3. 运行 `oasis` 或 `oasis.exe`
4. 从浏览器访问服务器的IP地址

### 功能

+ 用户验证
+ 文件预览
+ 播放列表
+ 移动端适配
+ 临时共享链接
+ 多平台支持
+ I18n (英语, 中文)

### 文件格式支持

+ 文本
+ 图片 (浏览器支持)
+ 音频 (浏览器支持)
+ 视频 (浏览器支持)
+ 字幕 (srt / vtt 格式, 支持 Chrome, Firefox 和 Edge 浏览器)
+ PDF (由 pdf.js 支持)

### 规划

+ [ ] 定制媒体播放器
+ [ ] 下载文件
+ [ ] 多用户管理
+ [ ] HTTPS
+ [ ] 更多文件格式支持

### 技术栈

+ [Svelte](https://svelte.dev)
+ [Rocket](https://rocket.rs)
+ [Tailwind](https://tailwindcss.com)
+ [Pdf.js](https://mozilla.github.io/pdf.js)
+ [Plyr](https://plyr.io)

### 构建

+ Node 14.17
+ Rust 1.54
+ Python3

```
cd path/to/oasis
python3 build.py
```