use common::futures::SinkExt;
use common::futures_util::stream::StreamExt;
use common::tokio_util::codec::BytesCodec;
use common::tokio_util::udp::UdpFramed;
use processor::Processor;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{self, channel, Sender, Receiver};

pub async fn start() -> Result<(), crate::Error> {
    let socket = UdpSocket::bind("0.0.0.0:5060").await?;
    common::log::debug!("starting udp server listening in port 5060");
    //let socket = UdpFramed::new(socket, BytesCodec::new());
    let (mut receiver, mut sender) = socket.split();
    let (mut udp_sender_tx, mut udp_sender_rx): (
        Sender<(Vec<u8>, SocketAddr)>,
        Receiver<(Vec<u8>, SocketAddr)>,
    ) = mpsc::channel(100);

    let processor = Processor::new(udp_sender_tx.clone()); //this should be initialized elsewhere and injected probably
    let mut request: Vec<u8> = Vec::new();

    tokio::spawn(async move {
        let (bytes, addr) = udp_sender_rx.recv().await.expect("udp_sender_rx receive");
        sender
            .send_to(&bytes, &addr)
            .await
            .expect("udp_send failed!");
    });

    #[allow(irrefutable_let_patterns)]
    while let (_, peer) = receiver.recv_from(&mut request).await? {
        common::log::info!("new request from {:?}", peer);
        let response = processor.process_message(request.clone().into()).await;

        match response {
            Ok(response) => {
                udp_sender_tx.send((response.to_vec(), peer)).await.expect("send to channel");
            }
            Err(e) => common::log::error!("{}", e.to_string()),
        };
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
