use reqwest::Client;

#[tokio::main]
async fn main() {
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
        println!("name: {:?}, price: {:?}", product_name, product_price);
    }
}
