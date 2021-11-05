## OASIS

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/machengim/oasis/blob/main/LICENSE-MIT) ![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/machengim/oasis)

[中文 README](https://github.com/machengim/oasis/blob/main/README_cn.md)

A self-hosted file server.

![](https://github.com/machengim/oasis/blob/main/doc/Oasis_demo.jpg?raw=true)

### Install

1. Download from the [release](https://github.com/machengim/oasis/releases) page
2. Uncompress
3. (Optional) Config server IP and port number in `oasis.conf` file
3. Run `oasis` or `oasis.exe`
4. Visit the server's IP address in your favorite browser

### Features

- User authentication
- File preview
- File download
- File upload
- Play list
- Mobile compatibility
- Temporary sharing link
- External media player support via sharing link
- Multiple platform support
- I18n (English, Chinese)

### File format support

- Text
- Image (browser support)
- Audio (browser support)
- Video (browser support)
- Subtitle (srt / vtt format, supported in Chrome, Firefox and Edge by now)
- PDF (supported by pdf.js)

### Roadmap

- [ ] Multiple users management
- [ ] HTTPS
- [ ] More file format support

### Tech stack

- [Svelte](https://svelte.dev)
- [Rocket](https://rocket.rs)
- [Tailwind](https://tailwindcss.com)

### Credit

- [Pdf.js](https://mozilla.github.io/pdf.js)
- [Plyr](https://plyr.io)

### Build process

- Node 14.17+
- Rust 1.54+

```
cd path/to/oasis
node build.js
```
