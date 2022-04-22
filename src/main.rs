extern crate icalendar;
use clap::{Arg, App};
use icalendar::{parser};
use rrule::{RRule, DateFilter};
use slint::{ModelRc, VecModel, SharedString};
use chrono::{Datelike, Month, NaiveDateTime};
use num_traits::FromPrimitive;
use std::fs::File;
use std::rc::Rc;
use std::process::exit;
use std::collections::HashMap;
use std::io::{self, Read};

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
    match NaiveDateTime::parse_from_str(datetime, "%Y%m%dT%H%M%S") {
        Ok(datetime) => datetime,
        Err(_) => NaiveDateTime::parse_from_str(datetime, "%Y%m%dT%H%M%SZ").expect("Unable to parse date time string")
    }
}

impl Event {
    /// Returns an Event with the parsed event properties available
    /// in a HashMap
    /// For more information about properties see: https://datatracker.ietf.org/doc/html/rfc5545
    ///
    /// # Arguments
    ///
    /// * `properties` - A HashMap of Strings that hold the parsed properties of a Calendar event
    pub fn new(properties: HashMap<String, String>) -> Event {
        Event { properties: properties }
    }

    /// Get the time for the event
    pub fn get_time(&self, key: &str) -> NaiveDateTime {
        parse_datetime(self.properties.get(key).expect(&format!("{} not found.", key)))
    }

    /// Get the start time of the event
    pub fn get_start_time(&self) -> NaiveDateTime {
        self.get_time("DTSTART")
    }

    /// Get the end time of the event
    pub fn get_end_time(&self) -> NaiveDateTime {
        self.get_time("DTEND")
    }

    /// Calculate the difference between DTSTART and DTEND
    pub fn difftime(&self) -> chrono::Duration {
        let start = self.get_start_time().time();
        let end = self.get_end_time().time();
        end - start
    }

    /// Appends DTSTART to the RRULE string
    pub fn format_rrule(&self) -> String{
        let dtstart  = &self.properties.get("DTSTART").unwrap();
        let rrule_str = &self.properties.get("RRULE").unwrap();

        let mut rrule: String = "DTSTART:".to_string();
        rrule.push_str(&dtstart.to_string());
        rrule.push_str("\n");
        rrule.push_str(&rrule_str.to_string());
        rrule.to_string()
    }

    /// Print the dates that the event occurs on
    ///
    /// # Arguments
    ///
    /// * `limit` - A u16 that sets a hard limit in case of infinitely recurring rules.
    pub fn occurs_on(&self, limit: u16) {
        let format_rrule = &self.format_rrule();
        let rrule: RRule = format_rrule.parse().unwrap();
        // Get all recurrences of the rrule
        let recurrences = rrule.all(limit).unwrap();
        println!("{:?}", recurrences);
    }

    //pub fn is_ongoing(&self) -> bool {
        //let now = get_time_now();
        //let ongoing = now > self.get_start_time();
        //return false;
    //}
}

/// Reads a file into a string and returns the result
pub fn read_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(err) => Err(err),
    }
}

/// Convert all the calendar components into a vector of
/// Events with the properties available as an easy to use HashMap
///
/// # Arguments
///
/// * `verbose` - Show verbose output
/// * `parser_components` - The Vec of icalendar::parser::Components from parsing the calendar file
pub fn parse_events(verbose: bool, parser_components: Vec<icalendar::parser::Component>) -> Vec<Event> {
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
    events
}

/// Display all the calendar components found
pub fn display_calcomp(verbose: bool, cal: Vec<icalendar::parser::Component>) {
    if verbose {
        for calcomp in cal {
            println!("Components");
            println!("{:?}\n", calcomp);
        }
    }
}

/// Setup the User Interface
pub fn build_ui() -> AppWindow {
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
    ui
}

/// Gets the current date
pub fn get_time_now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

/// Gets the current midnight for today
pub fn get_midnight() -> chrono::DateTime<chrono::Utc> {
    get_time_now().date().and_hms(23, 59, 59)
}

/// Return all the events for today
pub fn get_events_today() {
}

/// Return all the events for the week
pub fn get_events_week() {
}

/// Get all possible events
pub fn get_all_events() {
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

    // Match only one occurrence of -v
    match matches.occurrences_of("v") {
        0 => verbose = false,
        1 => verbose = true,
        _ => {
            // Display program usage and exit otherwise
            borrow_app.print_help().unwrap();
            println!("");
            exit(1);
        }
    }

    // Parse the calendar into a vector of parser components
    let output = read_file(calpath).expect("Could not read the contents of {:?}");
    let unfolded = parser::unfold(&output);
    let cal = parser::read_calendar_simple(&unfolded).expect("Unable to create Calendar");

    display_calcomp(verbose, cal.clone());
    let events = parse_events(verbose, cal);
    
    if verbose {
        println!("Events Vector:");
        println!("{:?}", events);
    }

    let example_event : &Event = &events[2];
    println!("{:?}", events[2]);
    println!("{:?}", example_event);
    println!("{:?}", example_event.difftime());
    println!("{}", &example_event.properties.get("DTSTART").unwrap());

    println!("{:?}", example_event.occurs_on(100));

    exit(0);

    let ui = build_ui();
    ui.run();
}
