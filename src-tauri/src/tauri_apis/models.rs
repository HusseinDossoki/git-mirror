#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ProjectRef {
    pub pat: String,
    pub organization_name: String,
    pub project_name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OrganizationRef {
    pub pat: String,
    pub organization_name: String,
}
