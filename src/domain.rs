#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct QueryLeg {
    pub origin_place_id: Place,
    pub destination_place_id: Place,
    pub date: Date,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
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

impl Default for Query {
    fn default() -> Self {
        Self {
            market: "".to_string(),
            locale: "".to_string(),
            currency: "".to_string(),
            query_legs: vec![],
            cabin_class: CabinClass::CabinClassUnspecified,
            adults: 0,
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
