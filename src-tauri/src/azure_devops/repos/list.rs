use crate::azure_devops::headers::request_headers;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AzureRepoList {
    pub count: u16,
    pub value: Vec<AzureRepo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AzureRepo {
    pub id: String,
    pub name: String,
    pub url: String,
    pub remoteUrl: String,
}

pub struct Params {
    pub pat: String,
    pub organization_name: String,
    pub project_name: String,
}

pub async fn get_repos(params: Params) -> Result<AzureRepoList, String> {
    let url = format!(
        "https://dev.azure.com/{}/{}/_apis/git/repositories?api-version=7.0",
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
                "Failed to get the repositories for the project '{}/{}'",
                params.organization_name, params.project_name
            ))
        }
    };

    let result = match res.json::<AzureRepoList>().await {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "Failed to get the repositories for the project '{}/{}'",
                params.organization_name, params.project_name
            ))
        }
    };

    return Ok(result);
}
