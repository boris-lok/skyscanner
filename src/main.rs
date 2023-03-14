use chrono::Datelike;
use clap::value_parser;
use futures::future::join_all;
use skyscanner::configuration::get_configuration;
use skyscanner::datasource::Datasource;
use skyscanner::domain::{Date, FlightsResponse, Place, Query, QueryLeg};
use skyscanner::services::Services;
use skyscanner::utils::{create_dates, parse_input_days};

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
                .value_delimiter(',')
                .required(true),
        )
        .arg(
            clap::Arg::new("months")
                .action(clap::ArgAction::Set)
                .short('m')
                .long("months")
                .value_parser(value_parser!(u16))
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
        .get_many::<u16>("months")
        .expect("Invalid months")
        .copied()
        .collect::<Vec<_>>();
    let days = matches
        .get_one::<Vec<Vec<u16>>>("days")
        .expect("Invalid days");
    let durations = matches
        .get_many::<u64>("duration")
        .expect("Invalid duration")
        .copied()
        .collect::<Vec<_>>();

    if months.len() != days.len() {
        panic!("Invalid input, months length should equal days length.");
    }

    let mut q = Query::default();
    q = q.set_market(market);
    q = q.set_currency(currency);
    q = q.set_locale(locale.to_owned());

    let from = Place::new(Some(from.to_owned()), None);
    let to = Place::new(Some(to.to_owned()), None);

    let dates = create_dates(*year, months, days, durations);

    let mut data_sources = dates
        .into_iter()
        .map(|(s, e)| {
            (
                Date::new(s.year(), s.month(), s.day()),
                Date::new(e.year(), e.month(), e.day()),
            )
        })
        .map(|(s, e)| {
            let mut q = q.clone();
            let leg = QueryLeg::new(from.clone(), to.clone(), s);
            q = q.set_query_leg(leg);
            let leg = QueryLeg::new(to.clone(), from.clone(), e);
            q = q.set_query_leg(leg);
            q
        })
        .map(|query| Datasource::new(query, services.clone()))
        .collect::<Vec<_>>();

    let mut result = vec![];

    for _ in 0..2 {
        let tasks = data_sources
            .iter_mut()
            .map(|e| e.next())
            .collect::<Vec<_>>();

        let res: Vec<anyhow::Result<Option<FlightsResponse>>> = join_all(tasks).await;
        result.extend(res);
        std::thread::sleep(std::time::Duration::from_secs(10));
    }

    let mut response = result
        .into_iter()
        .flatten()
        .flatten()
        .flat_map(|e| e.content.results.format())
        .collect::<Vec<_>>();
    response.sort();

    response.iter().rev().for_each(|f| println!("{}", f));
}
