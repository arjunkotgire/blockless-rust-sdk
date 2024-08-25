use reqwest::Client;
use std::error::Error;

pub struct BlocklessAPI {
    base_url: String,
    client: Client,
}

impl BlocklessAPI {
    pub fn new(base_url: &str) -> Self {
        BlocklessAPI {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;
        Ok(response.text().await?)
    }

    pub async fn post(&self, endpoint: &str, body: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.post(&url).body(body.to_string()).send().await?;
        Ok(response.text().await?)
    }
}
