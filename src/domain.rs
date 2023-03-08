use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(serde::Serialize)]
pub struct CreateFlightsRequest {
    pub query: Query,
}

#[derive(serde::Serialize)]
pub struct Query {
    pub market: String,
    pub locale: String,
    pub currency: String,
    pub query_legs: Vec<QueryLeg>,
    pub cabin_class: CabinClass,
    pub adults: u16,
    pub children_ages: Vec<u16>,
    pub include_carriers_ids: Vec<String>,
    pub exclude_carriers_ids: Vec<String>,
    pub include_agents_ids: Vec<String>,
    pub exclude_agents_ids: Vec<String>,
    pub include_sustain_ability_data: bool,
    pub near_by_airports: bool,
}

#[derive(serde::Serialize)]
pub struct QueryLeg {
    pub origin_place_id: Place,
    pub destination_place_id: Place,
    pub date: Date,
}

#[derive(serde::Serialize)]
pub struct Place {
    iata: Option<String>,
    entry_id: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(serde::Serialize)]
pub enum CabinClass {
    // Cabin class is not specified.
    #[serde(rename = "CABIN_CLASS_UNSPECIFIED")]
    CabinClassUnspecified,
    #[serde(rename = "CABIN_CLASS_ECONOMY")]
    // Cabin class is economy
    CabinClassEconomy,
    // Cabin class is premium economy
    #[serde(rename = "CABIN_CLASS_PREMIUM_ECONOMY")]
    CabinClassPremiumEconomy,
    // Cabin class is business
    #[serde(rename = "CABIN_CLASS_BUSINESS")]
    CabinClassBusiness,
    // Cabin class is first class
    #[serde(rename = "CABIN_CLASS_FIRST")]
    CabinClassFirst,
}

#[derive(serde::Deserialize)]
pub struct Markets {
    pub markets: Vec<Market>,
}

#[derive(serde::Deserialize)]
pub struct Market {
    pub code: String,
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct Locales {
    pub locales: Vec<Locale>,
}

#[derive(serde::Deserialize)]
pub struct Locale {
    pub code: String,
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlightsResponse {
    pub session_token: String,
    pub status: String,
    pub action: String,
    pub content: FightsContent,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FightsContent {
    pub results: FightResult,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FightResult {
    pub itineraries: HashMap<String, Itinerary>,
    pub legs: HashMap<String, Leg>,
    pub segments: HashMap<String, Segment>,
    pub places: HashMap<String, ResponsePlace>,
    pub carriers: HashMap<String, Carrier>,
    pub agents: HashMap<String, Agent>,
    pub alliances: HashMap<String, Alliance>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Itinerary {
    pub pricing_options: Vec<PriceOption>,
    pub leg_ids: Vec<String>,
    pub sustainability_data: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceOption {
    pub id: String,
    pub price: Price,
    pub agent_ids: Vec<String>,
    pub transfer_type: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub amount: String,
    pub unit: String,
    pub update_status: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Leg {
    pub origin_place_id: String,
    pub destination_place_id: String,
    pub departure_date_time: ResponseDateTime,
    pub arrival_date_time: ResponseDateTime,
    pub duration_in_minutes: u32,
    pub stop_count: u16,
    pub marketing_carrier_ids: Vec<String>,
    pub operating_carrier_ids: Vec<String>,
    pub segment_ids: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ResponseDateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub origin_place_id: String,
    pub destination_place_id: String,
    pub departure_date_time: ResponseDateTime,
    pub arrival_date_time: ResponseDateTime,
    pub duration_in_minutes: u32,
    pub marking_flight_number: Option<String>,
    pub marking_carrier_id: Option<String>,
    pub operating_carrier_id: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePlace {
    pub entity_id: String,
    pub parent_id: String,
    pub name: String,
    pub iata: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub coordinates: Option<Coordinates>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Carrier {
    pub name: String,
    pub alliance_id: String,
    pub image_url: String,
    pub iata: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub image_url: String,
    pub feedback_count: i32,
    pub rating: f64,
    pub rating_breakdown: Option<RatingBreakdown>,
    pub is_optimised_for_mobile: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RatingBreakdown {
    pub customer_service: f64,
    pub reliable_prices: f64,
    pub clear_extra_fees: f64,
    pub ease_of_booking: f64,
    pub other: f64,
}

#[derive(serde::Deserialize, Debug)]
pub struct Alliance {
    pub name: String,
}

impl Default for Query {
    fn default() -> Self {
        let leg = QueryLeg {
            origin_place_id: Place {
                iata: Some("TPE".to_string()),
                entry_id: None,
            },
            destination_place_id: Place {
                iata: Some("HKG".to_string()),
                entry_id: None,
            },
            date: Date {
                year: 2023,
                month: 6,
                day: 2,
            },
        };
        Self {
            market: "TW".to_string(),
            locale: "zh-TW".to_string(),
            currency: "TWD".to_string(),
            query_legs: vec![leg],
            cabin_class: CabinClass::CabinClassEconomy,
            adults: 1,
            children_ages: vec![],
            include_carriers_ids: vec![],
            exclude_carriers_ids: vec![],
            include_agents_ids: vec![],
            exclude_agents_ids: vec![],
            include_sustain_ability_data: false,
            near_by_airports: false,
        }
    }
}

impl Display for ResponseDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.unit, self.amount)
    }
}
