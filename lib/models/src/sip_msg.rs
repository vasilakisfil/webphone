use crate::server::UdpTuple;
use crate::transport::TransportMsg;
use crate::transaction::TransactionMsg;
use rsip::common::Transport;
use std::convert::{TryFrom, TryInto};
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct SipMsg {
    pub sip_message: rsip::SipMessage,
    pub peer: SocketAddr,
    pub transport: Transport, //pub ttl: u32
}

impl Into<TransportMsg> for SipMsg {
    fn into(self) -> TransportMsg {
        TransportMsg::SipMsg(self)
    }
}

impl Into<TransactionMsg> for SipMsg {
    fn into(self) -> TransactionMsg {
        TransactionMsg::SipMsg(self)
    }
}

impl TryFrom<UdpTuple> for SipMsg {
    type Error = crate::Error;

    fn try_from(udp_tuple: UdpTuple) -> Result<Self, Self::Error> {
        Ok(Self {
            sip_message: udp_tuple.bytes.try_into()?,
            peer: udp_tuple.peer,
            transport: Transport::Udp,
        })
    }
}

impl Into<UdpTuple> for SipMsg {
    fn into(self) -> UdpTuple {
        UdpTuple {
            bytes: self.sip_message.into(),
            peer: self.peer,
        }
    }
}
