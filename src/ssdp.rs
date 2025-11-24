use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};


//------------- 构造SSDP消息 -------------



pub fn build_ssdp_message(local_ip: &str, status: &str, groupid: &str) -> String {
    
    let usn_ip = local_ip.replace(".", "");
    let os = "linux";
    let os_version = "5.4.0";
    let upnp = "UPnP";
    let upnp_version = "1.0";
    let server_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    format!(
        "NOTIFY * HTTP/1.1\r\n\
        HOST: 239.255.255.250:1900\r\n\
        CACHE-CONTROL: max-age=60\r\n\
        LOCATION: http://{}\r\n\
        NT: urn:{}:service:localdns:{}\r\n\
        NTS: ssdp:{}\r\n\
        SERVER: {}/{} {}/{} {}/{}} \r\n
        GROUPID: {}\r\n",
        local_ip, server_name, version, status, server_name, os, os_version, upnp, upnp_version, server_name, version
    )
}


//------------- 主功能函数 -------------
pub fn ssdp_broadcast(message: &str) {
    let multicast_addr: SocketAddrV4 = "239.255.255.250:1900".parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").expect("无法绑定UDP socket");
    socket.join_multicast_v4(&Ipv4Addr::new(239,255,255,250), &Ipv4Addr::new(0,0,0,0)).unwrap();
    socket.set_multicast_ttl_v4(4).unwrap();

    if let Err(e) = socket.send_to(message.as_bytes(), multicast_addr) {
        eprintln!("SSDP 广播发送失败: {}", e);
    } else {
        println!("SSDP 广播已发送");
    }
}

pub fn ssdp_listen(local_iface: &str) {
    let bind_addr: SocketAddrV4 = "0.0.0.0:1900".parse().unwrap();
    let socket = match UdpSocket::bind(bind_addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("监听 SSDP 绑定失败 {}: {}", bind_addr, e);
            return;
        }
    };

    let iface: Ipv4Addr = local_iface.parse().unwrap_or(Ipv4Addr::UNSPECIFIED);
    if let Err(e) = socket.join_multicast_v4(&Ipv4Addr::new(239,255,255,250), &iface) {
        eprintln!("加入 SSDP 组播失败: {}", e);
        // 仍然继续尝试接收（某些平台不需要显式 join）
    }

    let mut buf = [0u8; 2048];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((n, src)) => {
                if let Ok(s) = String::from_utf8(buf[..n].to_vec()) {
                    println!("[SSDP recv from {}]\n{}", src, s);
                } else {
                    println!("[SSDP recv binary {} bytes from {}]", n, src);
                }
            }
            Err(e) => {
                eprintln!("接收 SSDP 数据错误: {}", e);
                // 可根据需要决定是否 break
            }
        }
    }
}