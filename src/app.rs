use std::path::PathBuf;

use crate::consts::PROGRAM_DESCRIPTION;

use clap::{Parser, arg, Subcommand, ValueEnum};

// Manage the command-line-interface for the app

#[derive(Parser)]
#[command(author, version, about, long_about = PROGRAM_DESCRIPTION)]
pub struct CLI {
    #[arg(short, long, default_value_t = false, global = true, help = "Toggle verbose information")]
    pub verbose: bool,

    #[arg(short, long, global = true, help = "File path or URL to a valid .ics calendar")]
    pub calendar: Option<PathBuf>,

    #[arg(short, long, default_value_t = false, global = true, help = "Display event title names only")]
    pub title: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Show a calendar's events")]
    Show {
        #[arg(value_enum)]
        display_type: Option<CalendarDisplayType>
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CalendarDisplayType {
    Today,
    Tomorrow,
    Week,
    Month,
    Year,
}
