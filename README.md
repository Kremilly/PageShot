# PageShot

Capture screenshots of web pages from specified URL using Rust. Customize viewport dimensions and save the resulting image in PNG format. You can customize the viewport dimensions and save the resulting image in PNG format.

## Features

- Capture screenshots from any URL.
- Customize viewport width and height.
- Full-page screenshots that capture entire scrollable content.
- Multiple output formats: PNG, JPEG, WebP.
- Quality control for JPEG and WebP formats.
- Simple command-line interface.

## Installation

To build and run PageShot, make sure you have Rust and Cargo installed. Clone this repository and use Cargo to build the project:

```sh
cargo install pageshot
```

## Usage

Run the compiled binary with the desired URL, viewport dimensions, and output file name:

```sh
pageshot -u https://example.com --width 1920 --height 1080 -o example.png

# Capture full-page screenshot (entire scrollable content)
pageshot -u https://example.com -f -o example_fullpage.png

# JPEG format with quality control (smaller file size)
pageshot -u https://example.com --format jpeg --quality 85 -o example.jpg

# WebP format for best compression
pageshot -u https://example.com --format webp --quality 90 -o example.webp

# Full-page JPEG with lower quality for smaller file size
pageshot -u https://example.com -f --format jpeg --quality 70 -o fullpage.jpg
```

### Arguments

- `-u, --url <URL>`: The URL of the web page to capture.
- `--width <WIDTH>`: The width of the viewport (default: 1920).
- `--height <HEIGHT>`: The height of the viewport (default: 1080).
- `-o, --output <FILE>`: The name of the output file (default: `screenshot.png`).
- `-f, --full-page`: Capture the entire scrollable page content, not just the viewport.
- `--format <FORMAT>`: Output format - `png`, `jpeg`, or `webp` (default: `png`).
- `--quality <QUALITY>`: Quality for JPEG/WebP, 0-100 where higher is better (default: 85).

### Format Recommendations

- **PNG**: Lossless quality, best for documentation and pixel-perfect captures. Larger file size.
- **JPEG**: Good for general web captures. Use quality 70-85 for balanced size/quality, 90-100 for high quality.
- **WebP**: Modern format with best compression. Recommended for sharing and storage efficiency.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
