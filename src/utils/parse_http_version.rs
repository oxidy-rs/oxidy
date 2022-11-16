pub(crate) async fn parse_http_version(header: String) -> f64 {
    let header: String = header
        .lines()
        .next()
        .expect("[Error] Fail to read header lines for HTTP Version")
        .to_owned();

    let header = header.split_whitespace().map(|x: &str| x.to_owned());

    let http_version: f64 = match header.into_iter().nth(2).unwrap_or_default().as_str() {
        "HTTP/3.0" => 3.0,
        "HTTP/2.0" => 2.0,
        "HTTP/1.1" => 1.1,
        "HTTP/1.0" => 1.0,
        _ => 0.9,
    };

    http_version
}
