extern crate ical;

use clap::{Arg, App};
use slint::{ModelRc, VecModel, SharedString};
use chrono::{Datelike, Month};
use num_traits::FromPrimitive;
use std::rc::Rc;
//use std::process::exit;

slint::include_modules!();

fn gen_days(numdays: i32) -> Vec<DayData> {
    let mut days: Vec<DayData> = Vec::new();
    for i in 1..numdays {
        days.push(DayData {
            daynum: SharedString::from(i.to_string()),
        });
    }
    return days;
}

fn gen_month(month: &str) -> MonthData { MonthData { current_month: SharedString::from(month.to_owned()) } }
fn gen_year(year: &str) -> YearData { YearData { current_year: SharedString::from(year.to_owned()) } }

fn month() -> String { Month::from_u32(chrono::Utc::now().month()).unwrap().name().to_owned() }
fn year() -> String { chrono::Utc::now().year().to_string() }

fn main() {
    let matches = App::new("Timesync")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("calpath").help("File path to the pdf"))
        .get_matches();

    let calpath = matches.value_of("calpath").expect("No calendar provided.");

    let buf = BufReader::new(File::open(calpath).unwrap());
    let parser = ical::IcalParser::new(buf);

    for line in parser {
        let events : Vec<ical::parser::ical::component::IcalEvent> = line.unwrap().events;
        for event in events {
            let properties : Vec<ical::property::Property> = event.properties;
            println!{"Event:"}
            for property in properties {
                if property.name == "SUMMARY" {
                    println!("Event Name: {}", property.name);
                } else {
                    println!("{:?}", property.name);
                }
                println!("{:?}", property.value);
            }
            println!();
        }
    }
    //exit(0);

    let ui = AppWindow::new();

    let years = gen_year(&year());
    let months = gen_month(&month());
    let days = gen_days(31);

    let days_model = Rc::new(VecModel::from(days));
    let months_model = months;
    let years_model = years;

    ui.set_days(ModelRc::from(days_model.clone()));
    ui.set_months(months_model);
    ui.set_years(years_model);
    let _appwin_weak = ui.as_weak();
    ui.run();
}
