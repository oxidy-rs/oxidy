use std::collections::HashMap;

pub(crate) async fn parse(str: String) -> HashMap<String, String> {
    let mut h: Vec<&str> = str.lines().collect();
    let h_firstln: Vec<String> = h[0].split_whitespace().map(|s| s.to_string()).collect();
    h.remove(0);
    /*
     * Split path with Path & Query
     */
    let mut path_split: Vec<String> = vec!["/".to_string()];

    if h_firstln.get(1).is_some() {
        path_split = h_firstln[1].split("?").map(|s| s.to_string()).collect();
    }
    /*
     * Separate Path & Query
     */
    let mut path: String = "".to_string();
    let mut query: String = "".to_string();

    if path_split.get(0).is_some() {
        path = path_split[0].clone();
    }
    if path_split.get(1).is_some() {
        query = path_split[1].clone();
    }
    /*
     * Http Version
     */
    let mut http_version: String = "".to_string();
    if h_firstln.get(2).is_some() {
        http_version = match h_firstln[2].as_str() {
            "HTTP/3.0" => "3.0".to_string(),
            "HTTP/2.0" => "2.0".to_string(),
            "HTTP/1.1" => "1.1".to_string(),
            "HTTP/1.0" => "1.0".to_string(),
            _ => "0.9".to_string(),
        };
    }
    /*
     * Headers
     */
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert("method".to_string(), h_firstln[0].clone());
    header.insert("path".to_string(), path);
    header.insert("query".to_string(), query);
    header.insert("http-version".to_string(), http_version);

    h.iter().for_each(|ln| {
        let mut ln_split: Vec<String> = ln.split_whitespace().map(|s| s.to_string()).collect();
        if ln_split.len() < 1 {
            return;
        }
        /*
         * Key
         */
        let k: String = ln_split[0]
            .clone()
            .trim()
            .to_string()
            .to_lowercase()
            .replace(":", "");
        /*
         * Filter Key
         */
        let k: Vec<String> = k.split_whitespace().map(|s| s.to_string()).collect();
        let k: String = k.join("");
        if k.len() < 1 {
            return;
        }
        /*
         * Value
         */
        let mut v: String = "".to_string();

        if ln_split.get(1).is_some() {
            v = ln_split[1].clone().to_string();
            if ln_split.len() > 2 {
                ln_split.remove(0);
                v = ln_split.join(" ").to_string();
            }
        }

        header.insert(k, v);
    });
    /*
     * Return Parsed Data
     */
    header
}
