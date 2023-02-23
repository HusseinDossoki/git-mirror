use async_process::Command;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;

pub async fn mirror_repository(
    src_pat: String,
    src_repo_url: String,
    dest_pat: String,
    dest_repo_url: String,
) -> Result<(), String> {
    let repo_name = src_repo_url.split("/").last().unwrap();
    println!("⌛️ Mirroring a repository '{}'...", repo_name);

    let src = format_url(&src_repo_url, &src_pat);
    let dest = format_url(&dest_repo_url, &dest_pat);

    let folder_name = &generate_random_name();

    let res = git_config().await;
    if res.is_err() {
        return Err(res.unwrap_err());
    }
    let res = git_clone_mirror(&src, folder_name, &".".to_string()).await;
    if res.is_err() {
        return Err(res.unwrap_err());
    }
    let res = git_push(&folder_name, &dest).await;
    if res.is_err() {
        fs::remove_dir_all(folder_name).expect("error");
        return Err(res.unwrap_err());
    }

    fs::remove_dir_all(folder_name).expect("error");

    println!(
        "✅ Repository '{}' has been mirrored successfully.",
        repo_name
    );

    return Ok(());
}

//#region Private functions

fn format_url(http_url: &String, pat: &String) -> String {
    if http_url.contains("@") {
        return http_url.replace("@", format!(":{}@", pat.clone()).as_str());
    } else {
        return http_url.replace("https://", format!("https://{}@", pat.clone()).as_str());
    }
}

fn generate_random_name() -> String {
    // We use (.) to hide the temp folder
    let prefix = ".temp_";

    let mut rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    rand_string.insert_str(0, prefix);

    return rand_string;
}

async fn git_config() -> Result<(), String> {
    // git config --global http.postBuffer 524288000
    // 500 MB: 524288000 (as posted in the original answer)
    // 1 GB: 1048576000
    // 2 GB: 2097152000 (anything higher is rejected as 'out of range')

    let cmd = &["config", "--global", "http.postBuffer", "1048576000"];

    let output = Command::new("git")
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr while cloning the repo"));

    if !output.status.success() {
        print!("{:#?}", String::from_utf8_lossy(&output.stderr).to_string());
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(());
}

async fn git_clone_mirror(
    src_repo_url: &String,
    folder_name: &str,
    working_dir: &String,
) -> Result<(), String> {
    let cmd = &["clone", "--bare", &src_repo_url, folder_name];

    let output = Command::new("git")
        .current_dir(working_dir)
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr while cloning the repo"));

    if !output.status.success() {
        print!("{:#?}", String::from_utf8_lossy(&output.stderr).to_string());
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(());
}

async fn git_push(working_dir: &String, dest: &String) -> Result<(), String> {
    let cmd = &["push", "--mirror", dest];

    let output = Command::new("git")
        .current_dir(working_dir)
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr while pushing the repo changes"));

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(());
}

//#endregion
