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

fn gen_month(month: &str, ui: &AppWindow) -> MonthData {
    let mut months: MonthData = ui.get_months();
    months = MonthData {
        current_month: SharedString::from(month.to_owned())
    };
    return months;
}


/// Get the current month
fn month() -> String {
    let dt = chrono::Utc::now();
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

    let months = gen_month(&month(), &ui);
    let days = gen_days(&month(), &ui);

    let days_model = Rc::new(VecModel::from(days));
    let months_model = months;

    ui.set_days(ModelHandle::new(days_model.clone()));
    ui.set_months(months_model);
    let _appwin_weak = ui.as_weak();
    ui.run();
}
