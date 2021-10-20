## OASIS

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/machengim/oasis/blob/master/LICENSE-MIT) ![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/machengim/oasis)

Minimalized file hosting and sharing app.

### Install

1. Download from the release page
2. Uncompress
3. Run `oasis` or `oasis.exe`
4. Visit the server's IP address in your favorite browser

### Features

+ User authentication
+ File preview
+ Play list
+ Mobile compatibility
+ Temporary sharing link
+ Multiple platform support
+ I18n (English, Chinese)

### File format support

+ Text
+ Image (browser support)
+ Audio (browser support)
+ Video (browser support)
+ Subtitle (srt / vtt format, tested in Chrome, Firefox and Edge by now)
+ PDF (supported by pdf.js)

### Roadmap

+ [ ] Custom media player
+ [ ] Multiple users management
+ [ ] HTTPS
+ [ ] More file format support

### Tech stack

+ [Svelte](https://svelte.dev)
+ [Rocket](https://rocket.rs)
+ [Tailwind](https://tailwindcss.com)
+ [Pdf.js](https://mozilla.github.io/pdf.js)
+ [Plyr](https://plyr.io)

### Build process

+ Node 14.17
+ Rust 1.54
+ Python3

```
cd path/to/oasis
python3 build.py
```