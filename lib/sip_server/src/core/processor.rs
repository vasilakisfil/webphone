pub use super::{Capabilities, CoreProcessor, Registrar, ReqProcessor, DialogsProcessor};
pub use crate::{presets, Error, SipManager};
use common::async_trait::async_trait;
use models::transport::ResponseMsg;
use models::transport::{RequestMsg, TransportMsg};
use rsip::SipMessage;
use std::{
    any::Any,
    sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct Processor<R: ReqProcessor, C: ReqProcessor, D: DialogsProcessor> {
    sip_manager: Weak<SipManager>,
    registrar: R,
    capabilities: C,
    dialogs: D,
}

#[async_trait]
impl<R: ReqProcessor, C: ReqProcessor, D: DialogsProcessor> CoreProcessor for Processor<R, C, D> {
    fn new(sip_manager: Weak<SipManager>) -> Self {
        Self {
            registrar: R::new(sip_manager.clone()),
            capabilities: C::new(sip_manager.clone()),
            dialogs: D::new(sip_manager.clone()),
            sip_manager,
        }
    }

    async fn process_incoming_message(&self, msg: TransportMsg) -> Result<(), Error> {
        let sip_message = msg.sip_message;

        match sip_message {
            SipMessage::Request(request) => {
                self.handle_request(RequestMsg::new(request, msg.peer, msg.transport))
                    .await
            }
            SipMessage::Response(_) => Err(Error::from("we don't support responses yet")),
        }?;

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<R: ReqProcessor, C: ReqProcessor, D: DialogsProcessor> Processor<R, C, D> {
    async fn handle_request(&self, msg: RequestMsg) -> Result<(), Error> {
        use rsip::common::Method;

        match msg.sip_request.method {
            Method::Register => {
                self.registrar
                    .process_incoming_request(self.with_auth(msg).await?)
                    .await?
            }
            Method::Options => self.capabilities.process_incoming_request(msg).await?,
            _ => {
                self.sip_manager()
                    .transport
                    .send(
                        ResponseMsg::new(
                            presets::create_405_from(msg.sip_request)?,
                            msg.peer,
                            msg.transport,
                        )
                        .into(),
                    )
                    .await?
            }
        };

        Ok(())
    }

    fn sip_manager(&self) -> Arc<SipManager> {
        self.sip_manager.upgrade().expect("sip manager is missing!")
    }

    async fn with_auth(&self, msg: RequestMsg) -> Result<RequestMsg, Error> {
        match msg.sip_request.authorization_header() {
            Some(_) => Ok(msg),
            None => {
                self.sip_manager()
                    .transport
                    .send(
                        ResponseMsg::from((
                            presets::create_unauthorized_from(msg.sip_request)?,
                            msg.peer,
                            msg.transport,
                        ))
                        .into(),
                    )
                    .await?;
                Err(Error::from("missing auth header"))
            }
        }
    }
}
