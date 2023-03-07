use anyhow::Context;
use reqwest::{Error, Url};
use secrecy::ExposeSecret;

use crate::configuration::Settings;
use crate::domain::{CreateFlightsRequest, FlightsResponse, Locales, Markets};

pub struct Services {
    client: reqwest::Client,
    base_url: Url,
}

impl Services {
    pub fn new(config: &Settings) -> Result<Self, anyhow::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            reqwest::header::HeaderValue::from_str(config.api_key.expose_secret().as_str())
                .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("Can't build a request client")?;

        let url = Url::parse("https://partners.api.skyscanner.net/apiservices/v3/")?;

        Ok(Self {
            client,
            base_url: url,
        })
    }

    pub async fn get_markets(&self, locale: &str) -> Result<Markets, Error> {
        let uri = format!("culture/markets/{locale}");
        let url = self
            .base_url
            .join(uri.as_str())
            .expect("Can't join the url");

        let res = self.client.get(url.as_str()).send().await;

        match res {
            Ok(res) => res.json::<Markets>().await,
            Err(e) => Err(e),
        }
    }

    pub async fn get_locales(&self) -> Result<Locales, Error> {
        let uri = "culture/locales";
        let url = self.base_url.join(uri).expect("Can't join the url");

        let res = self.client.get(url.as_str()).send().await;

        match res {
            Ok(res) => res.json::<Locales>().await,
            Err(e) => Err(e),
        }
    }

    pub async fn create_a_request_to_find_flights(
        &self,
        q: &CreateFlightsRequest,
    ) -> Result<FlightsResponse, Error> {
        let uri = "flights/live/search/create";

        let url = self.base_url.join(uri).expect("Can't join the url");

        let res = self.client.post(url.as_str()).json(q).send().await;

        match res {
            Ok(res) => res.json::<FlightsResponse>().await,
            Err(e) => Err(e),
        }
    }
}
