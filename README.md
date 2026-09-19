## Kevcord Desktop app

This repo contains a lightweight standalone desktop app (built
with [Tauri](https://tauri.app)) for use with the Kevcord service. You will
need a URL for a hosted Kevcord instance.

What it does:

- Opens the Kevcord server in its own window; you enter
  the server address once and it's remembered.
- **Closing the window hides it to the system tray** so voice chat, unread
  tracking, and mention sounds keep running in the background.
- Tray menu: Open Kevcord, **Start with system** (autostart toggle), Change
  server, Quit.
- Single-instance: launching it again just focuses the running window.

### Building installers

Build on the OS you're targeting. One-time prerequisites:

- All platforms: [Node.js](https://nodejs.org/en/download) and [Rust](https://rustup.rs).
- Windows: Visual Studio Build Tools with the "Desktop development with C++"
  workload. WebView2 is preinstalled on Windows 10/11.
- Linux (Debian/Ubuntu):
  `sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev libayatana-appindicator3-dev librsvg2-dev`
- macOS: Xcode command line tools (`xcode-select --install`).

Then:

```bash
cd 'Kevcord Desktop'
npm install
npm run build
```

Installers land in `Kevcord Desktop/src-tauri/target/release/bundle/`:
`nsis/Kevcord_*.exe` on Windows, `deb/` and `appimage/` on Linux, `dmg/` on
macOS. Run that file, type the server address,
and sign in/sign up. For development, `npm run dev` opens the app with hot reload of
the connect page.

Notes:

- Voice chat works in the webview; the OS prompts for microphone access on
  first use.
- Mentions and DMs show native system notifications while the app is in the
  background (version 1.0.2 and later). They are on by default; the bell in the
  channel header turns them off. On Windows they appear once the app has been
  installed with its installer, not when running the .exe from the build folder.