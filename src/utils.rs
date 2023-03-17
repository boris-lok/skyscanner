use chrono::{Datelike, Days, NaiveDate, Weekday};

pub fn check_date_is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

pub fn parse_date(year: i32, month: u16, day: u16) -> NaiveDate {
    let formatted_date = format!("{year}-{month}-{day}");
    NaiveDate::parse_from_str(formatted_date.as_str(), "%Y-%m-%d")
        .unwrap_or_else(|_| panic!("Failed to parse date {formatted_date}"))
}

pub fn parse_input_days(s: &str) -> Result<Vec<Vec<u16>>, String> {
    Ok(s.trim()
        .split(':')
        .into_iter()
        .map(|e| {
            e.trim()
                .split(',')
                .into_iter()
                .filter_map(|ee| ee.parse::<u16>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
}

pub fn create_dates(
    year: i32,
    months: Vec<u16>,
    days: &[Vec<u16>],
    durations: Vec<u64>,
) -> Vec<(NaiveDate, NaiveDate)> {
    months
        .iter()
        .zip(days.iter())
        .flat_map(|(m, ds)| {
            ds.iter()
                .map(|d| parse_date(year, *m, *d))
                .collect::<Vec<_>>()
        })
        .flat_map(|from| {
            durations
                .iter()
                .filter_map(|d| from.checked_add_days(Days::new(*d)))
                .map(|to| (from, to))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_when_checking_the_date_is_weekend() {
        let naive_dates = vec![
            parse_date(2023, 3, 6),  // Mon
            parse_date(2023, 3, 7),  // Tue
            parse_date(2023, 3, 8),  // Wed
            parse_date(2023, 3, 9),  // Thu
            parse_date(2023, 3, 10), // Fri
            parse_date(2023, 3, 11), // Sat
            parse_date(2023, 3, 12), // Sun
        ];

        let expected_values = vec![false, false, false, false, false, true, true];
        for (&naive_date, &expected_value) in naive_dates.iter().zip(expected_values.iter()) {
            assert_eq!(check_date_is_weekend(naive_date), expected_value);
        }
    }

    #[test]
    fn it_works_when_parsing_input_days() {
        let s = "3,4:5,6";
        let result = parse_input_days(s);
        assert!(result.is_ok());
        if let Ok(res) = result {
            assert_eq!(res.len(), 2);
            assert_eq!(res, vec![vec![3, 4], vec![5, 6]]);
        }
    }

    #[test]
    fn it_works_when_creating_date() {
        let year = 2023;
        let months = vec![4, 5];
        let days = vec![vec![1, 2], vec![3, 4]];
        let durations = vec![3, 4];

        let expected_values = vec![
            (parse_date(2023, 4, 1), parse_date(2023, 4, 4)),
            (parse_date(2023, 4, 1), parse_date(2023, 4, 5)),
            (parse_date(2023, 4, 2), parse_date(2023, 4, 5)),
            (parse_date(2023, 4, 2), parse_date(2023, 4, 6)),
            (parse_date(2023, 5, 3), parse_date(2023, 5, 6)),
            (parse_date(2023, 5, 3), parse_date(2023, 5, 7)),
            (parse_date(2023, 5, 4), parse_date(2023, 5, 7)),
            (parse_date(2023, 5, 4), parse_date(2023, 5, 8)),
        ];

        let res = create_dates(year, months, &days, durations);
        assert_eq!(res.len(), expected_values.len());
        assert_eq!(res, expected_values);
    }
}
