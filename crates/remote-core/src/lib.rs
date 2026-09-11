use std::net::{Ipv4Addr, SocketAddrV4};
pub struct Connection {
    pub ip: String,
    pub port: u16,
}
impl Connection {
    pub fn endpoint(&self) -> Result<String, String> {
        let ip: Ipv4Addr = self.ip.parse().map_err(|_| "请输入 Tailscale IPv4 地址")?;
        let b = ip.octets();
        if b[0] != 100 || !(64..=127).contains(&b[1]) {
            return Err("地址必须属于 Tailscale 100.64.0.0/10 网段".into());
        }
        if self.port == 0 {
            return Err("端口必须为 1–65535".into());
        }
        Ok(SocketAddrV4::new(ip, self.port).to_string())
    }
    pub fn commands(&self) -> Result<String, String> {
        let ep = self.endpoint()?;
        Ok(format!(
            "adb connect {ep}\nadb -s {ep} get-state\nscrcpy -s {ep}"
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_tailnet_boundaries_and_ports() {
        for ip in ["100.64.0.1", "100.127.255.254"] {
            assert!(Connection {
                ip: ip.into(),
                port: 1
            }
            .endpoint()
            .is_ok());
        }
        for ip in [
            "100.63.255.255",
            "100.128.0.1",
            "127.0.0.1",
            "100.64.0.1;id",
            "--help",
        ] {
            assert!(Connection {
                ip: ip.into(),
                port: 5555
            }
            .endpoint()
            .is_err());
        }
        assert!(Connection {
            ip: "100.64.0.1".into(),
            port: 0
        }
        .endpoint()
        .is_err());
    }
}
