use reqwest::Client;
use json;

mod agent;

use crate::agent::agent::enroll_agent;
use crate::agent::agent::post_item;

const ip: &str = "http://127.0.0.1:8080";

#[tokio::main]
async fn main() {

    let future_id = enroll_agent(ip); 
    let response = future_id.await;
    let json = match response.unwrap().text().await {
        Ok(i) => {println!("json: {:?}", i.clone()); i},
        Err(e) => panic!("Failed to read reponse text with: {}", e),
    };
    let id = &json::parse(&json).unwrap()["id"];
    println!("id to string: {}", id.to_string());

    let client = Client::new();

    let response = match client.get("https://scrapeme.live/shop")
        .send()
        .await {
            Ok(n) => n,
            Err(e) => panic!("Unable to get html content: {}", e), 
        };
    let html_content = response.text().await.unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_product_selector = 
        scraper::Selector::parse("li.product") //read straight from html
            .unwrap();
    let html_products = document.select(&html_product_selector);

    for product in html_products {
        let product_name = product.select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());

        let product_price = product.select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        match post_item(ip, &id.to_string(), &product_name.clone().unwrap(), product_price.unwrap().strip_prefix('£').unwrap()).await { 
            Ok(_) => println!("Posted {}", product_name.clone().unwrap()),
            Err(e) => panic!("Failed to post product {} with error: {:?}", product_name.clone().unwrap(), e),
        }
    }
}

    
