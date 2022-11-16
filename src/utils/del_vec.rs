pub(crate) async fn del_vec(obj: &[(String, String)], key: String) -> Vec<(String, String)> {
    obj.iter()
        .cloned()
        .filter(|(k, _)| k.to_owned().to_lowercase() != key.to_lowercase())
        .collect()
}
