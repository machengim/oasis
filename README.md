## OASIS

Minimalized file hosting and sharing app.

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
+ Subtitle (srt / vtt)
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

### Build requirements

+ Node 14.17
+ Rust 1.54
+ Python3

```
cd path/to/oasis
python3 build.py
```