use crate::{core::CoreLayer, helpers, transaction::TransactionLayer, Error};
use common::async_trait::async_trait;
use models::server::UdpTuple;
use models::{transport::TransportTuple, Request, Response, SipMessage};
use std::convert::TryInto;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub trait TransportLayer {
    type Handler: TransportLayerSink;
    fn new(server_handle: Sender<UdpTuple>) -> Self::Handler;
}

#[async_trait]
pub trait TransportLayerSink {
    async fn send(&mut self, tuple: UdpTuple);
}

pub struct Transport<C, T>
where
    C: CoreLayer + Send + Sync,
    T: TransactionLayer + Send + Sync,
{
    from_server_stream: Receiver<UdpTuple>,
    from_core_stream: Receiver<TransportTuple>,
    core: C,
    ts: T,
}

pub struct TransportSink {
    pub from_server_sink: Sender<UdpTuple>,
}

#[async_trait]
impl TransportLayerSink for TransportSink {
    async fn send(&mut self, msg: UdpTuple) {
        self.from_server_sink
            .send(msg)
            .await
            .expect("sending into transport layer from server failed")
    }
}

impl<C, T> TransportLayer for Transport<C, T>
where
    C: CoreLayer + Send + Sync + Clone,
    T: TransactionLayer + Send + Sync,
{
    type Handler = TransportSink;

    fn new(to_server_sink: Sender<UdpTuple>) -> TransportSink {
        let (_from_server_sink, from_server_stream): (Sender<UdpTuple>, Receiver<UdpTuple>) =
            mpsc::channel(100);

        let (_from_core_sink, from_core_stream): (
            Sender<TransportTuple>,
            Receiver<TransportTuple>,
        ) = mpsc::channel(100);

        let core = C::new(_from_core_sink.clone());
        let ts = T::new(core, _from_core_sink);

        let transport = Self {
            from_server_stream,
            from_core_stream,
            core,
            ts,
        };

        tokio::spawn(transport.run());

        return TransportSink { _from_server_sink };
    }
}

impl<C, T> Transport<C, T>
where
    C: CoreLayer + Send + Sync,
    T: TransactionLayer + Send + Sync,
{
    fn run(&self) {
        loop {
            let message = self.from_server_stream
                .recv()
                .await
                .expect("udp server stream receive failed!");

            println!("received message! {:?}", message);
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

        Ok(match self.ts.find_transaction_for_request(&request) {
            Some(transaction) => transaction.handle_request(request)?,
            None => self.core.handle_request(request)?,
        })
    }

    fn handle_incoming_response(&self, response: Response) -> Result<(), Error> {
        check_sent_by(&response)?;
        Ok(match self.ts.find_transaction_for_response(&response) {
            Some(transaction) => transaction.handle_response(response)?,
            None => self.core.handle_response(response)?,
        })
    }

    /*
        fn spawn(&self, mut incoming_stream: Receiver<TransportTuple>) {
            let mut server_handle = self.server_handle.clone();

            tokio::spawn(async move {
                loop {
                    let transport_tuple = incoming_stream
                        .recv()
                        .await
                        .expect("transport stream receive failed!");

                    process_outgoing_message(&transport_tuple);

                    server_handle
                        .send(transport_tuple.into())
                        .await
                        .expect("udp send failed!");
                }
            });
        }
    */
}

//adds "received" param if necessary
fn ensure_received_param(_request: &mut Request) {}

//checks if sent-by is correctly set, or the response is mis-routed
fn check_sent_by(_response: &Response) -> Result<(), Error> {
    Ok(())
}

fn process_outgoing_message(_tuple: &TransportTuple) {}
