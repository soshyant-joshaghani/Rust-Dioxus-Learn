# Dioxus cheat sheet

Personal quick reference for bare `dx` and the FoxG `rust-dioxus` kit wrappers.
Full kit docs: [`rust-dioxus/__ctrl__/README.md`](rust-dioxus/__ctrl__/README.md).

---

## Bare `dx` (any Dioxus crate)

```bat
dx --help
dx doctor
dx self-update

dx new my-app
dx init

dx serve --web
dx serve --desktop
dx serve --windows
dx serve --android
dx serve --ios

dx serve --platform web
dx serve --platform android --device <serial>
dx serve --android --target aarch64-linux-android

dx build --web
dx run --desktop
dx bundle --android --package-types apk
dx bundle --windows --package-types nsis
dx bundle --release --out-dir dist

dx fmt
dx check
dx translate <file>
```

| Flag | Notes |
|------|--------|
| `--web` / `--desktop` / `--android` / … | Platform shorthand (same as `--platform …`) |
| `--release` / `-r` | Release build |
| `--features <…>` | Cargo features (e.g. `mobile`, `web`) |
| `--target <triple>` | Rustc triple — **critical on Android** |
| `--device <name\|serial>` | Phone / emulator; without it Android defaults to **emulator ABI (x86_64)** |
| `--open false` | Don’t auto-open browser / window |
| `--hot-reload false` | Disable hot reload |
| `--port` / `--addr` | Dev server bind (web) |

### Android (the gotchas)

Bare `dx` does **not** auto-set SDK/NDK. The kit does.

```powershell
# once per shell (or set User env permanently)
$env:ANDROID_HOME = "$env:LOCALAPPDATA\Android\Sdk"
$env:ANDROID_NDK_HOME = "$env:ANDROID_HOME\ndk\<version>"   # e.g. 29.0.14206865

$env:ANDROID_HOME = "$env:LOCALAPPDATA\Android\Sdk"
$env:ANDROID_NDK_HOME = "$env:ANDROID_HOME\ndk\29.0.14206865"

adb devices
dx serve --android --device <serial>          # physical phone → arm64
# or
dx serve --android --target aarch64-linux-android
```

| Symptom | Cause | Fix |
|---------|--------|-----|
| `ANDROID_NDK_HOME` not set | Env missing | Set `ANDROID_HOME` + `ANDROID_NDK_HOME` |
| `INSTALL_FAILED_NO_MATCHING_ABIS` | APK is x86_64, phone is arm64 | `--device <serial>` or `--target aarch64-linux-android` |
| Activity class does not exist | Install failed; nothing to launch | Fix ABI / reinstall |

Phone ABI check:

```bat
adb shell getprop ro.product.cpu.abi
```

API from device → host (kit does this for you):

```bat
adb reverse tcp:8000 tcp:8000
```

### Common rustup targets

```bat
rustup target add wasm32-unknown-unknown
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
rustup target add armv7-linux-androideabi
```

---

## Kit: `rust-dioxus-ctrl` (preferred for FoxG)

From `rust-dioxus/__ctrl__/`:

```bat
rust-dioxus-ctrl.bat
rust-dioxus-ctrl.bat setup-local
rust-dioxus-ctrl.bat list
rust-dioxus-ctrl.bat connect
```

### Dev stack

```bat
rust-dioxus-ctrl.bat dev run all
rust-dioxus-ctrl.bat dev run all --slim
rust-dioxus-ctrl.bat dev stop all
rust-dioxus-ctrl.bat dev down all
rust-dioxus-ctrl.bat dev purge infra
rust-dioxus-ctrl.bat dev reset all
```

| Target | Meaning |
|--------|---------|
| `infra` | Docker only (db, redis, proxy, …) |
| `apps` | Host API :8000 + worker + `dx serve --web` :5000 |
| `all` | Both |

### Native clients (`frontend/`)

Auto-fills Android env, picks attached phone, defaults arm64 for builds.

```bat
rust-dioxus-ctrl.bat native list
rust-dioxus-ctrl.bat native doctor
rust-dioxus-ctrl.bat native setup android
rust-dioxus-ctrl.bat native run desktop
rust-dioxus-ctrl.bat native run android
rust-dioxus-ctrl.bat native run android --device <serial>
rust-dioxus-ctrl.bat native build windows
rust-dioxus-ctrl.bat native build android --package-types apk,aab --api https://api.example.com/api/v1
rust-dioxus-ctrl.bat native build all
rust-dioxus-ctrl.bat native clean all
```

| Platform id | `dx` flag | Build packages |
|-------------|-----------|----------------|
| `web` | web | — |
| `desktop` | desktop (host OS) | — |
| `windows` | windows | msi, nsis |
| `macos` | macos | macos, dmg |
| `linux` | linux | deb, appimage, rpm |
| `android` | android | apk, aab |
| `ios` | ios (mac only) | ios, ipa |

### Tests / scaffold / prod / SSH

```bat
rust-dioxus-ctrl.bat app create <name>
rust-dioxus-ctrl.bat test all
rust-dioxus-ctrl.bat test backend
rust-dioxus-ctrl.bat test frontend
rust-dioxus-ctrl.bat test contract

rust-dioxus-ctrl.bat prod start|stop|reset|backup-acme

rust-dioxus-ctrl.bat setup
rust-dioxus-ctrl.bat pubkey
rust-dioxus-ctrl.bat clone
rust-dioxus-ctrl.bat env
rust-dioxus-ctrl.bat start
rust-dioxus-ctrl.bat stop
rust-dioxus-ctrl.bat update
rust-dioxus-ctrl.bat status
rust-dioxus-ctrl.bat backup-acme
rust-dioxus-ctrl.bat connect
```

---

## Mental model

| Context | Use |
|---------|-----|
| FoxG `rust-dioxus` day-to-day | `rust-dioxus-ctrl.bat …` |
| Scratch / external Dioxus crate | bare `dx …` + set Android env yourself |
| Phone install fails with ABI error | pass `--device` or `--target aarch64-linux-android` |
| Kit Android run | env + `adb reverse` + device selection done for you |
