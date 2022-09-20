use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub region: String,
    pub created_at: String
}

impl super::SupabaseAPI {

    pub fn list_all_projects(&self) -> Vec<Project> {
        serde_json::from_str(&self.get("/v1/projects")).unwrap()
    }

}