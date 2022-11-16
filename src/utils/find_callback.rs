/*
 * Get Callback for Route in Parallel
 */
pub(crate) struct IsFind {
    pub(crate) find: bool,
    pub(crate) param: Vec<(String, String)>,
}

pub(crate) async fn find_callback(path: String, callback_path: String) -> IsFind {
    /*
     * Path Split
     */
    let path_split: Vec<String> = path
        .split('/')
        .filter(|x: &&str| !x.is_empty())
        .map(|x: &str| x.to_owned())
        .collect();
    /*
     * Callback Path Split
     * Dynamic Match
     */
    let callback_path_split: Vec<String> = callback_path
        .split('/')
        .filter(|x: &&str| !x.is_empty())
        .map(|x: &str| x.to_owned())
        .collect();
    /*
     * Check Split Length
     */
    if callback_path_split.len() != path_split.len() {
        return IsFind {
            find: false,
            param: Vec::new(),
        };
    }

    let mut prepare_path: String = String::new();
    let mut param: Vec<(String, String)> = Vec::new();

    callback_path_split
        .into_iter()
        .enumerate()
        .for_each(|(i, callback_path_elm)| {
            let callback_path_char: char = callback_path_elm
                .chars()
                .next()
                .expect("[Error] Fail to convert from path string to path char");
            /*
             * Static
             */
            if callback_path_elm == path_split[i] {
                prepare_path.push_str(&format!("/{}", callback_path_elm));
            }
            /*
             * Dynamic
             */
            else if callback_path_char == ':' {
                prepare_path.push_str(&format!("/{}", callback_path_elm));
                param.push((callback_path_elm.replace(':', ""), path_split[i].to_owned()));
            }
        });
    /*
     * Match current path with prepare path
     */
    if callback_path == prepare_path {
        return IsFind { find: true, param };
    }

    IsFind { find: false, param }
}
