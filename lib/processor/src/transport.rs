use crate::{helpers, Error, Processor};
use models::server::UdpTuple;
use models::{Request, Response, SipMessage};
use std::convert::TryInto;
use tokio::sync::mpsc::Sender;

//TODO: the udp_server should be something that wraps the channel half, and ideally,
//defined inside the server component, so transport should be injected in the server
//and probably the whole thing should be started from a common place
pub struct Transport {
    udp_server: Sender<UdpTuple>,
    core: Processor,
}

impl Transport {
    pub fn new(udp_server: Sender<UdpTuple>) -> Self {
        Self {
            udp_server,
            core: Processor::new(),
        }
    }

    pub async fn process_incoming(&self, tuple: UdpTuple) {
        match self.process_incoming_message(tuple).await {
            Ok(_) => (),
            Err(error) => {
                common::log::error!("error when processing incoming message: {:?}", error)
            }
        }
    }

    async fn process_incoming_message(&self, tuple: UdpTuple) -> Result<(), Error> {
        let sip_message: SipMessage = tuple.bytes.try_into()?;
        helpers::trace_sip_message(sip_message.clone())?;

        Ok(match sip_message {
            SipMessage::Request(request) => self.handle_incoming_request(request)?,
            SipMessage::Response(response) => self.handle_incoming_response(response)?,
        })
    }

    fn handle_incoming_request(&self, mut request: Request) -> Result<(), Error> {
        ensure_received_param(&mut request);

        Ok(match find_transaction_for_request(&request) {
            Some(transaction) => transaction.handle_request(request)?,
            None => self.core.handle_request(request)?,
        })
    }

    fn handle_incoming_response(&self, response: Response) -> Result<(), Error> {
        check_sent_by(&response)?;
        Ok(match find_transaction_for_response(&response) {
            Some(transaction) => transaction.handle_response(response)?,
            None => self.core.handle_response(response)?,
        })
    }
}

//adds "received" param if necessary
fn ensure_received_param(_request: &mut Request) {}

//checks if sent-by is correctly set, or the response is mis-routed
fn check_sent_by(_response: &Response) -> Result<(), Error> {Ok(())}


//TODO: these 2 functions should be one using a simple trait
fn find_transaction_for_request(_request: &Request) -> Option<Transaction> {
    None
}
fn find_transaction_for_response(_response: &Response) -> Option<Transaction> {
    None
}

struct Transaction;
impl Transaction {
    pub fn handle_request(&self, _request: Request) -> Result<(), Error> {
        //Ok(crate::presets::create_unauthorized_from(request)?)
        Ok(())
    }
    pub fn handle_response(&self, _response: Response) -> Result<(), Error> {
        //Ok(crate::presets::create_unauthorized_from(request)?)
        Ok(())
    }
}
