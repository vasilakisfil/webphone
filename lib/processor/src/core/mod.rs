use crate::Error;
use models::{transport::TransportTuple, Request, Response};
use tokio::sync::mpsc::{Sender};

#[derive(Clone)]
pub struct Core {
    #[allow(dead_code)]
    transport_handle: Sender<TransportTuple>,
}

pub trait CoreLayer {
    fn new(transport_handle: Sender<TransportTuple>) -> Self;
    fn handle_request(&self, request: Request) -> Result<(), Error>;
    fn handle_response(&self, _response: Response) -> Result<(), Error>;
}

impl CoreLayer for Core {
    fn new(transport_handle: Sender<TransportTuple>) -> Self {
        Self { transport_handle }
    }

    fn handle_request(&self, request: Request) -> Result<(), Error> {
        let _response = self.handle_next_step_for(self.dialog_from(request.clone()), request)?;

        Ok(())
    }

    fn handle_response(&self, _response: Response) -> Result<(), Error> {
        Ok(())
    }
}

impl Core {
    fn handle_next_step_for(
        &self,
        dialog: Option<models::Dialog>,
        request: Request,
    ) -> Result<Response, Error> {
        use crate::transactions::DialogExt;

        match dialog {
            Some(dialog) => Ok(dialog.transaction().next(request)?),
            None => {
                let auth_header = request.auth_header();
                match auth_header {
                    Ok(Some(header)) => {
                        if crate::presets::is_authorized(header)? {
                            let dialog: models::Dialog =
                                store::Dialog::create_with_transaction(request.clone())?.into();
                            Ok(dialog.transaction().next(request)?)
                        } else {
                            Ok(crate::presets::create_unauthorized_from(request)?)
                        }
                    }
                    Ok(None) => Ok(crate::presets::create_unauthorized_from(request)?),
                    Err(err) => {
                        common::log::warn!("issue in auth header: {}", err);
                        Ok(crate::presets::create_unauthorized_from(request)?)
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
