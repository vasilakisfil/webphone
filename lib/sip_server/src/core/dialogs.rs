use super::DialogsProcessor;
pub use crate::{Error, SipManager};
use common::async_trait::async_trait;
//use models::transport::{RequestMsg, ResponseMsg};
use std::{
    any::Any,
    sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct Dialogs {
    sip_manager: Weak<SipManager>,
}

#[async_trait]
impl DialogsProcessor for Dialogs {
    fn new(sip_manager: Weak<SipManager>) -> Self {
        Self { sip_manager }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Dialogs {
    fn sip_manager(&self) -> Arc<SipManager> {
        self.sip_manager.upgrade().expect("sip manager is missing!")
    }
}
