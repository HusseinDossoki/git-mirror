use crate::azure_devops::custom as azure_custom;
use crate::azure_devops::projects as azure_project;
use crate::git_operations::repository_mirroring;
use crate::tauri_apis::models;

#[tauri::command]
pub async fn mirror_repository(src_repo_url: String, dest_repo_url: String) -> Result<(), String> {
    let res = repository_mirroring::mirror_repository(src_repo_url, dest_repo_url).await;
    match res {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => return Err(err),
    }
}

#[tauri::command]
pub async fn get_azure_devops_projects(
    params: models::OrganizationRef,
) -> Result<Vec<azure_project::list::AzureProject>, String> {
    let res = azure_project::list::get_projects(&params.pat, &params.organization_name).await;

    match res {
        Ok(data) => {
            return Ok(data.value);
        }
        Err(err) => return Err(err),
    }
}

#[tauri::command]
pub async fn sync_azure_devops_project(
    params: azure_custom::mirror_two_projects::Params,
) -> Result<(), String> {
    let res = azure_custom::mirror_two_projects::mirror_project(params).await;

    match res {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => return Err(err),
    }
}
