const BASE_URL: &'static str = "https://api.supabase.com";

pub struct SupabaseAPI {
    access_token: String,
    client: reqwest::blocking::Client
}

impl SupabaseAPI {
    
    pub fn new(access_token: String) -> SupabaseAPI {
        SupabaseAPI { 
            access_token,
            client: reqwest::blocking::Client::new()
        }
    }

    fn get(&self, endpoint: &str) -> String {
        self.client.get(&(BASE_URL.to_owned() + endpoint))
            .bearer_auth(self.access_token.clone())
            .send()
            .unwrap()
            .text()
            .unwrap()
    }

}

pub mod organizations;
pub mod projects;