use crate::azure_devops::headers::request_headers;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Params {
    pub pat: String,
    pub organization_name: String,
    pub project_id: String,
}

pub async fn delete_project(params: Params) -> Result<(), String> {
    let url = format!(
        "https://dev.azure.com/{}/_apis/projects/{}?api-version=7.0",
        params.organization_name, params.project_id
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
                "Failed to delete the project '{}/{}'",
                params.organization_name, params.project_id
            ))
        }
    };

    return Ok(());
}
