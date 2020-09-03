use common::futures::SinkExt;
use common::bytes::Bytes;
use common::futures_util::stream::StreamExt;
use common::tokio_util::codec::BytesCodec;
use common::tokio_util::udp::UdpFramed;
use processor::Processor;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{self, channel, Receiver, Sender};

pub async fn start() -> Result<(), crate::Error> {
    let socket = UdpSocket::bind("0.0.0.0:5060").await?;
    common::log::debug!("starting udp server listening in port 5060");
    let socket = UdpFramed::new(socket, BytesCodec::new());
    let (mut sink, mut stream) = socket.split();
    let (mut udp_sender_tx, mut udp_sender_rx): (
        Sender<(Bytes, SocketAddr)>,
        Receiver<(Bytes, SocketAddr)>,
    ) = mpsc::channel(100);

    let processor = Processor::new(udp_sender_tx.clone()); //this should be initialized elsewhere and injected probably

    tokio::spawn(async move {
        loop {
            let (bytes, addr) = udp_sender_rx.recv().await.expect("udp_sender_rx receive");
            sink.send((bytes, addr)).await.expect("udp_send failed!");
        }
    });

    while let Some(request) = stream.next().await {
        match request {
            Ok((request, addr)) => {
                let response = processor.process_message(request.freeze()).await;
                common::log::info!("{}", addr);
                match response {
                    Ok(response) => udp_sender_tx
                        .send((response, addr))
                        .await
                        .expect("send to channel"),
                    Err(e) => common::log::error!("{}", e.to_string()),
                };
            }
            Err(e) => common::log::error!("{:?}", e),
        }
    }

    Ok(())
}

/*
pub async client() {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.local_addr
    let (mut sink, mut stream) = UdpFramed::new(socket, BytesCodec::new()).split();

    Ok(Self {
        ip_addr,
        stream,
        sink,
    })
}*/
