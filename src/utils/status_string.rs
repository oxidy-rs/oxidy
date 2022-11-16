/*
 * Get HTTP Response Status String from Status Code.
 * Added Most Used Codes.
 * Feel free to make PR for More Status Codes.
 * More Info: https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
 */
pub(crate) async fn status_string(code: usize) -> String {
    match code {
        426 => "Upgrade Required".to_owned(),
        414 => "URI Too Long".to_owned(),
        413 => "Payload Too Large".to_owned(),
        410 => "Gone".to_owned(),
        405 => "Method Not Allowed".to_owned(),
        404 => "Not Found".to_owned(),
        403 => "Forbidden".to_owned(),
        401 => "Unauthorized".to_owned(),
        400 => "Bad Request".to_owned(),

        308 => "Permanent Redirect".to_owned(),
        307 => "Temporary Redirect".to_owned(),
        306 => "Switch Proxy".to_owned(),
        305 => "Use Proxy".to_owned(),
        304 => "Not Modified".to_owned(),
        302 => "Found".to_owned(),
        301 => "Moved Permanently".to_owned(),

        202 => "Accepted".to_owned(),
        201 => "Created".to_owned(),
        200 => "OK".to_owned(),

        101 => "Switching Protocols".to_owned(),

        _ => String::new(),
    }
}
