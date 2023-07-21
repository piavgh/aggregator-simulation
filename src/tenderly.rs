use std::env;

use serde_json::json;

struct TenderlyRepository {
    client: reqwest::Client,
}

impl TenderlyRepository {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    fn sample_approve(&self) {
        let tenderly_user = env::var("TENDERLY_USER").unwrap_or_default();
        let tenderly_project = env::var("TENDERLY_PROJECT").unwrap_or_default();
        let tenderly_access_key = env::var("TENDERLY_ACCESS_KEY").unwrap_or_default();

        let url = format!("https://api.tenderly.co/api/v1/account/{TENDERLY_USER}/project/{TENDERLY_PROJECT}/simulate", TENDERLY_USER = tenderly_user, TENDERLY_PROJECT = tenderly_project);

        println!("url: {}", url);

        // self.client.post(url).json(json!({
        //     "network": "mainnet",
        //     "contract": "0x6b175474e89094c44da98b954eedeac495271d0f",
        //     "method": "approve",
        //     "args": [
        //         "0x7a250d5630b4cf539739df2c5dacb4c659f2488d",
        //         ]
        //     }
        // ));
    }
}

#[cfg(test)]
mod tests {
    use crate::{env, tenderly};

    #[test]
    fn it_works() {
        env::load_env_files();
        let repo = tenderly::TenderlyRepository::new();
        repo.sample_approve();
    }
}
