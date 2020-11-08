use crate::{transaction::TransactionMsg, transport::TransportMsg, Error, SipMsg};
use std::convert::TryInto;

pub enum CoreMsg {
    SipMsg(SipMsg),
    Error(Error),
}

impl TryInto<TransportMsg> for CoreMsg {
    type Error = crate::Error;

    fn try_into(self) -> Result<TransportMsg, crate::Error> {
        match self {
            Self::SipMsg(sip_msg) => Ok(TransportMsg::SipMsg(sip_msg)),
            Self::Error(error) => Err(error.into()),
        }
    }
}

impl From<TransportMsg> for CoreMsg {
    fn from(transport_msg: TransportMsg) -> Self {
        match transport_msg {
            TransportMsg::SipMsg(sip_msg) => Self::SipMsg(sip_msg),
            TransportMsg::Error(error) => Self::Error(error),
        }
    }
}

impl TryInto<TransactionMsg> for CoreMsg {
    type Error = crate::Error;

    fn try_into(self) -> Result<TransactionMsg, crate::Error> {
        match self {
            Self::SipMsg(sip_msg) => Ok(TransactionMsg::SipMsg(sip_msg)),
            Self::Error(error) => Err(error.into()),
        }
    }
}

impl From<TransactionMsg> for CoreMsg {
    fn from(transaction_msg: TransactionMsg) -> Self {
        match transaction_msg {
            TransactionMsg::SipMsg(sip_msg) => Self::SipMsg(sip_msg),
            TransactionMsg::Error(error) => Self::Error(error),
        }
    }
}
