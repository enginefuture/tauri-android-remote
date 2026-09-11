import { invoke, isTauri } from "@tauri-apps/api/core";
import "./style.css";
document.querySelector<HTMLDivElement>("#app")!.innerHTML = `<main>
<header><span class="mark">↗</span><span>ANDROID REMOTE<small>私人设备 · 远程调试</small></span><span class="badge">ALPHA</span></header>
<section class="hero"><p class="eyebrow">YOUR PHONE, WITHIN REACH</p><h1>让手机<br>随时触手可及。</h1><p>通过 Tailscale 连接你的 Mac，<br>使用 ADB 调试，用 scrcpy 操作屏幕。</p></section>
<section class="card"><div class="row"><h2>设备状态</h2><button id="refresh" class="quiet">刷新</button></div><p id="device">正在读取…</p><p class="hint">检测到的 100.x 地址只是候选地址，请与 Tailscale 核对。</p><button id="tailscale">打开 Tailscale ↗</button></section>
<section class="card"><h2><span class="step">01</span>开启无线调试</h2><p>在开发者选项中启用「无线调试」。首次连接时，点「使用配对码配对设备」。</p><button id="settings">打开开发者选项 ↗</button><p class="hint">需要手机手动授权。应用无法替你开启 ADB。</p></section>
<section class="card"><h2><span class="step">02</span>连接你的 Mac</h2><label>Tailscale IPv4<input id="ip" placeholder="100.x.x.x" autocomplete="off" inputmode="decimal"></label><label>连接端口<input id="port" placeholder="无线调试主页面显示的端口" inputmode="numeric" type="number" min="1" max="65535"></label><button id="generate" class="primary">保存并生成连接命令</button><pre id="commands">填写地址和连接端口后生成命令。</pre><p class="hint">首次先在 Mac 执行 adb pair IP:配对端口，并按提示输入配对码。配对端口 ≠ 连接端口。配对码不在本应用保存。</p></section>
<section class="card subtle"><h2>连接不上？</h2><p>确认两端 Tailscale 在线、访问策略放行、手机无线调试开启。切换网络或重启后，重新查看端口并连接。</p><p>关闭本应用不会关闭系统 ADB。停止远程调试，请在系统设置关闭无线调试或撤销配对。</p></section><footer>TAURI 2 + RUST · OPEN SOURCE</footer><p id="notice" role="status" aria-live="polite"></p></main>`;
const el = <T extends HTMLElement>(id: string) =>
  document.getElementById(id) as T;
function notice(s: string) {
  el("notice").textContent = s;
}
async function action(command: string) {
  if (!isTauri()) {
    notice("请安装安卓 APK 后使用此功能。");
    return;
  }
  try {
    await invoke(command);
    notice("已打开系统页面");
  } catch (e) {
    notice(String(e));
  }
}
async function refresh() {
  if (!isTauri()) {
    el("device").textContent = "网页预览 · 安装 APK 后可读取手机状态";
    return;
  }
  try {
    const s = await invoke<{
      model: string;
      android: string;
      candidateIps: string[];
      developerEnabled: boolean;
    }>("device_status");
    el("device").textContent =
      `${s.model} · Android ${s.android} · 开发者选项${s.developerEnabled ? "已开启" : "未开启"}${s.candidateIps.length ? " · " + s.candidateIps.join(", ") : ""}`;
    if (!el<HTMLInputElement>("ip").value && s.candidateIps.length === 1)
      el<HTMLInputElement>("ip").value = s.candidateIps[0];
  } catch (e) {
    el("device").textContent = String(e);
  }
}
try {
  const saved = JSON.parse(localStorage.getItem("connection") || "{}");
  el<HTMLInputElement>("ip").value = saved.ip || "";
  el<HTMLInputElement>("port").value = saved.port || "";
} catch {
  localStorage.removeItem("connection");
}
el("refresh").onclick = refresh;
el("settings").onclick = () => action("open_developer_settings");
el("tailscale").onclick = () => action("open_tailscale");
el("generate").onclick = async () => {
  try {
    const ip = el<HTMLInputElement>("ip").value.trim();
    const port = Number(el<HTMLInputElement>("port").value);
    if (!Number.isInteger(port) || port < 1 || port > 65535)
      throw new Error("请输入 1–65535 的连接端口");
    if (!isTauri())
      throw new Error("命令由 Rust 生成，请在安装后的应用中操作。");
    const commands = await invoke<string>("connection_commands", { ip, port });
    localStorage.setItem("connection", JSON.stringify({ ip, port }));
    el("commands").textContent = commands;
    notice("连接参数已保存到本机；命令请在 Mac 执行。");
  } catch (e) {
    el("commands").textContent = "请修正连接参数后重新生成。";
    notice(String(e));
  }
};
document.addEventListener("visibilitychange", () => {
  if (!document.hidden) void refresh();
});
void refresh();
