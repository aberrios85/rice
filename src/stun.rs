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
