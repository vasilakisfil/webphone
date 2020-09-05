use models::server::UdpTuple;
use tokio::sync::mpsc::Sender;
use models::{Request, Response, SipMessage};
use crate::{Processor, Error, helpers};
use common::bytes::Bytes;
use std::convert::TryInto;

//TODO: the udp_server should be something that wraps the channel half, and ideally,
//defined inside the server component, so transport should be injected in the server
//and probably the whole thing should be started from a common place
pub struct Transport {
    udp_server: Sender<UdpTuple>,
    processor: Processor,
}

impl Transport {
    pub fn new(udp_server: Sender<UdpTuple>) -> Self {
        Self {
            udp_server,
            processor: Processor::new(),
        }
    }

    pub async fn process_message(&self, tuple: UdpTuple) -> Result<Bytes, Error> {
        let sip_message: SipMessage = tuple.bytes.try_into()?;
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
        check_sent_by(&request);
        let response: Response = match find_transaction_for(&request) {
            Some(transaction) => transaction.handle_request(request)?,
            None => self.processor.handle_request(request)?,
        };

        Ok(response)
    }
}

//adds "received" param if necessary
fn check_sent_by(_request: &Request) {}

fn find_transaction_for(_request: &Request) -> Option<Transaction> {
    None
}

struct Transaction;
impl Transaction {
    pub fn handle_request(&self, request: Request) -> Result<Response, Error> {
        Ok(crate::presets::create_unauthorized_from(request)?)
    }
}
