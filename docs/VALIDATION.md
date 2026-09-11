# Validation

## Automated

- TypeScript check and Vite production build.
- Rust endpoint boundary and injection-like input rejection tests.
- CLI subprocess integration tests: failed connection state must fail the command; explicit target serial passed to scrcpy; invalid address cannot launch a tool.
- ARM64 Android debug APK compilation (Rust, JNI, Kotlin, Gradle).

## Real-device acceptance (requires phone authorization)

- [ ] Install and launch on Android 11+.
- [ ] Native device status displays correctly; developer settings and Tailscale buttons open their apps.
- [ ] Parameters persist across app restart; invalid values produce an error.
- [ ] Mac pairs through the tailnet and `get-state` returns `device`.
- [ ] scrcpy displays the phone and accepts user input.
- [ ] APK install and logcat work against the explicit serial.
- [ ] Repeat connection from a different network, then with the phone locked.
- [ ] Reboot the phone and document the steps actually required to reconnect.

Compilation and tailnet reachability do not establish real-device acceptance. No unattended-access guarantee is made.
