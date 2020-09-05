use std::net::SocketAddr;

//TODO: we probably need better naming here
#[derive(Debug)]
pub struct TransportTuple {
    pub sip_message: crate::SipMessage,
    pub peer: SocketAddr,
    pub transport: crate::TransportType
    //pub ttl: u32
}

impl Into<crate::server::UdpTuple> for TransportTuple {
    fn into(self) -> crate::server::UdpTuple {
        crate::server::UdpTuple {
            bytes: self.sip_message.into(),
            peer: self.peer,
        }
    }
}
