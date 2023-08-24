use std::collections::HashMap;

use chrono::{Duration, Datelike, Month, NaiveDateTime, DateTime, Utc, TimeZone, NaiveTime, Months};
use icalendar::{parser, Event, Calendar, CalendarComponent, Component, DatePerhapsTime, CalendarDateTime, EventLike, Todo};
use rrule::{RRule, RRuleSet, Tz};

use num_traits::FromPrimitive;

use crate::app::CalendarDisplayType;

fn month() -> String { Month::from_u32(chrono::Utc::now().month()).unwrap().name().to_owned() }
fn year() -> String { chrono::Utc::now().year().to_string() }

// TODO:
// Fix Time Zone inconsistencies with `chrono-tz`
//
// Use Local Time Zones For:
// 1. Filtering Calendar events for today, tomorrow, ...
// 2. Maintaining compatibility with the user's .ics file
// 3. Setting reminders for dates
//
// Use UTC For:
// 1. Everything else (if there even is anything left)

//#[derive(Default, Debug, Clone)]
//pub struct VEvent {
    //pub properties: HashMap<String, String>
//}

/*
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
*/

pub fn read_calendar(conts: &str) -> Calendar {
    parser::read_calendar(&parser::unfold(conts))
        .expect("Could not read Calendar").into()
}

// Display the entire calendar
pub fn show_calendar(cal: &Calendar) {
    info!("Displaying Calendar: \n");
    println!("{}", cal);
}

// Show all calendar events
macro_rules! show {
    ($event:expr, $titles_only:expr) => {
        if $titles_only {
            println!("{}", $event.get_summary().unwrap());
        } else {
            println!("{}", $event.to_string());
        }
    };
}

pub fn show_event(component: &CalendarComponent, titles_only: bool) {
    match (component.as_event(), component.as_todo()) {
        (Some(event), None) => show!(event.to_owned(), titles_only),
        (None, Some(todo))  => show!(todo.to_owned(), titles_only),
        (_, _)              => println!("No events to show"),
    }
}
pub fn show_calendar_events(cal: &Calendar, titles_only: bool) {
    info!("Displaying Calendar Events: \n");
    cal.iter().for_each(|event| show_event(event, titles_only));
}

//
// Time
//

// Filter events before a given date
pub type Time = DateTime<Utc>;
//pub type VEventLike = impl EventLike;

pub fn parse_maybe_date(maybe_dt: DatePerhapsTime) -> Option<Time> {
    match maybe_dt {
        DatePerhapsTime::DateTime(dt) => {
            Some(match dt {
                CalendarDateTime::Utc(dt) => dt,
                CalendarDateTime::Floating(dt) => Utc.from_utc_datetime(&dt),
                #[allow(unused_variables)]
                CalendarDateTime::WithTimezone { date_time, tzid } => Utc.from_local_datetime(&date_time).single().unwrap(),
            })
        },
        DatePerhapsTime::Date(date) => {
            let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            Some(DateTime::<Utc>::from_utc(NaiveDateTime::new(date, time), Utc))
        }
    }
}

pub fn get_event_start(event: impl EventLike) -> Option<Time> {
    info!("get_event_start()");
    let maybe_dt = event.get_start()
        .unwrap_or_else(|| panic!("Error: No DTSTART found for event: {}", event.get_summary().unwrap()));
    debug!("{:?}", maybe_dt);
    parse_maybe_date(maybe_dt)
}

pub fn filter_event_by_time(event: impl EventLike, time: Time) -> bool {
    let dt = get_event_start(event);
    dt.map(|datetime| { datetime < time }).unwrap_or(false)
}

// Filter events before a given date
pub fn filter_events(cal: &Calendar, time: DateTime<Utc>) -> Vec<CalendarComponent> {
    info!("filter_events()");
    cal.iter().cloned().filter(|component| {
        let right = component.as_todo()
            .is_some_and(|e| filter_event_by_time(e.to_owned(), time));
        let left = component.as_event()
            .is_some_and(|e| filter_event_by_time(e.to_owned(), time));
        trace!("{}", left || right);
        left || right
    }).collect()
}

pub fn midnight() -> Time {
    let midnight = Utc::now().date_naive().and_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap());
    DateTime::<Utc>::from_utc(midnight, Utc)
}

// Show all the events only for a given time frame
pub fn filter_by(cal: &Calendar, display_type: CalendarDisplayType) -> Vec<CalendarComponent> {
    let timedelta = match display_type {
        CalendarDisplayType::Today      => midnight(),
        CalendarDisplayType::Tomorrow   => midnight() + Duration::days(1),
        CalendarDisplayType::Week       => midnight() + Duration::weeks(1),
        CalendarDisplayType::Month      => midnight() + Months::new(1),
        CalendarDisplayType::Year       => midnight().with_year(midnight().year() + 1).unwrap(),
    };
    filter_events(cal, timedelta)
}

// Reminders:
// We have 3 separate functions that correspond to past, present and future event times
// Namely: has_started, is_starting, and will_start

// Determines if an event has already started
pub fn has_started(event: impl EventLike) -> bool {
    let start = get_event_start(event).unwrap();
    let now = Utc::now();
    (now - start) > Duration::zero()
}

// Determines if an event is starting soon
pub fn is_starting(event: impl EventLike) -> bool {
    let start = get_event_start(event).unwrap();
    let now = Utc::now();
    (now - start) == Duration::zero()
}

// Determines if an event will start soon
pub fn will_start(event: impl EventLike, tdelta: Duration) -> bool {
    let start = get_event_start(event).unwrap();
    let now = Utc::now();
    (start - now) <= tdelta
}

//
// Reoccuring Events
//

pub fn parse_rrule(rrule: &str) -> RRuleSet {
    rrule.parse().unwrap_or_else(|_| panic!("Unable to parse RRULE: {}", rrule))
}

// Retrieve n recurrences from an event's rrule
//pub fn n_reoccurs(rrule: &str, n: u16) -> Vec<Time> {
pub fn n_reoccurs(rrule: RRuleSet, n: u16) -> Vec<Time> {
    let occurences = rrule.all(n);
    //occurences.dates
        //.expect("Could not calculate reccurence");

    //occurences.dates.into_iter().map(|dt| {
        //Utc::from_local_datetime(&self, dt.naive_local())
    //}).collect()
    occurences.dates.into_iter().map(|dt| {
        Utc.from_utc_datetime(&dt.naive_utc())
        //Utc::from_utc_datetime(&self, &dt.naive_utc())
        //dt.naive_utc();
        //Utc::from_local_datetime(&self, local)
        //Utc::from_local_datetime(&self, &dt.naive_local())
    }).collect()
}

pub fn all_events_between(before: Time, after: Time, rrule: &str, n: u16) -> Vec<Time> {
    let before = before.with_timezone(&rrule::Tz::UTC);
    let after = after.with_timezone(&rrule::Tz::UTC);

    let rrule = parse_rrule(rrule);
    let rrule = rrule.after(after).before(before);
    n_reoccurs(rrule, n)
}
