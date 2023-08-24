use std::collections::HashMap;

use chrono::{Duration, Datelike, Month, NaiveDateTime, DateTime, Utc, TimeZone, NaiveTime};
use icalendar::{parser, Event as VEvent, Calendar, CalendarComponent, Component, DatePerhapsTime, CalendarDateTime, EventLike, Todo};
use rrule::{RRule, DateFilter};
use either::*;

use num_traits::FromPrimitive;

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

pub fn filter_today(events: Vec<Event>) -> Vec<Event> {
    let mut events_today = vec![];
    for event in events {
        if event.is_today() {
            events_today.push(event);
        }
    }
    events_today
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

//pub fn to_events(conts: &str) -> Vec<Event> {
pub fn read_calendar(conts: &str) -> Calendar {
    parser::read_calendar(&parser::unfold(conts))
        .expect("Could not read Calendar").into()
}

//pub fn display_calendar(cal: &Calendar) {
// Display the entire calendar
pub fn show_calendar(cal: &Calendar) {
    println!("Displaying Calendar: \n");
    println!("{}", cal);
}

pub fn show_calendar_events(cal: &Calendar) {
    println!("Displaying Calendar Events: \n");
    cal.iter().for_each(|component| {
        match (component.as_event(), component.as_todo()) {
            (Some(event), None) => println!("{}", event.to_string()),
            (None, Some(todo)) => println!("{}", todo.to_string()),
            (_, _) => {}
        }
    });
}

// Filter events before a given date
//pub fn filter_time(event: impl EventLike) {

pub type Time = DateTime<Utc>;

pub fn get_event_start(event: impl EventLike) -> Option<Time> {
    let maybe_dt = event.get_start()
        .expect(&format!("Error: No DTSTART found for event: {}", event.get_summary().unwrap()));

    let dt = match maybe_dt {
        DatePerhapsTime::DateTime(dt) => {
            dt.try_into_utc()
        },
        DatePerhapsTime::Date(date) => {
            Some(Utc.from_utc_datetime(&NaiveDateTime::new(date, NaiveTime::default())))
        }
    };
    dt
}

pub fn filter_event_by_time(event: impl EventLike, time: Time) -> bool {
    let dt = get_event_start(event);
    dt.map(|datetime| { datetime < time }).unwrap_or(false)
}

pub fn filter_events<'a>(cal: Calendar, time: DateTime<Utc>) -> Vec<CalendarComponent> {
    cal.to_owned().into_iter()
        .filter(|component| {
            //component.as_event()
            //.or(component.as_todo())
            //.map(|event| filter_event_by_time(event, time))

            //let o1 = component.as_event();
            //let o2 = component.as_todo();
            //let o3 = o1.or(o2).map(|event| filter_event_by_time(event, time));
            //o3.is_some()
            //let values = [Left(component.as_event()), Right(component.as_todo())];
            //let values = [Left(component.as_event()), Right(component.as_todo())];

            //let left = Left(component.as_event());
            //let right = Right(component.as_todo());

            //let left_right = [Left(component.as_event()), Right(component.as_todo())];

            //let left  = either::for_both!(left, event => filter_event_by_time(event.into(), time));
            //let right = either::for_both!(right, event => filter_event_by_time(event.into(), time));
            //let in_time_frame = either::for_both!(left_right, event => filter_event_by_time(event.into(), time));
            //let in_time_frame = either::for_both!(left_right, vevent => filter_event_by_time(vevent.into(), time));
            //in_time_frame;
            let right = component.as_todo()
                .is_some_and(|e| filter_event_by_time(e.to_owned(), time));
            let left = component.as_event()
                .is_some_and(|e| filter_event_by_time(e.to_owned(), time));
            left || right
            //values.map_either(|(a, b)| {
            //})
        }).collect()

        //matches!(component.as_todo(), component.as_event());
        //match component.as_todo() {
        //}
        //let event: Option<&'a dyn EventLike> = match (component.as_event(), component.as_todo()) {
        //let event: Option<dyn Into<EventLike>> = match (component.as_event(), component.as_todo()) {
        //let event: Option<dyn EventLike> = match (component.as_event(), component.as_todo()) {

        /*
        let event: Option<CalendarComponent> = match (component.as_event(), component.as_todo()) {
            (Some(event), _) => Some(CalendarComponent::Event(event.to_owned())),
            (_, Some(todo)) => Some(CalendarComponent::Todo(todo.to_owned())),
            _ => None,
        };
        if let Some(event) = event {
            return filter_event_by_time(event, time);
        } else {
            return false;
        }
        */

        //if component.as_event().is_some() || component.as_todo().is_some() {
            //let even = component.unwrap();
            //return filter_event_by_time(event, time);
        //} else {
            //return false;
        //}
        //if let Some(event) = component.as_event() {

        //} else {
            //false
        //}
    //});
}

pub fn to_events(cal: &Calendar) {
    //for component in cal.iter() {
        //if let Some(event) = component.as_event() {
            //let status = event.get_status().unwrap();
            //event.status
        //}
    //}

    //for component in cal.into_iter() {
        //if let Some(event) = component.as_event() {
            //println!("{}", event.to_string());
        //}
    //}
    //let unfolded = parser::unfold(conts);
    //let cal = parser::read_calendar(conts)
        //.expect("Could not read Calendar");

    //parser::read_calendar(input)

    //let cal = parser::read_calendar_simple(&unfolded)
        //.expect("Unable to read Calendar");
    //display_calcomp(cal.clone());

    //// Parse
    //cal.into_iter().for_each(|component| {
    //});
    


    //let events = parse_events(cal);
    //events
}
