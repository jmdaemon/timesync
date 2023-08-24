use std::collections::HashMap;

use chrono::{Duration, Datelike, Month, NaiveDateTime, DateTime, Utc, TimeZone, NaiveTime, Months};
use icalendar::{parser, Event, Calendar, CalendarComponent, Component, DatePerhapsTime, CalendarDateTime, EventLike, Todo};
use rrule::{RRule, DateFilter};

use num_traits::FromPrimitive;

use crate::app::CalendarDisplayType;

fn month() -> String { Month::from_u32(chrono::Utc::now().month()).unwrap().name().to_owned() }
fn year() -> String { chrono::Utc::now().year().to_string() }

//#[derive(Default, Debug, Clone)]
//pub struct VEvent {
    //pub properties: HashMap<String, String>
//}

/*
impl VEvent {
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
