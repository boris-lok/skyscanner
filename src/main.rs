use skyscanner::configuration::get_configuration;
use skyscanner::domain::{CreateFlightsRequest, Query};
use skyscanner::services::Services;

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Can't get the configuration.");

    let services = Services::new(&config).expect("Can't create a services");

    let q = CreateFlightsRequest {
        query: Query::default(),
    };

    let res = services.create_a_request_to_find_flights(&q).await;

    if let Ok(res) = res {
        for (_, itinerary) in res.content.results.itineraries.iter() {
            for leg_id in itinerary.leg_ids.iter() {
                let leg = res.content.results.legs.get(leg_id);
                if let Some(leg) = leg {
                    println!(
                        "departure date: {} arrival date: {}, carrier ids: {:?}, operating carrier ids: {:?}",
                        leg.departure_date_time, leg.arrival_date_time, leg.marketing_carrier_ids, leg.operating_carrier_ids,
                    );
                }
            }
        }
    }
}
