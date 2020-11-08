use crate::{server::UdpTuple, Error, SipMsg};
use rsip::common::Transport;
use std::convert::TryInto;

pub enum TransportMsg {
    SipMsg(SipMsg),
    Error(Error),
}

impl TryInto<UdpTuple> for TransportMsg {
    type Error = crate::Error;

    fn try_into(self) -> Result<UdpTuple, Error> {
        match self {
            Self::SipMsg(sip_msg) => Ok(sip_msg.into()),
            Self::Error(error) => Err(Error::from(format!(
                "transport msg to udp tuple failed: {:?}",
                error,
            ))),
        }
    }
}

impl From<UdpTuple> for TransportMsg {
    fn from(udp_tuple: UdpTuple) -> TransportMsg {
        let sip_message: Result<rsip::SipMessage, String> = udp_tuple.bytes.try_into();
        match sip_message {
            Ok(sip_message) => Self::SipMsg(SipMsg {
                sip_message: sip_message,
                peer: udp_tuple.peer,
                transport: Transport::Udp,
            }),
            Err(error) => Self::Error(error.into()),
        }
    }
}
