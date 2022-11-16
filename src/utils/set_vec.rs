pub(crate) async fn set_vec(
    obj: &[(String, String)],
    key: String,
    value: String,
) -> Vec<(String, String)> {
    let mut obj: Vec<(String, String)> = obj
        .iter()
        .cloned()
        .filter(|(k, _)| k.to_owned().to_lowercase() != key.to_lowercase())
        .collect();

    obj.push((key, value));

    obj
}
