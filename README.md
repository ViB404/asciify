<h1 align="center">
  ASCIIFY
</h1>

---
![Rust](https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![GitHub Repo stars](https://img.shields.io/github/stars/ViB404/asciify?style=for-the-badge)
![GitHub issues](https://img.shields.io/github/issues/ViB404/asciify?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)
---
A command line tool written in Rust that decodes video and plays it in your terminal as ASCII art.

## How it works:
- Using the `vid2img` crate, we convert the video into frames and save them to RAM (so it doesn't feel laggy when playing).
- After that, we use the `image` crate to convert the frames into grayscale.
- `image_ascii` converts each grayscale image to ASCII and prints it.

---

## Installation:
1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

2. Install Project Dependencies: Visit https://gitlab.freedesktop.org/gstreamer/gstreamer-rs#installation to install the tools needed to convert video into frames.

### Build and run:

3. Clone the Git repository:

```bash
git clone https://github.com/vib404/asciify
cd asciify
```

4. Replace `testing_video.mp4` with your preferred video.
5. Change the palette you want to use in the `image_ascii.rs` file:

```rust
// Available palettes: light, medium, dark
let palette = dark;
```

6. Run the project (Recommended: `--release` flag):
   Since video decoding and image processing are CPU intensive tasks, use the `--release` flag rather than standard debug mode for full speed.

```bash
cargo run --release
```

Here is the corrected version of your dependencies section.

---
## Dependencies
```toml
[dependencies]
image = "0.25.10"
vid2img = "0.1.0"
clearscreen = "4.0.6"
```

* `image`: For image processing
* `vid2img`: For video to frame conversion
* `clearscreen`: For clearing the terminal
---

## ⚠️ Troubleshooting Errors:

* **no element "pngenc" or no element "decodebin":** You are missing GStreamer plugins. Double-check step 2 and ensure you installed the `good` and `base` plugins.
* **Video looks stretched/squashed:** Terminal fonts are taller than they are wide. The player automatically compensates by pulling frames at a 16:9 terminal aspect ratio (e.g., 120x34). Adjust the window size (change `frame_size` in the `video_to_frame.rs` file) of your terminal for the best viewing experience.