use crate::azure_devops::headers::request_headers;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AzureProject {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub state: String,
    pub visibility: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Params {
    pub pat: String,
    pub organization_name: String,
    pub project_name: String,
    // includeCapabilities: bool,
    // includeHistory: bool,
}

pub async fn get_project(params: Params) -> Result<AzureProject, String> {
    let url = format!(
        "https://dev.azure.com/{}/_apis/projects/{}?api-version=7.0",
        params.organization_name, params.project_name
    );

    let client = reqwest::Client::new();

    let res = match client
        .get(url)
        .headers(request_headers(&params.pat))
        .send()
        .await
    {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "Failed to get the project '{}/{}'",
                params.organization_name, params.project_name
            ))
        }
    };

    let result = match res.json::<AzureProject>().await {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "Failed to get the project '{}/{}'",
                params.organization_name, params.project_name
            ))
        }
    };

    return Ok(result);
}
