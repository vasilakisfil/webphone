use processor::transport::{Transport};
use processor::core::{Core};
use processor::transaction::{Transaction};

#[tokio::main]
async fn main() {
    common::pretty_env_logger::init_timed();
    common::Config::verify();

    /*
    let tcp = tokio::spawn(async move {
        server::tcp::start().await;
    });
    */

    let udp = tokio::spawn(async move {
        server::udp::start::<Transport<Core, Transaction>>()
            .await
            .expect("failed to start udp server");
    });

    tokio::try_join!(udp).expect("try join failed");
}
