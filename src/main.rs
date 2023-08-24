use timesync::{
    app::{CLI, Commands, CalendarDisplayType},
    enable_logging,
    calendar::{
        remove_header,
        get_all_events,
        filter_today,
    },
};

use std::fs;

use anyhow::Result;
use clap::Parser;

#[macro_use]
#[allow(unused_imports)]
extern crate tracing;

fn main() -> Result<()> {
    let cli = CLI::parse();

    if cli.verbose {
        enable_logging();
    }

    let calendar = cli.calendar;

    // TODO: Parse calendar url or filetype here
    let calendar = calendar.unwrap();

    let calendar = fs::read_to_string(calendar)?;

    #[allow(clippy::single_match)]
    match cli.command {
        Some(Commands::Show { display_type }) => {
            let mut events = get_all_events(calendar);

            match display_type {
                CalendarDisplayType::Today => {
                    events = filter_today(events);
                },
                CalendarDisplayType::Week => {},
                CalendarDisplayType::Month => {},
            }
            // Remove the first event calendar
            let events = remove_header(events);

            // Show events
            for event in events {
                println!("{}", event.get_property("SUMMARY"));
                println!("{}", event.get_property("DESCRIPTION"));
            }
            // Quit early
            return Ok(());
        }
        _ => {},
    }
    
    Ok(())

    /*
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
    */
}
