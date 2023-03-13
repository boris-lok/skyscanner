use chrono::{Datelike, Days};
use clap::Parser;
use skyscanner::configuration::get_configuration;
use skyscanner::datasource::Datasource;
use skyscanner::domain::{Cli, Date, Place, Query, QueryLeg};
use skyscanner::services::Services;
use skyscanner::utils::parse_date;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = get_configuration().expect("Can't get the configuration.");

    let services = Services::new(&config).expect("Can't create a services");

    let mut q = Query::default();
    q = q.set_market(cli.market.to_uppercase());
    q = q.set_currency(cli.currency.to_uppercase());
    q = q.set_locale(cli.locale.to_uppercase());

    let from = Place::new(Some(cli.from.to_uppercase()), None);
    let to = Place::new(Some(cli.to.to_uppercase()), None);

    let dates = cli
        .months
        .iter()
        .zip(cli.days.iter())
        .map(|(m, d)| parse_date(cli.year, *m, *d))
        .filter_map(|from_date| {
            from_date
                .checked_add_days(Days::new(cli.duration as u64))
                .map(|e| (from_date, e))
        })
        .collect::<Vec<_>>();

    dbg!(&dates);

    let mut result = vec![];
    for (start, end) in dates.iter() {
        let q = q.clone();
        let start = Date::new(start.year() as u16, start.month() as u8, start.day() as u8);
        let leg = QueryLeg::new(from.clone(), to.clone(), start);
        let q = q.set_query_leg(leg);
        let end = Date::new(end.year() as u16, end.month() as u8, end.day() as u8);
        let leg = QueryLeg::new(from.clone(), to.clone(), end);
        let q = q.set_query_leg(leg);
        let mut datasource = Datasource::new(q, services.clone());

        let data = datasource.next().await;

        if let Ok(Some(res)) = data {
            let formatted_results = res.content.results.format();
            result.extend(formatted_results);
        }
    }

    result.sort();

    result.iter().for_each(|r| println!("{}", r));
}
