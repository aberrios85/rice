use std::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
use stunclient::StunClient;

pub fn get_srflx_address(srv_addr: String, host_socket: String) -> SocketAddr {
    let local_addr : SocketAddr = host_socket.parse().unwrap();
    let udp = UdpSocket::bind(local_addr).unwrap();
    let stun_addr = srv_addr.to_socket_addrs().unwrap().filter(|x|x.is_ipv4()).next().unwrap();
    let c = StunClient::new(stun_addr);

    c.query_external_address(&udp).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr};

    #[test]
    fn test_get_srflx_address() {
        assert_eq!(get_srflx_address("stun.l.google.com:19302".to_string(),"0.0.0.0:0".to_string()).ip(), Ipv4Addr::new(84, 92, 210, 205));
    }
}
