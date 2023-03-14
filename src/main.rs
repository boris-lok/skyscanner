use chrono::{Datelike, Days};
use clap::value_parser;
use skyscanner::configuration::get_configuration;
use skyscanner::datasource::Datasource;
use skyscanner::domain::{Date, Place, Query, QueryLeg};
use skyscanner::services::Services;
use skyscanner::utils::{parse_date, parse_input_days};

#[tokio::main]
async fn main() {
    let matches = clap::Command::new("raw")
        .arg(clap::Arg::new("from").long("from").required(true))
        .arg(clap::Arg::new("to").long("to").required(true))
        .arg(
            clap::Arg::new("year")
                .long("year")
                .value_parser(value_parser!(i32))
                .required(true),
        )
        .arg(
            clap::Arg::new("duration")
                .long("duration")
                .short('d')
                .value_parser(value_parser!(u64))
                .required(true),
        )
        .arg(
            clap::Arg::new("months")
                .action(clap::ArgAction::Set)
                .short('m')
                .long("months")
                .value_delimiter(',')
                .required(true),
        )
        .arg(
            clap::Arg::new("days")
                .long("days")
                .value_parser(clap::builder::ValueParser::new(parse_input_days))
                .required(true),
        )
        .arg(
            clap::Arg::new("locale")
                .long("locale")
                .default_value("zh-TW"),
        )
        .arg(clap::Arg::new("market").long("market").default_value("TW"))
        .arg(
            clap::Arg::new("currency")
                .long("currency")
                .default_value("TWD"),
        )
        .get_matches();

    let config = get_configuration().expect("Can't get the configuration.");

    let services = Services::new(&config).expect("Can't create a services");

    let market = matches
        .get_one::<String>("market")
        .map(|e| e.to_uppercase())
        .expect("Invalid market");

    let currency = matches
        .get_one::<String>("currency")
        .map(|e| e.to_uppercase())
        .expect("Invalid currency");

    let from = matches.get_one::<String>("from").expect("Invalid from");
    let to = matches.get_one::<String>("to").expect("Invalid to");
    let locale = matches.get_one::<String>("locale").expect("Invalid locale");
    let year = matches.get_one::<i32>("year").expect("Invalid year");
    let months = matches
        .get_many::<String>("months")
        .expect("Invalid months")
        .collect::<Vec<_>>();
    let days = matches
        .get_one::<Vec<Vec<u16>>>("days")
        .expect("Invalid days");
    let duration = matches
        .get_one::<u64>("duration")
        .expect("Invalid duration");

    if months.len() != days.len() {
        panic!("Invalid input, months length should equal days length.");
    }

    let mut q = Query::default();
    q = q.set_market(market);
    q = q.set_currency(currency);
    q = q.set_locale(locale.to_owned());

    let from = Place::new(Some(from.to_owned()), None);
    let to = Place::new(Some(to.to_owned()), None);

    let dates = months
        .iter()
        .zip(days.iter())
        .filter_map(|(m, d)| {
            m.parse::<u16>().ok().map(|m| {
                d.iter()
                    .map(|dd| parse_date(*year, m, *dd))
                    .collect::<Vec<_>>()
            })
        })
        .flatten()
        .filter_map(|from_date| {
            from_date
                .checked_add_days(Days::new(*duration))
                .map(|e| (from_date, e))
        })
        .collect::<Vec<_>>();

    dbg!(&dates);

    let mut result = vec![];
    for (start, end) in dates.iter() {
        let q = q.clone();
        let start = Date::new(start.year(), start.month(), start.day());
        let leg = QueryLeg::new(from.clone(), to.clone(), start);
        let q = q.set_query_leg(leg);
        let end = Date::new(end.year(), end.month(), end.day());
        let leg = QueryLeg::new(from.clone(), to.clone(), end);
        let q = q.set_query_leg(leg);
        let mut datasource = Datasource::new(q, services.clone());

        let data = datasource.next().await;

        if let Ok(Some(res)) = data {
            let formatted_results = res.content.results.format();
            result.extend(formatted_results);
        } else {
            println!("{:?}", data);
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    result.sort();

    result.iter().for_each(|r| println!("{}", r));
}
