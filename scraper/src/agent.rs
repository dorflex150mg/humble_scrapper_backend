pub mod agent {

    use reqwest::Client;
    use reqwest::Response;
    use std::collections::HashMap;

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

    pub async fn post_item(ip: &str, id: &str, name: &str, price: &str) -> Result<Response, String> {
        let address = ip.to_string() + "/add_item";
        let mut data = HashMap::new();
        data.insert("id", id);
        data.insert("name", name);
        data.insert("price", price);
        let client = Client::new();

        let response = match client.post(address)
            .json(&data)
            .send()
            .await {
                Ok(r) => r,
                Err(e) => panic!("Item post request failed with {:?}", e),
            };
        Ok(response)
    }
}
    

    
    

