use async_process::Command;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;

pub async fn mirror_repository(src_repo_url: String, dest_repo_url: String) -> Result<(), String> {
    let folder_name = &generate_random_name();

    git_clone_mirror(src_repo_url, folder_name).await;
    std::env::set_current_dir(folder_name).expect("Invalid paramater");
    set_push_url(dest_repo_url).await;
    git_fetch().await;
    git_push().await;

    std::env::set_current_dir("..").expect("Invalid paramater");
    fs::remove_dir_all(folder_name).expect("error");

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

async fn git_clone_mirror(src_repo_url: String, folder_name: &str) {
    let command = "git clone --mirror";

    println!("\n'{}' is executing....\n", command);

    let cmd = &["clone", "--mirror", &src_repo_url, folder_name];

    Command::new("git")
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr when run '{}'", command));

    println!("\n'{}' has been completed\n", command);
}

async fn set_push_url(dest_repo_url: String) {
    let command = "git remote set-url --push ";
    println!("\n'{}' is executing....\n", command);

    let cmd = &["remote", "set-url", "--push", "origin", &dest_repo_url];

    Command::new("git")
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr when run '{}'", command));

    println!("\n'{}' has been completed\n", command);
}

async fn git_fetch() {
    let command = "git fetch -p origin";
    println!("\n'{}' is executing....\n", command);

    let cmd = &["fetch", "-p", "origin"];

    Command::new("git")
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr when run '{}'", command));

    println!("\n'{}' has been completed\n", command);
}

async fn git_push() {
    let command = "git push --mirror";
    println!("\n'{}' is executing....\n", command);

    let cmd = &["push", "--mirror", "--verbose"];

    Command::new("git")
        .args(cmd)
        .output()
        .await
        .expect(&format!("Erorr when run '{}'", command));

    println!("\n'{}' has been completed\n", command);
}

//#endregion
