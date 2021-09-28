## OASIS

Minimalized file hosting and sharing app.

### Features

+ User authentication
+ File preview
+ Mobile compatibility
+ Temporary sharing link
+ Loop file list
+ Multiple platform support
+ I18n (English, Chinese)

### File format support

+ Text
+ Image (browser support)
+ Audio (browser support)
+ Video (browser support)
+ Subtitle (srt / vtt)
+ PDF (supported by pdf.js)

### Roadmap

+ [ ] Custom media player
+ [ ] Multiple users management
+ [ ] HTTPS
+ [ ] Server side transcoding
+ [ ] More file format support

### Tech stack

+ [https://svelte.dev](Svelte)
+ [https://rocket.rs](Rocket)
+ [https://tailwindcss.com](Tailwind)
+ [https://mozilla.github.io/pdf.js](Pdf.js)
+ [https://plyr.io](Plyr)

### Build requirements

+ Node 14.17
+ Rust 1.54
+ Python3

```
cd path/to/oasis
python3 build.py
```