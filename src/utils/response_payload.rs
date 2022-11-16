use crate::structs::context::Context;
use crate::utils::status_string::status_string;
use tokio::io::{AsyncWriteExt, Error};
use tokio::net::tcp::OwnedWriteHalf;

pub(crate) async fn response_payload(
    mut writer: OwnedWriteHalf,
    context: Context,
    http_version: f64,
) {
    /*
     * Prepare Response Headers
     */
    let mut response_header: String = String::new();

    context.response.header.iter().for_each(|(k, v)| {
        response_header.push_str(&format!("{}: {}\r\n", k, v));
    });
    /*
     * Prepare Response Payload
     */
    let status_str: String = status_string(context.response.status).await;

    let response: String = format!(
        "HTTP/{0} {1} {2}\r\n{3}Content-Type: {4}\r\nContent-Length: {5}\r\n\r\n{6}",
        http_version,
        context.response.status,
        status_str,
        response_header,
        context.response.content_type,
        context.response.body.len(),
        context.response.body,
    );
    /*
     * Write Payload
     */
    let stream_write: Result<(), Error> = writer.write_all(response.as_bytes()).await;

    if stream_write.is_err() {
        println!(
            "[Error] Fail to Write Stream:\n{}",
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
            "[Error] Fail to Flush Stream:\n{}",
            stream_flush.err().unwrap()
        );
    }
}
