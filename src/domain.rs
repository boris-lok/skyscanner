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
                month: 4,
                day: 1,
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
