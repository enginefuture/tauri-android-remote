use clap::{Parser, Subcommand};
use remote_core::Connection;
use std::{
    net::{SocketAddr, TcpStream},
    path::PathBuf,
    process::{Command, ExitCode},
    time::Duration,
};
#[derive(Parser)]
#[command(version, about = "ADB and scrcpy over your private tailnet")]
struct Args {
    #[arg(long)]
    ip: String,
    #[arg(long)]
    port: u16,
    #[command(subcommand)]
    action: Action,
}
#[derive(Subcommand)]
enum Action {
    /// Uses --port as the pairing port; adb asks for the code interactively.
    Pair,
    Connect,
    Doctor,
    Screen,
    Install {
        apk: PathBuf,
    },
    Logs,
    Shell,
    Disconnect,
}
fn binary(name: &str) -> PathBuf {
    if let Some(p) = std::env::var_os(format!("REMOTE_{}_BIN", name.to_uppercase())) {
        return p.into();
    }
    if name == "adb" {
        for root in [
            std::env::var_os("ANDROID_HOME").map(PathBuf::from),
            std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Android/sdk")),
        ]
        .into_iter()
        .flatten()
        {
            let p = root.join("platform-tools/adb");
            if p.is_file() {
                return p;
            }
        }
    }
    name.into()
}
fn run(name: &str, args: &[&str]) -> Result<(), String> {
    let mut command = Command::new(binary(name));
    if name == "scrcpy" {
        command.env("ADB", binary("adb"));
    }
    let status = command.args(args).status().map_err(|e| {
        format!(
            "无法运行 {name}: {e}。安装工具或设置 REMOTE_{}_BIN",
            name.to_uppercase()
        )
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{name} 退出状态: {status}"))
    }
}
fn main() -> ExitCode {
    match execute(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
fn execute(a: Args) -> Result<(), String> {
    let ep = Connection {
        ip: a.ip,
        port: a.port,
    }
    .endpoint()?;
    match a.action {
        Action::Pair => run("adb", &["pair", &ep]),
        Action::Connect => {
            run("adb", &["connect", &ep])?;
            run("adb", &["-s", &ep, "get-state"])
        }
        Action::Disconnect => run("adb", &["disconnect", &ep]),
        Action::Screen => run("scrcpy", &["-s", &ep]),
        Action::Logs => run("adb", &["-s", &ep, "logcat"]),
        Action::Shell => run("adb", &["-s", &ep, "shell"]),
        Action::Install { apk } => {
            let p = apk.canonicalize().map_err(|e| e.to_string())?;
            if !p.is_file() {
                return Err("APK 必须是文件".into());
            }
            run(
                "adb",
                &[
                    "-s",
                    &ep,
                    "install",
                    "-r",
                    p.to_str().ok_or("文件名必须为 UTF-8")?,
                ],
            )
        }
        Action::Doctor => {
            run("adb", &["version"])?;
            let addr: SocketAddr = ep.parse().map_err(|_| "无效地址")?;
            TcpStream::connect_timeout(&addr, Duration::from_secs(4)).map_err(|e| {
                format!(
                    "{ep} TCP 不可达: {e}。检查 Tailscale、访问策略、无线调试开关和当前连接端口。"
                )
            })?;
            println!("TCP 可达，继续检查 ADB 授权状态");
            run("adb", &["-s", &ep, "get-state"])
        }
    }
}
