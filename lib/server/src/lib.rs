//pub mod tcp;
mod error;
mod udp;

pub use error::Error;
pub use udp::UdpServer;

/*
fn pretty_print(request: Vec<u8>) -> String {
    let vec: Vec<u8> = request;
    format!(
        r#"
########################################################
{}
########################################################
"#,
        String::from_utf8_lossy(&vec)
    )
}
*/
