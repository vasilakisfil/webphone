mod states;

pub use states::{Confirmed, Deleted, Early, Errored, Unconfirmed};

use crate::Error;
use crate::SipManager;
use models::transport::RequestMsg;
use tokio::time::Instant;
use std::sync::Arc;
use rsip::common::Uri;

#[derive(Debug)]
pub struct DgStateMachine {
    pub id: String,
    pub local_sn: u16,
    pub local_uri: Uri,
    pub remote_sn: u16,
    pub remote_uri: Uri,
    pub msg: RequestMsg,
    pub state: DgState,
    pub created_at: Instant,
    pub sip_manager: Arc<SipManager>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum DgState {
    Unconfirmed(Unconfirmed),
    Early(Early),
    Confirmed(Confirmed),
    Deleted(Deleted),
    Errored(Errored),
}

impl std::fmt::Display for DgState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unconfirmed(_) => write!(f, "DgState::Unconfirmed"),
            Self::Early(_) => write!(f, "DgState::Early"),
            Self::Confirmed(_) => write!(f, "DgState::Confirmed"),
            Self::Deleted(_) => write!(f, "DgState::Deleted"),
            Self::Errored(_) => write!(f, "DgState::Errored"),
        }
    }
}

impl DgStateMachine {
    pub fn new(sip_manager: Arc<SipManager>, msg: RequestMsg) -> Result<Self, Error> {
        Ok(Self {
            id: "something".into(),
            local_sn: 0,
            local_uri: Default::default(),
            remote_sn: 0,
            remote_uri: Default::default(),
            msg,
            state: DgState::Unconfirmed(Default::default()),
            created_at: Instant::now(),
            sip_manager,
        })
    }
}
