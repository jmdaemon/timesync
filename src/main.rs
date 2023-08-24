use timesync::{
    app::{CLI, Commands},
    calendar::{read_calendar, show_calendar, show_calendar_events, filter_by},
    enable_logging,
};

use std::{fs, process::exit};

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
    let titles_only = cli.title;

    info!("Global Arguments: ");
    info!("Calendar         : {:?}", calendar);
    info!("Show Titles Only : {}", titles_only);

    let calendar = fs::read_to_string(calendar)?;

    #[allow(clippy::single_match)]
    match cli.command {
        Some(Commands::Show { display_type }) => {
            info!("Commands::Show");
            let cal = read_calendar(&calendar);
            
            // Show the entire calendar by default
            if display_type.is_none() {
                show_calendar(&cal);
                return Ok(());
            }

            // Show the specified events only
            let display_type = display_type.unwrap();
            let components = filter_by(&cal, display_type);
            show_calendar_events(components, titles_only);

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
