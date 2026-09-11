# Validation

## Automated

- TypeScript check and Vite production build.
- Rust endpoint boundary and injection-like input rejection tests.
- CLI subprocess integration tests: failed connection state must fail the command; explicit target serial passed to scrcpy; invalid address cannot launch a tool.
- ARM64 Android debug APK compilation (Rust, JNI, Kotlin, Gradle).

## Real-device acceptance (requires phone authorization)

- [x] Install and launch on Android 11+.
- [ ] Native device status displays correctly; developer settings and Tailscale buttons open their apps.
- [ ] Parameters persist across app restart; invalid values produce an error.
- [x] Mac pairs through the tailnet and `get-state` returns `device`.
- [ ] scrcpy displays the phone and accepts user input.
- [x] APK install and logcat work against the explicit serial.
- [ ] Repeat connection from a different network, then with the phone locked.
- [ ] Reboot the phone and document the steps actually required to reconnect.

Compilation and tailnet reachability do not establish real-device acceptance. No unattended-access guarantee is made.

## Realme GT8 Pro / RMX5200, Android 16 (API 36)

Verified on 2026-09-11: TLS pairing and an explicit ADB connection through the phone's Tailscale IPv4 address; `doctor` reports TCP reachable and ADB `device`. Shell queries and screenshot capture work while the phone remains locked. scrcpy's Android server starts successfully; interactive display/control validation remains pending.

The first streamed APK installation returned vendor status `-99` while the phone was locked. This code alone does not identify the cause. Check phone-side installation confirmation and retry with the phone unlocked; do not disable ADB authentication. Retrying with `adb -s <serial> install --no-streaming -r <apk>` succeeded. The app launched and its WebView rendered correctly. App-scoped logcat confirms the native `device.status` command was invoked. The phone subsequently locked, so form persistence and interactive control remain unchecked.

This run used two devices on the same underlying Wi-Fi via Tailscale addresses. Cross-network, cellular-only and post-reboot recovery remain untested. No device addresses, pairing codes or screenshots are included in this repository.
