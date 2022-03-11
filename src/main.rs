extern crate icalendar;
use clap::{Arg, App};
use slint::{ModelRc, VecModel, SharedString};
use chrono::{Datelike, Month, NaiveDateTime};
use num_traits::FromPrimitive;
use std::fs::File;
use std::rc::Rc;
//use std::time;
use std::process::exit;
use std::collections::HashMap;

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

//#[derive(Debug)]
#[derive(Default, Debug)]
pub struct Event {
    pub properties: HashMap<String, String>
}

pub fn parse_datetime(datetime: &str) -> chrono::NaiveDateTime {
    NaiveDateTime::parse_from_str(datetime, "%Y%m%dT%H%M%SZ").expect("Unable to parse date time string")
}

impl Event {
    pub fn new(properties: HashMap<String, String>) -> Event {
        Event { properties: properties }
    }

    pub fn difftime(&self) -> chrono::Duration {
        let start = parse_datetime(&self.properties.get("DTSTART").expect("DTSTART not found.")).time();
        let end = parse_datetime(&self.properties.get("DTEND").expect("DTEND not found")).time();
        end - start
    }
}

fn main() {
    let app = App::new("Timesync")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("calpath").help("File path to the pdf"))
        .arg(Arg::new("v").help("Show verbose output"));

    let mut borrow_app = app.clone();
    let matches = app.get_matches();
    let calpath = matches.value_of("calpath").expect("No calendar provided.");
    let verbose;
    match matches.occurrences_of("v") {
        0 => verbose = false,
        1 => verbose = true,
        _ => {
            borrow_app.print_help().unwrap();
            println!("");
            std::process::exit(1);
        }
    }

    // Read the file
    let mut file;
    let readable: &mut dyn std::io::Read = {
        file = File::open(calpath).unwrap();
        &mut file
    };

    // Parse the file and create the calendar
    let mut output = String::new();
    readable.read_to_string(&mut output).unwrap();
    let unfolded = parser::unfold(&output);
    let cal = parser::read_calendar_simple(&unfolded).unwrap();

    let mut parser_components = Vec::new();
    for calcomp in cal {
        // Display all the parser_components found
        if verbose {
            println!("Components");
            println!("{:?}\n", calcomp);
        }
        parser_components.push(calcomp);
    }

    // Convert all the calendar components into a vector of
    // Events with the properties available as an easy to use HashMap
    let mut events = Vec::new();
    for comp in parser_components {
        let acomponents = comp.components;
        for acomp in acomponents {
            if verbose {
                // Display component
                println!("{:?}", acomp);

                // Display all properties at once
                println!("{:?}", acomp.properties);
            }

            let properties = acomp.properties;
            let mut event_properties = HashMap::new();

            if verbose {
                println!("Component Properties Found:");
                for prop in properties {
                        println!("{:?}", prop.name);
                        println!("{:?}", prop.val);
                        event_properties.insert(prop.name.to_string(), prop.val.to_string());
                    }
                }
            else {
                for prop in properties {
                    event_properties.insert(prop.name.to_string(), prop.val.to_string());
                }
            }
            let event = Event::new(event_properties);
            events.push(event);
        }
    }
    
    if verbose {
        println!("Events Vector:");
        println!("{:?}", events);
    }

    let example_event : &Event = &events[1];
    println!("{:?}", events[1]);
    println!("{:?}", example_event);
    println!("{:?}", example_event.difftime());
    println!("{}", &example_event.properties.get("DTSTART").unwrap());

    exit(0);

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
