use crate::common::processor::EmptyTransport;

#[tokio::test]
async fn server_starts() {
    let udp = tokio::spawn(async move {
        server::udp::start::<EmptyTransport>()
            .await
            .expect("failed to start udp server");
    });
}
