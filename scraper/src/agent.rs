pub mod agent {

    use reqwest::Client;
    use reqwest::Response;

    pub async fn enroll_agent(ip: &str) -> Result<Response, String> {
        let address = ip.to_string() + "/create_agent";
        let client = Client::new();
        println!("Enrolling at: {}", address.clone());
    
        let response = match client.post(address)
            .send()
            .await {
                Ok(r) => r,
                Err(e) => panic!("Agent enrollment post request has failed with: {:?}", e),
            };
        Ok(response)
    }
}
    

    
    

