use tokio::io::{AsyncWriteExt, Error};
use tokio::net::tcp::OwnedWriteHalf;

pub(crate) async fn response_payload_empty(mut writer: OwnedWriteHalf) {
    /*
     * Write Payload
     */
    let stream_write: Result<(), Error> = writer.write_all("".as_bytes()).await;

    if stream_write.is_err() {
        println!(
            "[Error] Fail to Write Empty Stream:\n{}",
            stream_write.err().unwrap()
        );

        return;
    }
    /*
     * Flush Payload
     */
    let stream_flush: Result<(), Error> = writer.flush().await;

    if stream_flush.is_err() {
        println!(
            "[Error] Fail to Flush Empty Stream:\n{}",
            stream_flush.err().unwrap()
        );
    }
}
