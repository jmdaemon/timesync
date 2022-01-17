use sixtyfps::{Model, ModelHandle, VecModel, SharedString};
use chrono::{Datelike, Utc, Month};
use num_traits::FromPrimitive;
use std::rc::Rc;

sixtyfps::include_modules!();

/// Generate days of the month
fn gen_days(month: &str, ui: &AppWindow) -> Vec<DayData> {
    let mut days: Vec<DayData> = ui.get_days().iter().collect();
    for i in 1..31 {
        days.push(DayData {
            daynum: SharedString::from(i.to_string()),
        });

    }
    return days;
}

fn gen_month(month: &str, ui: &AppWindow) -> Vec<MonthData> {
    let mut months: Vec<MonthData> = ui.get_months().iter().collect();
    months.push(MonthData {
        current_month: SharedString::from(month.to_owned())
    });
    return months;
}


/// Get the current month
fn month() -> String {
    let dt = chrono::Utc::now();
    //let month_num = dt.month();
    //let month_str = dt.format("%m").to_string();
    let month_str = Month::from_u32(dt.month()).unwrap().name().to_owned();
    return month_str;
}

fn year() -> i32 {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    return year;
}

fn main() {
    let ui = AppWindow::new();

    //let mut days: Vec<DayData> = ui.get_days().iter().collect();
    //let months = [
        //"January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November"
    //];
    //let month = month();
    let months = gen_month(&month(), &ui);
    //let days = gen_days(&month[0], &ui);
    let days = gen_days(&month(), &ui);

    let days_model = Rc::new(VecModel::from(days));
    let months_model  = Rc::new(VecModel::from(months));

    ui.set_days(ModelHandle::new(days_model.clone()));
    ui.set_months(ModelHandle::new(months_model.clone()));
    let _appwin_weak = ui.as_weak();
    ui.run();
}
