use crate::azure_devops::headers::request_headers;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AzureProjectList {
    pub count: u16,
    pub value: Vec<AzureProject>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AzureProject {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub state: String,
    pub visibility: String,
}

pub async fn get_projects(
    pat: &String,
    organization_name: &String,
) -> Result<AzureProjectList, String> {
    let url = format!("https://dev.azure.com/{organization_name}/_apis/projects");

    let client = reqwest::Client::new();

    let res = match client.get(url).headers(request_headers(pat)).send().await {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "Failed to get the projects for the '{organization_name}' organization"
            ))
        }
    };

    let result = match res.json::<AzureProjectList>().await {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "Failed to get the projects payload for the '{organization_name}' organization"
            ))
        }
    };

    return Ok(result);
}
