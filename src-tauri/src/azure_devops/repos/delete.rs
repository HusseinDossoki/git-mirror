use crate::azure_devops::headers::request_headers;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Params {
    pub pat: String,
    pub organization_name: String,
    pub project_id: String,
    pub repo_id: String,
}

pub async fn delete_repo(params: Params) -> Result<(), String> {
    let url = format!(
        "https://dev.azure.com/{}/{}/_apis/git/repositories/{}?api-version=7.0",
        params.organization_name, params.project_id, params.repo_id
    );

    let client = reqwest::Client::new();

    match client
        .delete(url)
        .headers(request_headers(&params.pat))
        .send()
        .await
    {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "Failed to delete the repository '{}' in the project '{}/{}'",
                params.repo_id, params.organization_name, params.project_id
            ))
        }
    };

    return Ok(());
}
