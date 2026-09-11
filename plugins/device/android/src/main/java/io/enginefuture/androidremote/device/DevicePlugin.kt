package io.enginefuture.androidremote.device
import android.app.Activity
import android.content.Intent
import android.os.Build
import android.provider.Settings
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import org.json.JSONArray
import java.net.NetworkInterface
@TauriPlugin
class DevicePlugin(private val activity:Activity):Plugin(activity){
 @Command fun status(invoke:Invoke){try{
  val ips=JSONArray()
  NetworkInterface.getNetworkInterfaces()?.toList()?.forEach { nic -> if(nic.isUp) nic.inetAddresses.toList().forEach { addr ->
   val b=addr.address
   if(b.size==4 && (b[0].toInt() and 255)==100 && (b[1].toInt() and 255) in 64..127) ips.put(addr.hostAddress)
  }}
  val result=JSObject()
  result.put("model","${Build.MANUFACTURER} ${Build.MODEL}")
  result.put("android",Build.VERSION.RELEASE)
  result.put("sdk",Build.VERSION.SDK_INT)
  result.put("candidateIps",ips)
  result.put("developerEnabled",Settings.Global.getInt(activity.contentResolver,Settings.Global.DEVELOPMENT_SETTINGS_ENABLED,0)==1)
  invoke.resolve(result)
 }catch(e:Exception){invoke.reject(e.message ?: "无法读取设备状态")}}
 @Command fun openSettings(invoke:Invoke){activity.runOnUiThread{try{activity.startActivity(Intent(Settings.ACTION_APPLICATION_DEVELOPMENT_SETTINGS));invoke.resolve()}catch(e:Exception){invoke.reject("请在系统设置中手动打开开发者选项")}}}
 @Command fun openTailscale(invoke:Invoke){activity.runOnUiThread{try{
  val intent=activity.packageManager.getLaunchIntentForPackage("com.tailscale.ipn")
  if(intent==null)invoke.reject("未找到 Tailscale，请手动打开") else {activity.startActivity(intent);invoke.resolve()}
 }catch(e:Exception){invoke.reject(e.message ?: "无法打开 Tailscale")}}}
}
