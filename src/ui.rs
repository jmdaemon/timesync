use slint::{ModelRc, VecModel, SharedString};

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

/// Setup the User Interface
pub fn build_ui() -> AppWindow {
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
    ui
}
