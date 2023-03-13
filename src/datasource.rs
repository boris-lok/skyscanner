use crate::domain::{CreateFlightsRequest, FlightsResponse, Query};
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

    pub async fn next(&mut self) -> Result<Option<FlightsResponse>, Error> {
        if self.completed {
            return Ok(None);
        }
        if self.session_token.is_none() {
            let req = CreateFlightsRequest {
                query: self.query.clone(),
            };
            let res = self.services.create_a_request_to_find_flights(&req).await;
            match res {
                Ok(res) => {
                    self.session_token = Some(res.session_token.clone());
                    Ok(Some(res))
                }
                Err(e) => Err(e),
            }
        } else if let Some(token) = &self.session_token {
            let res = self.services.poll_a_request_to_find_flights(token).await;
            match res {
                Ok(res) => {
                    if res.status.eq("RESULT_STATUS_COMPLETE") {
                        self.completed = true;
                    }
                    Ok(Some(res))
                }
                Err(e) => Err(e),
            }
        } else {
            unreachable!()
        }
    }
}
