/*
 * Get HTTP Response Status String from Status Code.
 * Added Most Used Codes.
 * Feel free to make PR for More Status Codes.
 * More Info: https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
 */
pub(crate) fn http_status_string(code: String) -> String {
    match code.as_str() {
        "426" => "Upgrade Required".to_string(),
        "414" => "URI Too Long".to_string(),
        "413" => "Payload Too Large".to_string(),
        "410" => "Gone".to_string(),
        "405" => "Method Not Allowed".to_string(),
        "404" => "Not Found".to_string(),
        "403" => "Forbidden".to_string(),
        "401" => "Unauthorized".to_string(),
        "400" => "Bad Request".to_string(),

        "308" => "Permanent Redirect".to_string(),
        "307" => "Temporary Redirect".to_string(),
        "306" => "Switch Proxy".to_string(),
        "305" => "Use Proxy".to_string(),
        "304" => "Not Modified".to_string(),
        "302" => "Found".to_string(),
        "301" => "Moved Permanently".to_string(),

        "202" => "Accepted".to_string(),
        "201" => "Created".to_string(),
        "200" => "OK".to_string(),

        "101" => "Switching Protocols".to_string(),

        _ => "".to_string(),
    }
}
