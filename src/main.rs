use chrono::{Datelike, Local, NaiveDate};

fn main() {
    let today = Local::now().date_naive();
    create_direcory_of(today)
}

fn create_direcory_of(date :NaiveDate) {
    println!("{}-{:02}-{:02}", date.year(), date.month(), date.day())
}

