# Video2WebP

A simple desktop application built with Tauri, Vue 3, and Rust to easily convert video files into animated WebP images.

> **Note**: This application is currently designed and configured for **macOS devices with Apple Silicon (M1/M2/M3/etc.)**.

## Features

- **Drag & Drop Interface**: Easily add videos by dragging them into the application window.
- **Batch Conversion**: Queue multiple files and convert them all at once.
- **Customizable Settings**: Control output quality with adjustable parameters:
    - **FPS**: Set the frame rate for smooth animations or smaller file sizes.
    - **Quality**: Adjust compression level.
    - **Scale**: Resize the output dimension.
    - **Looping**: Toggle animation looping.
    - **Skip Frames**: Skip the initial N frames of the video.
- **Conversion Management**:
    - **Clear All**: Remove all items from the queue.
    - **Clear Done**: Remove only completed conversions.
    - **Overwriting**: Option to overwrite existing files.
- **Optimized for Apple Silicon**: Leverages native performance on M-series Macs.

## Tech Stack

- **Frontend**: Vue 3, TypeScript, Vite
- **Backend & Window Management**: Rust, Tauri
- **Media Processing**: FFmpeg

## Prerequisites

Before running the application, ensure you have the following installed:

- [Node.js](https://nodejs.org/) (Latest LTS recommended)
- [Rust](https://www.rust-lang.org/tools/install)
- [FFmpeg](https://ffmpeg.org/download.html) (Sidecar binary required)

## Installation & Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/video2webp.git
   cd video2webp
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **FFmpeg Setup:**
   The application requires FFmpeg binaries to be placed in `src-tauri/binaries/`.
   You can use the included script to download them (if available) or download them manually.
   *Note: Ensure the binary name matches the target platform (e.g., `ffmpeg-aarch64-apple-darwin` for Apple Silicon Mac).*
   ```bash
   npm run download:ffmpeg
   ```

4. **Run the application in development mode:**
   ```bash
   npm run tauri dev
   ```

5. **Build for production:**
   ```bash
   npm run tauri build
   ```

## Usage

1. Launch the application.
2. Drag and drop video files onto the designated area.
3. Adjust settings in the "Settings" panel on the right.
4. Click "Convert All" to start processing.
5. Converted files will be saved in the same directory as the source video.

## Project Structure

- `src/`: Vue frontend components and logic.
- `src-tauri/`: Rust backend and Tauri configuration.
- `scripts/`: Utility scripts (e.g., ffmpeg downloader).
