use crate::azure_devops::headers::request_headers;

#[derive(serde::Serialize, Debug)]
struct CreateAzureRepoOptions {
    name: String,
    project: CreateAzureRepoProjectOptions,
}
#[derive(serde::Serialize, Debug)]
struct CreateAzureRepoProjectOptions {
    id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateAzureRepoResponse {
    pub id: String,
    pub name: String,
    pub url: String,
    pub remoteUrl: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Params {
    pub pat: String,
    pub organization_name: String,
    pub project_name: String,
    pub project_id: String,
    pub repo_name: String,
}

pub async fn create_repo(params: &Params) -> Result<CreateAzureRepoResponse, String> {
    let url = format!(
        "https://dev.azure.com/{}/{}/_apis/git/repositories?api-version=7.0",
        params.organization_name, params.project_name
    );

    let new_repo = CreateAzureRepoOptions {
        name: params.repo_name.clone(),
        project: CreateAzureRepoProjectOptions {
            id: params.project_id.clone(),
        },
    };

    let client = reqwest::Client::new();

    let res = match client
        .post(url)
        .headers(request_headers(&params.pat))
        .json(&new_repo)
        .send()
        .await
    {
        Ok(data) => data,
        Err(err) => {
            println!("{:#?}", err);
            return Err(format!(
                "Failed to create the repository '{}' for the project '{}/{}'",
                params.repo_name, params.organization_name, params.project_name
            ));
        }
    };

    let result = match res.json::<CreateAzureRepoResponse>().await {
        Ok(data) => data,
        Err(err) => {
            println!("{:#?}", err);
            return Err(format!(
                "Failed to create the repository '{}' for the project '{}/{}'",
                params.repo_name, params.organization_name, params.project_name
            ))
        }
    };

    return Ok(result);
}
