pub(crate) async fn parse_path(header: String) -> (String, String, String) {
    let url: String = header
        .lines()
        .next()
        .expect("[Error] Fail to read header lines for Path")
        .to_owned()
        .split_whitespace()
        .map(|x: &str| x.to_owned())
        .into_iter()
        .nth(1)
        .unwrap_or_else(|| "/".to_owned());

    let url_split: Vec<String> = url.split('?').map(|x: &str| x.to_owned()).collect();

    let path: String = match url_split.get(0) {
        Some(x) => x.to_owned(),
        None => String::new(),
    };

    let query: String = match url_split.get(1) {
        Some(x) => x.to_owned(),
        None => String::new(),
    };

    (url, path, query)
}
