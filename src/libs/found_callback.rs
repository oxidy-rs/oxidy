use std::collections::HashMap;

/*
 * Get Callback for Route in Parallel
 */
pub(crate) struct IsFound {
    pub(crate) found: bool,
    pub(crate) path: String,
    pub(crate) params: HashMap<String, String>,
}

pub(crate) async fn found_callback(path: String, path_curr: String) -> IsFound {
    /*
     * Static Match
     */
    if path_curr == path {
        return IsFound {
            found: true,
            path: path_curr,
            params: HashMap::new(),
        };
    }

    let path_split: Vec<String> = path
        .split("/")
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();

    /*
     * Dynamic Match
     */
    let path_curr_split: Vec<String> = path_curr
        .split("/")
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();

    /*
     * Check Split Length
     */
    if path_curr_split.len() != path_split.len() {
        return IsFound {
            found: false,
            path: "".to_string(),
            params: HashMap::new(),
        };
    }

    let mut prepare_path: String = String::from("");
    let mut params: HashMap<String, String> = HashMap::new();

    path_curr_split
        .into_iter()
        .enumerate()
        .for_each(|(j, path_curr_elm)| {
            let path_char: String = path_curr_elm.clone().chars().nth(0).unwrap().to_string();
            /*
             * Dynamic Param
             */
            if path_char == ":" {
                prepare_path.push_str(&format!("/{}", path_curr_elm));
                params.insert(path_curr_elm.replace(":", ""), path_split[j].to_string());
            }
            /*
             * Static Param
             */
            else if path_curr_elm == path_split[j] {
                prepare_path.push_str(&format!("/{}", path_curr_elm));
            }
        });

    /*
     * Match current path with prepare path
     */
    if path_curr == prepare_path {
        return IsFound {
            found: true,
            path: prepare_path,
            params,
        };
    }

    IsFound {
        found: false,
        path: "".to_string(),
        params: HashMap::new(),
    }
}
