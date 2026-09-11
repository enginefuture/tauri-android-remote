# Android Remote

An open-source **Tauri 2 + Rust Android companion** for connecting an authorized Mac to your Android phone using **Tailscale + ADB + scrcpy**.

**Alpha:** this app guides setup and generates commands. Android's system `adbd` executes commands; the app does not implement an ADB server, grant itself shell access, or enable debugging automatically. No root, accessibility service, embedded VPN, or custom control server is required. Android 11+; ARM64 APK provided for testing when available.

## 中文快速开始

1. 手机和 Mac 登录同一个 Tailscale 网络，并确认访问策略允许 Mac 访问手机调试端口。
2. 手机设置 → 开发者选项 → 无线调试 → 使用配对码配对设备。首次需手动授权，保持配对弹窗打开。
3. Mac 安装 Android Platform Tools 和 scrcpy（例如 `brew install android-platform-tools scrcpy`）。
4. 执行 `adb pair <手机的Tailscale-IP>:<配对端口>`，按提示输入配对码。
5. 回到无线调试主页面，查看**连接端口**，执行 `adb connect <手机的Tailscale-IP>:<连接端口>`。
6. 执行 `adb -s <手机的Tailscale-IP>:<连接端口> get-state`，确认返回 `device`。
7. 安装本项目 APK，打开应用核对设备信息、生成后续连接命令。用 `scrcpy -s <手机的Tailscale-IP>:<连接端口> --force-adb-forward --no-audio --max-size=1280` 看屏幕和操作手机。

配对端口和连接端口不同。Tailscale 不会自动开启 ADB，也不能保证所有厂商 ROM 上无线调试端口均可从 VPN 访问。首次可先在同一 Wi-Fi 完成配对，再验证 Tailscale 地址。不要依赖跨 tailnet 的 mDNS 自动发现。

## Mac Rust CLI

```sh
cargo install --path crates/remote-cli --locked
android-remote --help
android-remote --ip 100.64.0.10 --port 37001 pair
android-remote --ip 100.64.0.10 --port 39001 connect
android-remote --ip 100.64.0.10 --port 39001 doctor
android-remote --ip 100.64.0.10 --port 39001 screen
android-remote --ip 100.64.0.10 --port 39001 install ./app.apk
android-remote --ip 100.64.0.10 --port 39001 logs
android-remote --ip 100.64.0.10 --port 39001 shell
android-remote --ip 100.64.0.10 --port 39001 disconnect
```

Example addresses/ports above are placeholders. `pair` asks for the code interactively through adb; codes are not stored. All other commands use the connection port. `screen`, `install`, `logs` and `shell` require an existing connection. Ctrl-C stops interactive sessions. `doctor` checks adb availability, TCP reachability (4-second timeout), then ADB device state. `connect` verifies `get-state` because adb may return exit code 0 even when connection fails.

`screen` uses ADB forward mode, disables audio and limits the longest video edge to 1280 pixels. This configuration produced video on the tested Realme GT8 Pro where the default reverse tunnel received no video headers. `install` uses `--no-streaming`, which succeeded after the vendor rejected the initial streamed install.

The CLI uses argument arrays, not a shell. It accepts only IPv4 addresses within `100.64.0.0/10`; this range check is not proof of Tailscale identity. It finds adb via `ANDROID_HOME`, the standard macOS SDK path, or PATH. Override with `REMOTE_ADB_BIN` / `REMOTE_SCRCPY_BIN`. IPv6 and MagicDNS names are not yet supported.

## Android app

- Native device model, Android version and developer-options status.
- Candidate Tailscale IPv4 addresses from local interfaces; confirm them in Tailscale.
- Open Tailscale and developer settings through a small Kotlin plugin.
- Save IP/connection port locally and generate validated Mac commands in Rust.
- No telemetry or remote listener. Pairing codes and ADB keys are never stored in the app.

## Build

Install [Tauri Android prerequisites](https://v2.tauri.app/start/prerequisites/): Node, pnpm, Rust, JDK 17, Android SDK 36 and NDK 28.2.13676358. Set `JAVA_HOME`, `ANDROID_HOME`, `NDK_HOME` for your machine.

```sh
rustup target add aarch64-linux-android
pnpm install --frozen-lockfile
pnpm build
cargo fmt --all -- --check
cargo test -p remote-core -p remote-cli --locked
pnpm tauri android init --ci
pnpm tauri android build --debug --apk --target aarch64 --ci
```

Debug APKs are for testing. Production distribution requires your own persistent signing key, kept out of git. Generated Android source is committed; build outputs, local configuration, private keys and device-specific settings are ignored.

## Architecture

```text
Mac Rust CLI -> adb / scrcpy -> Tailscale -> Android system adbd
Android Tauri UI -> Rust validation -> Kotlin Android settings/status adapter
```

`crates/remote-core` validates endpoints; `crates/remote-cli` manages Mac commands; `src-tauri` hosts the app; `plugins/device` adapts native Android APIs. The app can be closed after setup: it does not keep ADB alive or turn it off.

## Recovery and limits

- Phone reboot / network change: check Tailscale, wireless debugging and the current connection port, then reconnect. No unattended reboot recovery is promised.
- Wi-Fi disconnected: some systems disable wireless debugging. Cellular-only access needs explicit device testing.
- Unauthorized/offline: check the phone's pairing list, pair again if necessary, then reconnect. `disconnect` only disconnects this Mac; revoke the Mac under Android wireless debugging to remove authorization.
- A reachable TCP port alone does not prove ADB is authorized.
- Optional USB fallback: authorize this Mac over USB, then `adb -s <USB_SERIAL> tcpip 5555`; connect through Tailscale. This may listen on LAN interfaces too, and may reset after reboot. Tailscale policy does not protect direct LAN access. Prefer wireless-debugging TLS pairing.
- Screen control is provided by scrcpy, not root access. Protected content and manufacturer input restrictions may limit functionality.

See [SECURITY.md](SECURITY.md), [validation checklist](docs/VALIDATION.md), and the [小红书真机发布操作手册](docs/XIAOHONGSHU_WORKFLOW.md).

References: [Android ADB](https://developer.android.com/tools/adb), [Tauri mobile plugins](https://v2.tauri.app/develop/plugins/develop-mobile/), [scrcpy](https://github.com/Genymobile/scrcpy).

MIT licensed.
