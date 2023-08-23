use timesync::{
    enable_logging,
    calendar::{
        Event,
        display_calcomp,
        parse_events,
        remove_header,
        get_all_events,
        filter_today,
    },
};

use std::fs::File;
use std::rc::Rc;
use std::process::exit;
use std::io::{self, Read};

use clap::{Arg, Command};
use icalendar::parser;

#[macro_use]
#[allow(unused_imports)]
extern crate tracing;

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

fn main() {
    enable_logging();

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

    //let ui = build_ui();
    //ui.run();
}
