use common::futures::SinkExt;
use common::futures_util::stream::StreamExt;
use common::tokio_util::codec::BytesCodec;
use common::tokio_util::udp::UdpFramed;
use processor::Processor;
use tokio::net::UdpSocket;

pub async fn start() -> Result<(), crate::Error> {
    let socket = UdpSocket::bind("0.0.0.0:5060").await?;
    common::log::debug!("starting udp server listening in port 5060");
    //let socket = UdpFramed::new(socket, BytesCodec::new());
    let (mut receiver, mut sender) = socket.split();
    //let (mut processor_tx, mut processor_rx) = mpsc::channel(100);

    let processor = Processor::new(); //this should be initialized elsewhere and injected probably
    let mut request: Vec<u8> = Vec::new();

    //I need to solve that through channels
    let foo = sender.clone();

    #[allow(irrefutable_let_patterns)]
    while let (size, peer) = receiver.recv_from(&mut request).await? {
        common::log::info!("new request from {:?}", peer);
        let response = processor.process_message(request.clone().into()).await;

        match response {
            Ok(response) => {
                sender.send_to(&response, &peer).await?;
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
