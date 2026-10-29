# Lesson 01 - CHANGES

Previous lesson: a plain `cargo new` binary with only the standard library.
This lesson: a **Dioxus** project. No hand-written code. Everything below is output of `dx new`.

> Verification note: steps 1-5 (tools, `dx new`, web, desktop) were run and checked while writing this lesson on Windows 11 with `rustc 1.99`, `dx 0.7.10`. The **Android** steps (6) follow the official Dioxus mobile guide; the author's machine had no Android SDK installed, so treat step 6 as "follow it and tell us where it differs".

## 1. Rust web target

Browsers run WebAssembly, so Rust needs a compiler target for it:

```bash
rustup target add wasm32-unknown-unknown
rustup target list --installed          # must list wasm32-unknown-unknown
```

## 2. Install the `dx` tool

```bash
cargo install dioxus-cli                # compiles from source: ~5-10 minutes
# faster alternative (downloads a prebuilt binary):
#   cargo install cargo-binstall
#   cargo binstall dioxus-cli
dx --version                            # dioxus 0.7.x
dx doctor                               # prints what is installed / missing per platform
```

`dx doctor` is your friend: run it whenever a platform refuses to build. On a fresh Windows machine it reports "not found" for everything Android and iOS; that's fine until step 6.

Windows desktop apps need **WebView2**. It ships with Windows 11 and with Edge, so usually nothing to do. If a desktop window opens blank, install the "Evergreen" WebView2 runtime from Microsoft.

## 3. Create the project

`dx new` is interactive in a real terminal (it asks about template, router, tailwind...). To get the exact same project as this lesson without prompts:

```bash
dx new rdx-01-create-dioxus-project -y --vcs none \
  --subtemplate Bare-Bones \
  -o is_fullstack=false -o is_router=false -o default_platform=web -o is_tailwind=false
cd rdx-01-create-dioxus-project
```

(Interactive: pick **Bare-Bones**, no fullstack, no router, web as default platform, no Tailwind.)

Generated files:

```
.gitignore
AGENTS.md        notes for AI coding agents (not used by us; deleted in this repo)
clippy.toml      linter tweaks for signals (not used by us; deleted in this repo)
Cargo.toml
Dioxus.toml
README.md
tailwind.css  +  assets/tailwind.css   (empty Tailwind stub; deleted in this repo)
assets/favicon.ico, header.svg, main.css
src/main.rs
```

Small cleanups made in this repo's copy (the only deviations from the raw output):

- removed `authors = [...]` from `Cargo.toml` (it contains a personal e-mail),
- removed `AGENTS.md`, `clippy.toml`, and the two empty `tailwind.css` files: we style with plain CSS in `assets/`,
- `.cargo/config.toml` one level up (`rust-dioxus-learn/.cargo/config.toml`) makes **all lessons share one `target/` folder**, so Dioxus compiles once, not once per lesson.

### The generated `Cargo.toml`

```toml
[package]
name = "rdx-01-create-dioxus-project"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = { version = "0.7.1", features = [] }

[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
```

This is the **only** dependency of the whole course: `dioxus`.

## 4. Run on the web

```bash
dx serve                          # same as: dx serve --platform web
```

First build takes a few minutes (it compiles ~175 crates). Then open the printed address, normally <http://localhost:8080>. Edit `assets/main.css` (change the background colour) and save: the page updates without reload. That is **hot reload**.

Stop with `Ctrl+C`.

Build-only (no server), useful to prove it compiles:

```bash
dx build --platform web           # output: <shared target>/dx/<name>/debug/web/public
```

## 5. Run on the desktop

```bash
dx serve --platform desktop
```

A native window opens. Same page, same hot reload. Under the hood `dx` passed `--features desktop` (and turned off `web`) for you, which is what the `[features]` table is for.

## 6. Run on Android

### 6.1 Install Android Studio and the SDK pieces

1. Install **Android Studio** from <https://developer.android.com/studio>.
2. Open it, create any blank project (this only unlocks the menus).
3. `Tools > SDK Manager > SDK Tools` tab. Tick:
   - Android SDK Build-Tools / Platform-Tools (usually already there)
   - **Android SDK Command-line Tools (latest)**
   - **NDK (Side by side)**
   - **CMake**
4. Press Apply. Afterwards look at `%LOCALAPPDATA%\Android\Sdk\ndk\` and note the folder name: that is your NDK version, e.g. `27.0.12077973`.

(NDK = Native Development Kit: the C toolchain Android needs to link Rust code. Rust compiles to machine code for ARM/x86 phones; the NDK provides the linker and system libraries.)

### 6.2 Environment variables (PowerShell, then open a NEW terminal)

Replace the NDK folder name with yours.

```powershell
[Environment]::SetEnvironmentVariable("JAVA_HOME",        "C:\Program Files\Android\Android Studio\jbr", "User")
[Environment]::SetEnvironmentVariable("ANDROID_HOME",     "$env:LOCALAPPDATA\Android\Sdk", "User")
[Environment]::SetEnvironmentVariable("ANDROID_NDK_HOME", "$env:LOCALAPPDATA\Android\Sdk\ndk\<YOUR-NDK-VERSION>", "User")
[Environment]::SetEnvironmentVariable("NDK_HOME",         "$env:LOCALAPPDATA\Android\Sdk\ndk\<YOUR-NDK-VERSION>", "User")

# add adb and the emulator to PATH
$old = [Environment]::GetEnvironmentVariable("Path", "User")
[Environment]::SetEnvironmentVariable("Path", "$old;$env:LOCALAPPDATA\Android\Sdk\platform-tools;$env:LOCALAPPDATA\Android\Sdk\emulator", "User")
```

The Dioxus guide uses `NDK_HOME`, `dx doctor` mentions `ANDROID_NDK_HOME`. Setting both is harmless.

### 6.3 Rust Android targets

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### 6.4 A device to run on (choose one)

- **Emulator**: Android Studio > `Device Manager` > Create device (e.g. Pixel 6, a recent system image). Start it, or from the terminal: `emulator -list-avds` then `emulator -avd <name>`.
- **Real phone**: Settings > About > tap *Build number* 7 times > Developer options > enable **USB debugging**, plug in, accept the prompt.

Check that it is visible:

```bash
adb devices            # a line ending in "device" (not "unauthorized")
```

### 6.5 Check and run

```bash
dx doctor              # Android section should now show sdk / ndk / adb / java_home and the 4 targets
dx serve --platform android
```

The first Android build is slow (Gradle and the NDK are used). The app is installed on the device and starts; saving a file hot-reloads it.

### Android troubleshooting

| Symptom | Likely cause |
|---------|-------------|
| `dx doctor` says `ndk: not found` | `ANDROID_NDK_HOME` / `NDK_HOME` points to a folder that does not exist: compare with `ls $env:LOCALAPPDATA\Android\Sdk\ndk` |
| `java_home: not found` | `JAVA_HOME` should point at Android Studio's `jbr` folder. Reopen the terminal after setting it. |
| `adb` is not recognized | `platform-tools` is not on PATH, or the terminal was not reopened |
| `adb devices` shows `unauthorized` | accept the debugging prompt on the phone |
| app starts then shows a blank screen | wait for the first launch to finish; check `adb logcat \| findstr -i dioxus` |

## 7. What did we change in the code?

Nothing. That is the point. Next lesson we start editing `src/main.rs`.

## 8. Quick reference

```bash
dx serve                             # web
dx serve --platform desktop
dx serve --platform android
dx build --platform web              # compile only
dx bundle --platform web --release   # production build (later)
dx doctor
```
