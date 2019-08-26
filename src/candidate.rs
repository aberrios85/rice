use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
pub enum Protocol {
    Udp,
    Tcp,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
pub enum CandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Candidate {
    addr: Ipv4Addr,
    pub base_addr: Ipv4Addr,
    port: u16,
    protocol: Protocol,
    r#type: CandidateType,
    foundation: u32,
    pub component: u32,
}

impl Candidate {
    pub fn priority(&self) -> u32 {
        let type_pref = match self.r#type {
            CandidateType::Host => 126,
            CandidateType::Prflx => 110,
            CandidateType::Srflx => 100,
            CandidateType::Relay => 0,
        };

        //When there is only a single IP address, this value SHOULD be set to 65535
        //If a host is multihomed because it is dual stack, the local preference SHOULD be set equal to the precedence value for IP addresses described in RFC 3484
        let _local_pref = 65535;

        (2u32.pow(24)*type_pref) +
        (2u32.pow(8)*_local_pref) +
        (2u32.pow(0)*(256 - self.component))
    }

    pub fn transport_addr(&self) -> SocketAddrV4 {
        SocketAddrV4::new(self.addr, self.port)
    }

    pub fn new() -> Candidate {
        Candidate{
            addr: Ipv4Addr::new(10, 0, 1, 40),
            base_addr: Ipv4Addr::new(10, 0, 1, 40),
            port: 20000,
            protocol: Protocol::Udp,
            r#type: CandidateType::Host,
            foundation: 1,
            component: 1, // 1 for RTP and 2 for RTCP unless multiplexed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcs_priority() {
        let mut candidate = Candidate{
            addr: Ipv4Addr::new(10, 0, 1, 40),
            base_addr: Ipv4Addr::new(10, 0, 1, 40),
            port: 20000,
            protocol: Protocol::Udp,
            r#type: CandidateType::Host,
            foundation: 1,
            component: 1, // 1 for RTP and 2 for RTCP unless multiplexed
        };
        assert_eq!(candidate.priority(), 2130706431);
        candidate.component = 2;
        assert_eq!(candidate.priority(), 2130706430);
        candidate.component = 1;
        candidate.r#type = CandidateType::Srflx;
        assert_eq!(candidate.priority(), 1694498815);
    }
}
