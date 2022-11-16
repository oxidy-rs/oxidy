pub(crate) async fn get_vec(obj: &[(String, String)], key: String) -> Option<String> {
    let mut value: Option<String> = None;

    for (k, v) in obj {
        if k.to_owned().to_lowercase() == key.to_lowercase() {
            value = Some(v.to_owned());
            break;
        }
    }

    value
}
