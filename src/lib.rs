use chrono::prelude::{Date, Local, TimeZone};

const _MONTHS: [&str; 13] = [
    "_offset",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// today() returns a string with the current date formatted as number of days in March 2020
fn today() -> String {
    let start: Date<Local> = Local.ymd(2020, 3, 1);
    let today: Date<Local> = Local::today();
    let duration = today - start;
    return format!("{} {}, {}", "March", duration.num_days(), 2020);
}

#[cfg(test)]
mod tests {
    use super::today;

    #[test]
    fn it_works() {
        let _result = today();
    }
}
