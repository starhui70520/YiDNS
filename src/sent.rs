use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};
mod system;

//------------- 构造SSDP消息 -------------
pub fn build_ssdp_message(local_ip: &str, status: &str, groupid: &str) -> String {
    
    let usn_ip = local_ip.replace(".", "");
    let os = system::os_type();
    let os_version = system::os_version();
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
        return;
    } 

    println!("SSDP 广播已发送");
}