use crate::{transport::TransportMsg, Error, SipMsg};
use std::convert::TryInto;

#[derive(Debug)]
pub enum TransactionMsg {
    SipMsg(SipMsg),
    Error(Error),
}

impl TryInto<TransportMsg> for TransactionMsg {
    type Error = crate::Error;

    fn try_into(self) -> Result<TransportMsg, crate::Error> {
        match self {
            Self::SipMsg(sip_msg) => Ok(TransportMsg::SipMsg(sip_msg)),
            Self::Error(error) => Err(error),
        }
    }
}

impl From<TransportMsg> for TransactionMsg {
    fn from(transport_msg: TransportMsg) -> Self {
        match transport_msg {
            TransportMsg::SipMsg(sip_msg) => Self::SipMsg(sip_msg),
            TransportMsg::Error(error) => Self::Error(error),
        }
    }
}
