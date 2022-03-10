extern crate icalendar;
use clap::{Arg, App};
use slint::{ModelRc, VecModel, SharedString};
use chrono::{Datelike, Month};
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

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

fn main() {
    let matches = App::new("Timesync")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("calpath").help("File path to the pdf"))
        .get_matches();

    let calpath = matches.value_of("calpath").expect("No calendar provided.");

    let mut file;
    let readable: &mut dyn std::io::Read = {
        file = File::open(calpath).unwrap();
        &mut file
    };

    let mut output = String::new();
    //readable.read_to_string(&mut output)?;
    //readable.read_to_string(&mut output).unwrap();
    readable.read_to_string(&mut output).unwrap();
    //Ok(Some(output));
    //let unfolded = unfold(&sample);
    let unfolded = icalendar::parser::unfold(&output);
    //icalendar::parser::unfold

    //let cal = icalendar::parser::read_calendar(calpath).unwrap();
    let cal = icalendar::parser::read_calendar(&unfolded).unwrap();
    println!("{}", cal);
    //let cal = icalendar::parser::read_calendar_simple(&unfolded).unwrap();
    
    //let properties = &cal[0].properties;
    //println!("{}", properties[""]);
    

    //cal.components();
    //icalendar::parser::components
    //icalendar::Component::properties();
    //let properties = cal.properties;

    //for property in properties {
        //println!("{}", property[""});
        ////println!("{}", property.name);
        ////println!("{}", property.value);
    //}

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
