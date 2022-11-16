pub(crate) async fn parse_method(header: String) -> String {
    header
        .lines()
        .next()
        .expect("[Error] Fail to read header lines for Method")
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_owned()
}
