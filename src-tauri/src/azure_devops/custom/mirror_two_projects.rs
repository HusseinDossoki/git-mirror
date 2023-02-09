use crate::azure_devops::projects;
use crate::azure_devops::repos;
use crate::git_operations;
use futures::{future, FutureExt};
use std::{thread, time};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ProjectRef {
    pub pat: String,
    pub organization_name: String,
    pub project_name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Params {
    pub src_project_ref: ProjectRef,
    pub dest_project_ref: ProjectRef,
    pub create_project_if_not_exist: bool,
}

pub async fn mirror_project(params: Params) -> Result<(), String> {
    let src_project = match projects::get::get_project(projects::get::Params {
        pat: params.src_project_ref.pat.clone(),
        organization_name: params.src_project_ref.organization_name.clone(),
        project_name: params.src_project_ref.project_name.clone(),
    })
    .await
    {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let dest_project = match projects::get::get_project(projects::get::Params {
        pat: params.dest_project_ref.pat.clone(),
        organization_name: params.dest_project_ref.organization_name.clone(),
        project_name: params.dest_project_ref.project_name.clone(),
    })
    .await
    {
        Ok(data) => data,
        Err(err) => match params.create_project_if_not_exist {
            true => {
                let new_project_options = projects::create::Params {
                    pat: params.dest_project_ref.pat.clone(),
                    organization_name: params.dest_project_ref.organization_name.clone(),
                    project_name: params.dest_project_ref.project_name.clone(),
                    project_visibility: src_project.visibility.clone(),
                    project_description: src_project.description.clone(),
                };

                let new_project = match projects::create::create_project(&new_project_options).await
                {
                    Ok(_) => {
                        let new_parms = projects::get::Params {
                            pat: params.dest_project_ref.pat.clone(),
                            organization_name: params.dest_project_ref.organization_name.clone(),
                            project_name: params.dest_project_ref.project_name.clone(),
                        };

                        thread::sleep(time::Duration::from_millis(3000)); // Wait till the project created

                        projects::get::get_project(new_parms).await.unwrap()
                    }
                    Err(err2) => return Err(err2),
                };

                new_project
            }
            false => return Err(err),
        },
    };

    let src_repos = match repos::list::get_repos(repos::list::Params {
        pat: params.src_project_ref.pat.clone(),
        organization_name: params.src_project_ref.organization_name.clone(),
        project_name: params.src_project_ref.project_name.clone(),
    })
    .await
    {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let dest_repos = match repos::list::get_repos(repos::list::Params {
        pat: params.dest_project_ref.pat.clone(),
        organization_name: params.dest_project_ref.organization_name.clone(),
        project_name: params.dest_project_ref.project_name.clone(),
    })
    .await
    {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    // Clean up the dest repos
    let default_repo_id = create_default_repo(
        &params.dest_project_ref,
        &dest_project.id,
        &dest_repos.value,
    )
    .await;

    let mut tasks = vec![];
    for repo in dest_repos.value {
        let task = delete_repo(
            repo.id.clone(),
            repo.name.clone(),
            &params.dest_project_ref.pat,
            &params.dest_project_ref.organization_name,
            &dest_project.id,
        );

        tasks.push(task.boxed());
    }

    let tasks_res = future::join_all(tasks).await;
    for item in tasks_res {
        if item.is_err() {
            return Err(item.unwrap_err());
        }
    }

    // Create the new repos
    let mut tasks = vec![];

    for repo in src_repos.value {
        let task = clone_repo(
            repo.name,
            repo.remoteUrl,
            &params.src_project_ref.pat,
            &params.dest_project_ref.pat,
            &params.dest_project_ref.organization_name,
            &params.dest_project_ref.project_name,
            &dest_project.id,
        );
        tasks.push(task.boxed());
    }

    let remove_task = remove_default_repo(
        &params.dest_project_ref,
        &dest_project.id,
        default_repo_id.unwrap().clone(),
    );

    tasks.push(remove_task.boxed());

    let tasks_res = future::join_all(tasks).await;
    for item in tasks_res {
        if item.is_err() {
            return Err(item.unwrap_err());
        }
    }

    return Ok(());
}

async fn create_default_repo(
    project_ref: &ProjectRef,
    project_id: &String,
    dest_repos: &Vec<repos::list::AzureRepo>,
) -> Result<String, String> {
    let default_repo = dest_repos.into_iter().find(|x| x.name == "default");

    if default_repo.is_none() {
        match repos::create::create_repo(&repos::create::Params {
            pat: project_ref.pat.clone(),
            organization_name: project_ref.organization_name.clone(),
            project_name: project_ref.project_name.clone(),
            project_id: project_id.clone(),
            repo_name: "default".to_string(),
        })
        .await
        {
            Ok(data) => return Ok(data.id),
            Err(err) => return Err(err),
        }
    }

    return Ok(default_repo.unwrap().id.clone());
}

async fn remove_default_repo(
    project_ref: &ProjectRef,
    project_id: &String,
    repo_id: String,
) -> Result<(), String> {
    let _ = repos::delete::delete_repo(repos::delete::Params {
        pat: project_ref.pat.clone(),
        organization_name: project_ref.organization_name.clone(),
        project_id: project_id.clone(),
        repo_id: repo_id.clone(),
    })
    .await;

    return Ok(());
}

async fn delete_repo(
    repo_id: String,
    repo_name: String,
    pat: &String,
    organization_name: &String,
    project_id: &String,
) -> Result<(), String> {
    if repo_name == "default" {
        return Ok(());
    }

    let res = repos::delete::delete_repo(repos::delete::Params {
        pat: pat.clone(),
        organization_name: organization_name.clone(),
        project_id: project_id.clone(),
        repo_id: repo_id.clone(),
    })
    .await;

    if res.is_err() {
        return Err(res.err().unwrap());
    }

    return Ok(());
}

async fn clone_repo(
    repo_name: String,
    repo_remote_url: String,
    src_pat: &String,
    dest_pat: &String,
    organization_name: &String,
    project_name: &String,
    project_id: &String,
) -> Result<(), String> {
    let new_repo = match repos::create::create_repo(&repos::create::Params {
        pat: dest_pat.clone(),
        organization_name: organization_name.clone(),
        project_name: project_name.clone(),
        project_id: project_id.clone(),
        repo_name: repo_name.clone(),
    })
    .await
    {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    match git_operations::repository_mirroring::mirror_repository(
        src_pat.clone(),
        repo_remote_url.clone(),
        dest_pat.clone(),
        new_repo.remoteUrl,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
    }
}
