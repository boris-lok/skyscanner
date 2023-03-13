use chrono::{Datelike, NaiveDate, Weekday};

pub fn check_date_is_weekend(date: chrono::DateTime<chrono::Local>) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

pub fn parse_date(year: u16, month: u8, day: u8) -> NaiveDate {
    let formatted_date = format!("{year}-{month}-{day}");
    NaiveDate::parse_from_str(formatted_date.as_str(), "%Y-%m-%d")
        .expect(format!("Failed to parse date {formatted_date}").as_str())
}

#[cfg(test)]
mod test {
    use super::check_date_is_weekend;
    use chrono::DateTime;

    #[test]
    fn it_works_when_checking_the_date_is_weekend() {
        let date_str = vec![
            "2023-03-06T00:00:00+08:00", // Mon
            "2023-03-07T00:00:00+08:00", // Tue
            "2023-03-08T00:00:00+08:00", // Wed
            "2023-03-09T00:00:00+08:00", // Thu
            "2023-03-10T00:00:00+08:00", // Fri
            "2023-03-11T00:00:00+08:00", // Sat
            "2023-03-12T00:00:00+08:00", // Sun
        ];
        let expected_values = vec![false, false, false, false, false, true, true];
        for (&s, &expected_value) in date_str.iter().zip(expected_values.iter()) {
            let date = DateTime::parse_from_rfc3339(s)
                .unwrap()
                .with_timezone(&chrono::Local);

            assert_eq!(check_date_is_weekend(date), expected_value);
        }
    }
}
