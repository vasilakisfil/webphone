use common::async_trait::async_trait;
use models::server::UdpTuple;
use tokio::sync::mpsc::{Sender};

pub struct EmptyTransport {
    server_handle: Sender<UdpTuple>,
    incoming_messages: Vec<UdpTuple>
}

#[async_trait]
impl processor::transport::TransportLayer for EmptyTransport {
    fn new(server_handle: Sender<UdpTuple>) -> Self {
        Self {
            server_handle,
            incoming_messages: Vec::new()
        }
    }

    async fn process_incoming(&self, tuple: UdpTuple) {
        self.incoming_messages.push(tuple);
    }
}
