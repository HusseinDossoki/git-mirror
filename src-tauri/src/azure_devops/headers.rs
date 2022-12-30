use base64;

pub fn request_headers(pat: &String) -> reqwest::header::HeaderMap {
    let encoded_pat = encode_pat_base64(pat);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append("content-type", "application/json".parse().unwrap());
    headers.append("authorization", encoded_pat.parse().unwrap());
    return headers;
}

fn encode_pat_base64(pat: &String) -> String {
    //  echo $(printf "%s"":<PAT>" | base64)
    const TOKEN: &str = "%s\"\":";
    let data = format!("{TOKEN}{pat}");
    let base64 = format!("Basic {}", base64::encode(data));
    return base64;
}
