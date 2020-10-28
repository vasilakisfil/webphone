use common::bytes::Bytes;
use common::futures::stream::{SplitSink, SplitStream};
use common::futures_util::stream::StreamExt;
use common::tokio_util::codec::BytesCodec;
use common::tokio_util::udp::UdpFramed;
//use processor::Processor;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use models::server::ServerHandle;

use processor2::transport::TransportLayer;

type UdpSink = SplitSink<UdpFramed<BytesCodec>, (Bytes, SocketAddr)>;
type UdpStream = SplitStream<UdpFramed<BytesCodec>>;

#[allow(dead_code)]
pub struct UdpServer {
    transport_layer: TransportLayer,
    udp_sink: UdpSink,
    udp_stream: UdpStream,
}

// listens to server_stream and forwards to udp_sink
// listens to udp_stream and forwards to transport_sink
impl UdpServer {
    pub async fn new() -> Result<Self, crate::Error> {
        let (udp_sink, udp_stream) = create_socket().await?;

        let transport_layer = TransportLayer::new();

        Ok(Self {
            transport_layer,
            udp_sink,
            udp_stream,
        })
    }

    pub async fn run(&mut self) {
        while let Some(request) = self.udp_stream.next().await {
            match request {
                Ok((request, addr)) => {
                    self.transport_layer
                        .process(
                            self.udp_sink.clone().into(),
                            (request.freeze(), addr).into(),
                        )
                        .await
                }
                Err(e) => common::log::error!("{:?}", e),
            }
        }
    }
}

async fn create_socket() -> Result<(UdpSink, UdpStream), crate::Error> {
    let socket = UdpSocket::bind("0.0.0.0:5060").await?;
    common::log::debug!("starting udp server listening in port 5060");
    let socket = UdpFramed::new(socket, BytesCodec::new());
    Ok(socket.split())
}

