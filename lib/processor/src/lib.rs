mod error;
pub mod helpers;
mod presets;
mod transactions;
pub mod transport;

pub use error::Error;
use models::{transport::TransportTuple, Request, Response};
use tokio::sync::mpsc::{Sender};

//type UdpSink = common::futures::stream::SplitSink<UdpFramed<BytesCodec>, (Bytes, SocketAddr)>;

//should be generic soon
//generic is going to be injected during initialization (no initialization atm)
pub struct Processor {
    transport_handle: Sender<TransportTuple>,
}

#[allow(clippy::new_without_default)]
impl Processor {
    pub fn new(transport_handle: Sender<TransportTuple>) -> Self {
        Self { transport_handle }
    }

    fn handle_request(&self, request: Request) -> Result<(), Error> {
        let _response = self.handle_next_step_for(self.dialog_from(request.clone()), request)?;

        Ok(())
    }

    fn handle_response(&self, _response: Response) -> Result<(), Error> {
        Ok(())
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
