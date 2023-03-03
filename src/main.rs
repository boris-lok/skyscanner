use skyscanner::configuration::get_configuration;
use skyscanner::domain::{CreateFlightsRequest, Query};
use skyscanner::services::Services;

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Can't get the configuration.");

    let services = Services::new(&config).expect("Can't create a services");

    let q = CreateFlightsRequest {
        query: Query::default()
    };

    services.create_a_request_to_find_flights(&q).await;
}
