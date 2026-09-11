# Validation

## Automated

- TypeScript check and Vite production build.
- Rust endpoint boundary and injection-like input rejection tests.
- CLI subprocess integration tests: failed connection state must fail the command; explicit target serial passed to scrcpy; invalid address cannot launch a tool.
- ARM64 Android debug APK compilation (Rust, JNI, Kotlin, Gradle).

## Real-device acceptance (requires phone authorization)

- [x] Install and launch on Android 11+.
- [x] Native device status displays correctly; developer settings and Tailscale buttons open their apps.
- [x] Connection parameters persist across app update/restart and Rust generates commands on the phone.
- [ ] Invalid-input feedback on the physical device (covered by frontend and Rust checks).
- [x] Mac pairs through the tailnet and `get-state` returns `device`.
- [x] scrcpy produces a decodable video stream with visible app content.
- [x] ADB tap, swipe and text input operate the phone.
- [ ] Mac mouse input through the scrcpy window (desktop automation could not address the standalone executable).
- [x] APK install and logcat work against the explicit serial.
- [ ] Repeat connection from a different network, then with the phone locked.
- [ ] Reboot the phone and document the steps actually required to reconnect.

Compilation and tailnet reachability do not establish real-device acceptance. No unattended-access guarantee is made.

## Realme GT8 Pro / RMX5200, Android 16 (API 36)

Verified on 2026-09-11: TLS pairing and an explicit ADB connection through the phone's Tailscale IPv4 address; `doctor` reports TCP reachable and ADB `device`. APK install/update, app-scoped logcat, screenshot capture, tap/swipe/text input, native device status, settings/Tailscale shortcuts, saved connection parameters and Rust command generation all passed.

The first streamed APK installation returned vendor status `-99` while the phone was locked. This code alone does not identify the cause. Retrying with `adb -s <serial> install --no-streaming -r <apk>` succeeded, and the updated CLI uses that method.

Default scrcpy sessions did not receive video headers. With `--force-adb-forward --no-audio --max-size=1280`, a 12.01-second local recording contained 124 H.264 frames at 586×1280; a decoded frame visibly showed the app. These parameters now ship in the CLI and generated commands. Mac mouse interaction through scrcpy remains a manual acceptance item; ADB input was separately verified.

This run used two devices on the same underlying Wi-Fi via Tailscale addresses. Cross-network, cellular-only and post-reboot recovery remain untested. No device addresses, pairing codes, recordings or screenshots are included in this repository.
