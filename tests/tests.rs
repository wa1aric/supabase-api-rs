extern crate dotenv;

#[cfg(test)]
mod tests {

    use dotenv::dotenv;

    #[test]
    fn organizations_list_test() {
        
        dotenv().ok();

        let access_token = std::env::var("SUPABASE_ACCESS_TOKEN".to_string())
            .expect("Supabase access token not defined");
        let supabase = supabase_api::SupabaseAPI::new(access_token);
        let organizations = supabase.list_all_organizations();
        organizations.iter().for_each(|org| {
            println!("{} {}", org.id, org.name);
        });
    }

    #[test]
    fn projects_list_test() {
        
        dotenv().ok();

        let supabase_api = supabase_api::SupabaseAPI::new(
            std::env::var("SUPABASE_ACCESS_TOKEN".to_string()).expect(""),
        );
        supabase_api
            .list_all_projects()
            .iter()
            .for_each(|project| println!("{} {}", project.id, project.name))
    }
    
}
