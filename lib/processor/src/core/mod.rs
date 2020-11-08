mod processor;

use crate::{Error, transaction::TransactionLayer};
use common::async_trait::async_trait;
use common::futures_util::stream::StreamExt;
use models::{transaction::TransactionMsg, transport::TransportMsg, ChannelOf, SipMsg};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[allow(dead_code)]
pub struct Core {
    transport_to_self_sink: Sender<TransportMsg>,
    self_to_transport_sink: Sender<TransportMsg>,
    transaction_to_self_sink: Sender<TransactionMsg>,
    self_to_transaction_sink: Sender<TransactionMsg>,
    processor: processor::Processor,
}

#[async_trait]
pub trait CoreLayer: Send + Sync {
    async fn spawn<T: TransactionLayer>(
        self_to_transport_sink: Sender<TransportMsg>,
        transaction_to_transport_sink: Sender<TransportMsg>,
    ) -> Result<(Sender<TransportMsg>, Sender<TransportMsg>), crate::Error>;
}

// listens to transport_to_core_stream and acts, might send message to self_to_transport_sink
#[async_trait]
impl CoreLayer for Core {
    async fn spawn<T: TransactionLayer>(
        self_to_transport_sink: Sender<TransportMsg>,
        transaction_to_transport_sink: Sender<TransportMsg>,
    ) -> Result<(Sender<TransportMsg>, Sender<TransportMsg>), crate::Error> {
        let (transport_to_self_sink, transport_to_self_stream): ChannelOf<TransportMsg> =
            mpsc::channel(100);

        let (transaction_to_self_sink, transaction_to_self_stream): ChannelOf<TransactionMsg> =
            mpsc::channel(100);

        let (self_to_transaction_sink, transport_to_transaction_sink) = T::spawn(
            transaction_to_transport_sink,
            transaction_to_self_sink.clone(),
        )
        .await?;

        let transport_to_self_sink_cloned = transport_to_self_sink.clone();
        tokio::spawn(async move {
            let mut core = Self {
                transport_to_self_sink,
                self_to_transport_sink,
                transaction_to_self_sink,
                self_to_transaction_sink,
                processor: processor::Processor::new(),
            };
            core.run(transport_to_self_stream, transaction_to_self_stream)
                .await;
        });

        Ok((transport_to_self_sink_cloned, transport_to_transaction_sink))
    }
}

impl Core {
    async fn run(
        &mut self,
        mut transport_to_self_stream: Receiver<TransportMsg>,
        mut transaction_to_self_stream: Receiver<TransactionMsg>,
    ) {
        loop {
            tokio::select! {
                Some(transport_msg) = transport_to_self_stream.next() => {
                    match transport_msg {
                        TransportMsg::SipMsg(sip_msg) => self.handle_transport_msg(transport_msg).await,
                        TransportMsg::Error(error) => common::log::error!("{:?}", error)
                    }
                }
                Some(transaction_msg) = transaction_to_self_stream.next() => {
                    match transaction_msg {
                        TransactionMsg::SipMsg(sip_msg) => self.handle_transaction_msg(transaction_msg).await,
                        TransactionMsg::Error(error) => common::log::error!("{:?}", error)
                    }
                }
            }
        }
    }

    async fn handle_transport_msg(&mut self, sip_msg: SipMsg) -> Result<(), Error> {
        let SipMsg {
            sip_message,
            peer,
            transport,
        } = sip_msg;

        match self.processor.process_message(sip_message).await {
            Ok(sip_message) => {
                if self
                    .self_to_transport_sink
                    .send(SipMsg {
                        sip_message,
                        peer,
                        transport,
                    }.into())
                    .await
                    .is_err()
                {
                    common::log::error!("failed to send to transport layer");
                }
            }
            Err(error) => common::log::error!("failed to process transport msg in core: {}", error),
        }
    }

    async fn handle_transaction_msg(&mut self, sip_msg: SipMsg) {
        let SipMsg {
            sip_message,
            peer,
            transport,
        } = sip_msg;

        match self.processor.process_message(sip_message).await {
            Ok(sip_message) => {
                if self
                    .self_to_transport_sink
                    .send(SipMsg {
                        sip_message,
                        peer,
                        transport,
                    }.into())
                    .await
                    .is_err()
                {
                    common::log::error!("failed to send to transaction layer");
                }
            }
            Err(error) => {
                common::log::error!("failed to process transaction msg in core: {}", error)
            }
        }
    }
}
