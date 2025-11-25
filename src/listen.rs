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