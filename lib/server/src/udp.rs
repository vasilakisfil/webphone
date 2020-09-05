use common::futures::SinkExt;
use common::futures_util::stream::StreamExt;
use common::tokio_util::codec::BytesCodec;
use common::tokio_util::udp::UdpFramed;
use processor::transport::Transport;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{self, Receiver, Sender};
use models::server::UdpTuple;

//TODO: remove UdpFramed from here and use raw datagrams
pub async fn start() -> Result<(), crate::Error> {
    let socket = UdpSocket::bind("0.0.0.0:5060").await?;
    common::log::debug!("starting udp server listening in port 5060");
    let socket = UdpFramed::new(socket, BytesCodec::new());
    let (mut udp_sink, mut udp_stream) = socket.split();
    let (server_sink, mut server_stream): (Sender<UdpTuple>, Receiver<UdpTuple>) =
        mpsc::channel(100);

    let transport = Transport::new(server_sink); //this should be initialized elsewhere and injected probably

    tokio::spawn(async move {
        loop {
            let udp_tuple = server_stream
                .recv()
                .await
                .expect("udp server stream receive failed!");
            udp_sink
                .send(udp_tuple.into())
                .await
                .expect("udp send failed!");
        }
    });

    while let Some(request) = udp_stream.next().await {
        match request {
            Ok((request, addr)) => {
                common::log::debug!("new message from {}", addr);
                transport.process_incoming((request.freeze(), addr).into()).await;
            }
            Err(e) => common::log::error!("failed to receive message from udp stream: {:?}", e),
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
