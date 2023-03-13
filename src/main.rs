use skyscanner::configuration::get_configuration;
use skyscanner::domain::{CreateFlightsRequest, Date, Place, Query, QueryLeg};
use skyscanner::services::Services;

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Can't get the configuration.");

    let services = Services::new(&config).expect("Can't create a services");

    let mut q = Query::default();
    q = q.set_market("TW".to_string());
    q = q.set_currency("TWD".to_string());
    q = q.set_locale("zh-TW".to_string());

    let from = Place::new(Some("TPE".to_string()), None);
    let to = Place::new(Some("HKG".to_string()), None);
    let date = Date::new(2023, 6, 2);
    let leg = QueryLeg::new(from, to, date);
    q = q.set_query_leg(leg);

    let from = Place::new(Some("HKG".to_string()), None);
    let to = Place::new(Some("TPE".to_string()), None);
    let date = Date::new(2023, 6, 5);
    let leg = QueryLeg::new(from, to, date);
    q = q.set_query_leg(leg);

    let q = CreateFlightsRequest::new(q);

    let res = services.create_a_request_to_find_flights(&q).await;

    if let Ok(res) = res {
        println!("{}", res.content.results);
    }
}
