use async_process::Command;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;

pub async fn mirror_repository(src_repo_url: String, dest_repo_url: String) -> Result<(), String> {
    let repo_name = src_repo_url.split("/").last().unwrap();
    println!("⌛️ Mirroring a repository '{}'...", repo_name);

    let folder_name = &generate_random_name();

    let res = git_clone_mirror(&src_repo_url, folder_name, &".".to_string()).await;
    if res.is_err() {
        fs::remove_dir_all(folder_name).expect("error");
        return Err(res.unwrap_err());
    }
    let res = set_push_url(&dest_repo_url, &folder_name).await;
    if res.is_err() {
        fs::remove_dir_all(folder_name).expect("error");
        return Err(res.unwrap_err());
    }
    let res = git_fetch(&folder_name).await;
    if res.is_err() {
        fs::remove_dir_all(folder_name).expect("error");
        return Err(res.unwrap_err());
    }
    let res = git_push(&folder_name).await;
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

async fn git_clone_mirror(
    src_repo_url: &String,
    folder_name: &str,
    working_dir: &String,
) -> Result<(), String> {
    let cmd = &["clone", "--mirror", &src_repo_url, folder_name];

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

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

    return Ok(());
}

async fn set_push_url(dest_repo_url: &String, working_dir: &String) -> Result<(), String> {
    let cmd = &["remote", "set-url", "--push", "origin", &dest_repo_url];

    let output = Command::new("git")
        .current_dir(working_dir)
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr while setting the push url"));

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(());
}

async fn git_fetch(working_dir: &String) -> Result<(), String> {
    let cmd = &["fetch", "-p", "origin"];

    let output = Command::new("git")
        .current_dir(working_dir)
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr while fetching"));

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(());
}

async fn git_push(working_dir: &String) -> Result<(), String> {
    let cmd = &["push", "--mirror", "--verbose"];

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
