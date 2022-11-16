use tokio::io::{AsyncReadExt, Error};
use tokio::net::tcp::OwnedReadHalf;

pub(crate) async fn get_header(mut reader: OwnedReadHalf) -> String {
    let mut header: [u8; 512] = [0; 512];

    let buffer_reader: Result<usize, Error> = reader.read(&mut header).await;

    if buffer_reader.is_err() {
        println!(
            "[Error] Error in Stream Buffer Reader:\n{}",
            buffer_reader.err().unwrap()
        );

        return String::new();
    }

    String::from_utf8_lossy(&header[..]).to_string()
}
