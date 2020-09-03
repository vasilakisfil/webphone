mod error;
pub mod helpers;
mod presets;
mod transactions;

use common::bytes::Bytes;
use common::tokio_util::codec::BytesCodec;
use common::tokio_util::udp::UdpFramed;
pub use error::Error;
use models::{Request, Response, SipMessage};
use std::convert::TryInto;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

//type UdpSink = common::futures::stream::SplitSink<UdpFramed<BytesCodec>, (Bytes, SocketAddr)>;

//should be generic soon
//generic is going to be injected during initialization (no initialization atm)
pub struct Processor {
    udp_sink: Sender<(Vec<u8>, SocketAddr)>
}

#[allow(clippy::new_without_default)]
impl Processor {
    pub fn new(udp_sink: Sender<(Vec<u8>, SocketAddr)>) -> Self {
        Self { udp_sink }
    }

    pub async fn process_message(&self, bytes: Bytes) -> Result<Bytes, Error> {
        let sip_message: SipMessage = bytes.try_into()?;
        helpers::trace_sip_message(sip_message.clone())?;

        let sip_message: SipMessage = match sip_message {
            SipMessage::Request(request) => self.handle_request(request),
            SipMessage::Response(_) => Err(Error::from("we don't support responses yet")),
        }?
        .into();

        helpers::trace_sip_message(sip_message.clone())?;
        Ok(sip_message.into())
    }

    fn handle_request(&self, request: Request) -> Result<Response, Error> {
        let response = self.handle_next_step_for(self.dialog_from(request.clone()), request)?;

        Ok(response)
    }

    fn handle_next_step_for(
        &self,
        dialog: Option<models::Dialog>,
        request: Request,
    ) -> Result<Response, Error> {
        use transactions::DialogExt;

        match dialog {
            Some(dialog) => Ok(dialog.transaction().next(request)?),
            None => {
                let auth_header = request.auth_header();
                match auth_header {
                    Ok(Some(header)) => {
                        if presets::is_authorized(header)? {
                            let dialog: models::Dialog =
                                store::Dialog::create_with_transaction(request.clone())?.into();
                            Ok(dialog.transaction().next(request)?)
                        } else {
                            Ok(presets::create_unauthorized_from(request)?)
                        }
                    }
                    Ok(None) => Ok(presets::create_unauthorized_from(request)?),
                    Err(err) => {
                        common::log::warn!("issue in auth header: {}", err);
                        Ok(presets::create_unauthorized_from(request)?)
                    }
                }
            }
        }
    }

    fn dialog_from(&self, request: Request) -> Option<models::Dialog> {
        store::Dialog::find_with_transaction(request.dialog_id()?)
            .ok()
            .map(|s| s.into())
    }
}
