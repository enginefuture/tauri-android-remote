#![cfg(unix)]
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
#[test]
fn dispatch_verifies_state_and_never_uses_shell_input() {
    let dir = std::env::temp_dir().join(format!(
        "android-remote-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&dir).unwrap();
    let tool = dir.join("fake-tool");
    let log = dir.join("args");
    fs::write(&tool,"#!/bin/sh\nprintf '%s\\n' \"$@\" >> \"$TEST_LOG\"\ncase \"$*\" in *get-state*) exit 1;; esac\nexit 0\n").unwrap();
    fs::set_permissions(&tool, fs::Permissions::from_mode(0o700)).unwrap();
    let run = |ip: &str, action: &str| {
        Command::new(env!("CARGO_BIN_EXE_android-remote"))
            .args(["--ip", ip, "--port", "39001", action])
            .env("REMOTE_ADB_BIN", &tool)
            .env("REMOTE_SCRCPY_BIN", &tool)
            .env("TEST_LOG", &log)
            .output()
            .unwrap()
    };
    assert!(!run("100.64.0.10", "connect").status.success());
    assert_eq!(
        fs::read_to_string(&log).unwrap(),
        "connect\n100.64.0.10:39001\n-s\n100.64.0.10:39001\nget-state\n"
    );
    fs::write(&log, "").unwrap();
    assert!(!run("100.64.0.10;id", "screen").status.success());
    assert_eq!(fs::read_to_string(&log).unwrap(), "");
    assert!(run("100.64.0.10", "screen").status.success());
    assert_eq!(fs::read_to_string(&log).unwrap(), "-s\n100.64.0.10:39001\n");
    fs::remove_dir_all(dir).unwrap();
}
