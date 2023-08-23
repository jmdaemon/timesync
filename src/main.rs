extern crate icalendar;
use log::{debug, error, info, warn};
use clap::{Arg, Command};
use icalendar::{parser};
use rrule::{RRule, DateFilter};
use slint::{ModelRc, VecModel, SharedString};
use chrono::{Duration, Datelike, Month, NaiveDateTime};
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

#[derive(Default, Debug, Clone)]
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

    /// Get a property for the event
    pub fn get_property(&self, key: &str) -> String {
        self.properties.get(key).expect(&format!("{} not found.", key)).to_string()
    }

    /// Gets the DTSTART event property
    pub fn dtstart(&self) -> String {
        self.get_property("DTSTART")
    }

    /// Gets the DTEND event property
    pub fn dtend(&self) -> String {
        self.get_property("DTEND")
    }

    /// Get the start time of the event
    pub fn get_start_time(&self) -> NaiveDateTime {
        parse_datetime(&self.dtstart())
    }

    /// Get the end time of the event
    pub fn get_end_time(&self) -> NaiveDateTime {
        parse_datetime(&self.dtend())
    }

    /// Calculate the difference between DTSTART and DTEND
    pub fn difftime(&self) -> chrono::Duration {
        let start = self.get_start_time().time();
        let end = self.get_end_time().time();
        end - start
    }

    /// Appends DTSTART to the RRULE string
    pub fn format_rrule(&self) -> String{
        let dtstart  = self.dtstart();
        let rrule_str = self.get_property("RRULE");

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

    /// Determine if the event is currently ongoing
    pub fn is_ongoing(&self) -> bool {
        let now = get_time_now().timestamp();
        let ongoing = (now > self.get_start_time().timestamp()) && !(now > self.get_end_time().timestamp());
        ongoing
    }

    /// Determine if the event occurs today
    pub fn is_today(&self) -> bool {
        let date_today = get_time_now().date().naive_utc();
        let today = date_today == self.get_start_time().date();
        today
    }

    /// Determine if the event occurs tomorrow
    pub fn is_tomorrow(&self) -> bool {
        let date_tomorrow = get_time_now() + Duration::days(1);
        let tomorrow = self.get_start_time().date() == date_tomorrow.date().naive_utc();
        tomorrow
    }

    /// Determine if the event occurs this week
    pub fn is_this_week(&self) -> bool {
        let date_next_week = get_time_now() + Duration::days(7);
        let week = self.get_start_time().date() < date_next_week.date().naive_utc();
        week
    }

    /// Determines if the event has already started
    pub fn has_started(&self) -> bool {
        let now = get_time_now().timestamp();
        let event_start_time = self.get_start_time().timestamp();
        let has_started = now > event_start_time;
        has_started
    }

    /// Determines if an event will start soon
    pub fn will_start_in(&self, duration: Duration) -> bool {
        let now = get_time_now();
        let future = (now + duration).timestamp();
        let start = self.get_start_time().timestamp();
        // If the event will start x minutes/hours/days into the future
        // return true, else false otherwise
        // Note that an absolute difference function might be better to use here
        let starts_soon = start == future; 
        starts_soon
    }
    
    /**
    * Determines if the event is urgent
    * Note that the event is determined to be urgent if:
    * - The specific event will occur within the next duration of minutes/hours/days
    * - The specific event has not already passed.
    */
    pub fn is_urgent(&self, duration: Duration) -> bool {
        let urgent = self.will_start_in(duration) && !(self.has_started());
        urgent
    }

    /// Determine if the event takes place during the course of the entire day
    pub fn is_allday(&self) -> bool {
        let start = self.get_start_time().timestamp();
        let end = self.get_end_time().timestamp();
        let duration = end - start;
        // An all day event is one whose start and end times are 00:00:00, and whose duration is divisible by 24
        let allday = (self.get_start_time().time() == chrono::NaiveTime::from_hms(0,0,0)) && ((duration % 24) == 0);
        allday
    }
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
pub fn parse_events(parser_components: Vec<icalendar::parser::Component>) -> Vec<Event> {
    let mut events = Vec::new();
    for comp in parser_components {
        let acomponents = comp.components;
        for acomp in acomponents {
            debug!("{:?}", acomp);               // Display component
            debug!("{:?}", acomp.properties);    // Display all properties at once

            let properties = acomp.properties;
            let mut event_properties = HashMap::new();

            debug!("Component Properties Found:");
            for prop in properties {
                debug!("{:?}", prop.name);
                debug!("{:?}", prop.val);
                event_properties.insert(prop.name.to_string(), prop.val.to_string());
            }
            let event = Event::new(event_properties);
            events.push(event);
        }
    }
    events
}

/// Display all the calendar components found
pub fn display_calcomp(cal: Vec<icalendar::parser::Component>) {
    for calcomp in cal {
        debug!("Components");
        debug!("{:?}\n", calcomp);
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
pub fn get_all_events(output: String) -> Vec<Event> {
    let unfolded = parser::unfold(&output);
    let cal = parser::read_calendar_simple(&unfolded).expect("Unable to create Calendar");
    display_calcomp(cal.clone());
    let events = parse_events(cal);
    events
}

pub fn filter_today(events: Vec<Event>) -> Vec<Event> {
    let mut events_today = vec![];
    for event in events {
        if event.is_today() {
            events_today.push(event);
        }
    }
    events_today
}

/// Build the command line interface
pub fn build_cli() -> clap::Command<'static> {
    let cli = Command::new("Timesync")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Lightweight, fast, and highly customizable calendar application")
        .arg(Arg::new("calpath")
            .required(true)
            .help("File path to the .ics calendar file"))
        .subcommand(
            Command::new("show")
            .arg(Arg::new("today")
                .short('t')
                .required(false)
                .help("Show the calendar events for today"))
            .about("Checks the calendar for events"));
    cli
}

/// Removes the first "event" which is not a real event but 
/// information about the calendar itself
pub fn remove_header(hevents: Vec<Event>) -> Vec<Event> {
    //let mut events: Vec<&Event> = vec![];
    let mut events = vec![];

    // Remove the first 'event' which is just the header
    for i in 1..hevents.len() {
        let event = hevents[i].clone();
        events.push(event);
    }
    events
}

// Get the first day of the current month
// Get the week day of the first day
// Calculate the offset of the week day
// Get the total number of days of the previous month
// 

fn main() {
    pretty_env_logger::init();
    let cli = build_cli();

    let matches = cli.get_matches();
    let calpath = matches.value_of("calpath").expect("No calendar provided.");
    
    let subcmds = matches.subcommand();
    match subcmds {
        Some(("show", subcmds)) => {
            // Display today's events or this weeks events
            let display_today = subcmds.is_present("today");

            // Get all events
            let output = read_file(calpath).expect("Could not read the contents of {:?}");
            let all_events = get_all_events(output);

            // Filter the events
            let mut hevents: Vec<Event>;
            match display_today {
                true => { hevents = filter_today(all_events); },
                false => { hevents = all_events}
            }

            // Remove the initial calendar blotter
            let events = remove_header(hevents);

            // Display the events
            for event in events {
                println!("{}", event.get_property("SUMMARY"));
                println!("{}", event.get_property("DESCRIPTION"));
            }

            exit(0);
        } 
        _ => {}
    };

    // Parse the calendar into a vector of parser components
    let output = read_file(calpath).expect("Could not read the contents of {:?}");
    let unfolded = parser::unfold(&output);
    let cal = parser::read_calendar_simple(&unfolded).expect("Unable to create Calendar");

    display_calcomp(cal.clone());
    let events = parse_events(cal);
    
    info!("Events Vector:");
    info!("{:?}", events);

    let example_event : &Event = &events[2];
    info!("Example Event");
    info!("{:?}", events[2]);
    info!("{:?}", example_event);
    info!("{:?}", example_event.difftime());
    info!("{}", &example_event.properties.get("DTSTART").unwrap());
    info!("Event Occurs On:");
    info!("{:?}", example_event.occurs_on(100));

    //exit(0);

    let ui = build_ui();
    ui.run();
}
