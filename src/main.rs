extern crate icalendar;
use clap::{Arg, App};
use slint::{ModelRc, VecModel, SharedString};
use chrono::{Datelike, Month};
use num_traits::FromPrimitive;
use std::fs::File;
//use std::io::Read;
use std::rc::Rc;
use std::process::exit;

use icalendar::{parser};

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

    let mut file;
    let readable: &mut dyn std::io::Read = {
        file = File::open(calpath).unwrap();
        &mut file
    };

    let mut output = String::new();
    readable.read_to_string(&mut output).unwrap();
    //let unfolded = icalendar::parser::unfold(&output);
    let unfolded = parser::unfold(&output);
    let cal = parser::read_calendar_simple(&unfolded).unwrap();

    let mut parser_components = Vec::new();
    for calcomp in cal {
        // Display all the parser_components found
        //println!("{:?}\n", calcomp);
        parser_components.push(calcomp);
    }
    for comp in parser_components {
        // Accessing Properties
        //let properties = comp.properties;
        ////println!("{:?}\n", comp.properties);
        //for prop in properties {
            //println!("{:?}", prop);
        //}
        let acomponents = comp.components;
        for acomp in acomponents {
            // Display component
            println!("{:?}", acomp);

            // Display all properties at once
            let properties = acomp.properties;
            println!("{:?}", properties);

            // Access properties individually
            for prop in properties {
                println!("{:?}", prop.name);
                println!("{:?}", prop.val);
            }
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
