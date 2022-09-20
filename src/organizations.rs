use serde::{Serialize, Deserialize, de::Error};

#[derive(Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String
}

impl super::SupabaseAPI {

    pub fn list_all_organizations(&self) -> Vec<Organization> {
        let response = self.get("/v1/organizations");
        let organizations = serde_json::from_str(&response).unwrap();
        organizations
    }

}