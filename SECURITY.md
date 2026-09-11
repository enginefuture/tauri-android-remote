# Security

Use only devices you own or are authorized to operate. Keep Android's ADB authentication enabled and restrict tailnet access to trusted computers. An authorized ADB client can install applications, access shell-visible data and operate the phone.

This application does not accept remote requests, open a listening port, enable ADB, or alter Tailscale policies. Endpoint range validation reduces accidental connections but does not authenticate a peer. Confirm the phone address in your tailnet.

Pairing codes are entered interactively in adb. ADB keys remain managed by Android Platform Tools in the user's standard local location. Never commit them, signing keystores, local logs, or real device configuration.

Closing the app or running CLI `disconnect` does not revoke ADB authorization. Disable wireless debugging or forget the paired Mac in Android settings. Traditional `adb tcpip 5555` may also expose ADB on LAN interfaces outside Tailscale's policy enforcement.

For a vulnerability report, use GitHub private vulnerability reporting if enabled; do not include secrets in public issues.
