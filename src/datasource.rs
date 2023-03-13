use crate::domain::{CreateFlightsRequest, FlightsResponse, Query, ResponseStatus};
use crate::services::Services;
use reqwest::Error;

pub struct Datasource {
    session_token: Option<String>,
    services: Services,
    query: Query,
    completed: bool,
}

impl Datasource {
    pub fn new(query: Query, services: Services) -> Self {
        Self {
            query,
            services,
            session_token: None,
            completed: false,
        }
    }

    pub async fn next(&mut self) -> anyhow::Result<Option<FlightsResponse>> {
        if self.completed {
            return Ok(None);
        }
        if self.session_token.is_none() {
            let req = CreateFlightsRequest {
                query: self.query.clone(),
            };
            let res = self.services.create_a_request_to_find_flights(&req).await?;
            if let Some(res) = res {
                self.session_token = Some(res.session_token.clone());
                self.completed = res.status == ResponseStatus::ResultStatusComplete;
                Ok(Some(res))
            } else {
                Ok(None)
            }
        } else if let Some(token) = &self.session_token {
            let res = self.services.poll_a_request_to_find_flights(token).await?;
            self.completed = res.status == ResponseStatus::ResultStatusComplete;
            Ok(Some(res))
        } else {
            unreachable!()
        }
    }
}
