use crate::azure_devops::headers::request_headers;

#[derive(serde::Serialize, Debug)]
struct CreateAzureProjectOptions {
    name: String,
    description: Option<String>,
    capabilities: Capabilities,
    visibility: String, // private, public
}

#[derive(serde::Serialize, Debug)]
struct Capabilities {
    processTemplate: ProcessTemplate,
    versioncontrol: Versioncontrol,
}

#[derive(serde::Serialize, Debug)]
struct Versioncontrol {
    sourceControlType: String,
}

#[derive(serde::Serialize, Debug)]
struct ProcessTemplate {
    templateTypeId: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateAzureProjectResponse {
    pub id: String,
    pub status: String,
    pub url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Params {
    pub pat: String,
    pub organization_name: String,
    pub project_name: String,
    pub project_description: Option<String>,
    pub project_visibility: String,
}

pub async fn create_project(params: &Params) -> Result<CreateAzureProjectResponse, String> {
    let url = format!(
        "https://dev.azure.com/{}/_apis/projects?api-version=7.0",
        params.organization_name
    );

    let capabilities = Capabilities {
        versioncontrol: Versioncontrol {
            sourceControlType: "Git".to_string(),
        },
        processTemplate: ProcessTemplate {
            templateTypeId: "6b724908-ef14-45cf-84f8-768b5384da45".to_string(),
        },
    };

    let new_project = CreateAzureProjectOptions {
        name: params.project_name.clone(),
        description: params.project_description.clone(),
        capabilities: capabilities,
        visibility: params.project_visibility.clone(),
    };

    let client = reqwest::Client::new();

    let res = match client
        .post(url)
        .headers(request_headers(&params.pat))
        .json(&new_project)
        .send()
        .await
    {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "Failed to create the project '{}/{}'",
                params.organization_name, params.project_name
            ))
        }
    };

    let result = match res.json::<CreateAzureProjectResponse>().await {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "Failed to create the project '{}/{}'",
                params.organization_name, params.project_name
            ))
        }
    };

    return Ok(result);
}
